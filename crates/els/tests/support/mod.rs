//! The ELS compile-and-evaluate fixture harness. Every protocol test calls it; a story that needs
//! it changed records that as a scope change on itself first.
//!
//! The harness adds no evaluator of its own and reads no clock, network or model. It validates and
//! compiles a `protocol/1` document with Canon (`b10x_canon::validate::validate` and
//! `b10x_canon::ir::compile`, the checks `canon validate` and `canon compile` run), evaluates a
//! fixture's states with `b10x_canon::eval::evaluate`, and returns Canon's `canon-decision/1`
//! decision, or its refusal, unchanged.
//!
//! # Where protocols and fixtures live
//!
//! - A built-in ELS protocol is a `protocol/1` document at `protocols/<name>/<major>.yaml`
//!   ([`protocol_path`]). Each protocol story creates its own file.
//! - A protocol's fixtures live under `fixtures/<protocol-name>/`, one `<fixture-id>.fixture.yaml`
//!   file per fixture. Each protocol story creates its own directory. A protocol that exists only
//!   to test the harness lives beside its fixtures (`fixtures/smoke/protocol.yaml`).
//!
//! Every path a fixture names is relative to the repository root, without a leading `/` or `\`,
//! a drive prefix or a `..` component, and must not resolve outside the repository through a
//! symbolic link.
//!
//! # The `els-fixture/1` format
//!
//! ```yaml
//! format: els-fixture/1
//! id: smoke                              # the fixture's name; the file is <id>.fixture.yaml
//! protocol: fixtures/smoke/protocol.yaml # the protocol/1 document the fixture evaluates
//! at: "2026-10-04T00:00:00Z"             # the evaluation instant, YYYY-MM-DDTHH:MM:SSZ (UTC)
//! case:                                  # a canon-case/1 document: the case and the current
//!   format: canon-case/1                 # revision of every artifact the protocol declares
//!   id: SMOKE-1
//!   protocol: smoke
//!   artifacts:
//!     service: {revision: r1}
//! states:                                # an ordered, non-empty list
//!   - id: initial                        # unique within the fixture
//!     add_evidence: []                   # evidence added to that of the states before it
//!     expect:
//!       claims:                          # every claim the protocol declares, and its value
//!         service.healthy: unknown       # true, false or unknown
//!   - id: healthy-observed
//!     add_evidence:
//!       - observed_at: "2026-10-03T23:00:00Z"   # when it was observed, not after `at`
//!         record: {format: canon-evidence/1, id: observation-1, kind: operational_observation,
//!                  result: healthy, subject: service, subject_revision: r1}
//!     expect:
//!       claims:
//!         service.healthy: true
//! ```
//!
//! States are cumulative: a state is evaluated over its own `add_evidence` and that of every
//! state before it. A fixture cannot carry an input the harness would drop or read as something
//! else: an unknown key is refused, a key written with no value (`add_evidence:`) is refused as
//! Canon refuses one rather than read as its default, and a claim written twice in
//! `expect.claims` is refused, naming the claim. [`Fixture::load`] refuses a file not named
//! `<id>.fixture.yaml` after the id it declares, so two files cannot declare one fixture.
//!
//! The case and every evidence record are read with Canon's own readers
//! (`b10x_canon::eval::case_from_value` and `evidence_from_value`, the path `canon evaluate`
//! takes), so a document Canon refuses does not load: an identifier, revision or result written
//! as a number or a boolean is refused with Canon's `malformed-input` refusal.
//!
//! The evaluation instant is an input of the fixture, never read from a clock. Canon's evaluator
//! does not take an instant or an observation time yet, so the harness cannot pass either to it.
//! It refuses, when loading, a fixture holding evidence observed after its instant, instead of
//! evaluating evidence the instant excludes. Instants are written in the one form above and name
//! a day that exists in its month (leap years counted), so they compare as text.
//!
//! Authority and independence decisions are not part of the format yet: Canon does not evaluate
//! them, and a fixture that gives them (`authority:`, `independence:`) is refused as having an
//! unknown key. They join the format with the Canon capability that reads them.
//!
//! The items only later protocol stories call carry `#[allow(dead_code)]`, each marked as
//! later-story API; everything else is used by this story's tests.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};

use b10x_canon::eval::{self, Refusal};
use b10x_canon::ir::Ir;
use b10x_canon::model::{Case, Decision, EvidenceRecord, ParseError, Truth};
use b10x_canon::validate::Problem;
use serde::de::{Error as _, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_yaml_ng::Value;

/// The fixture format this harness reads.
pub const FORMAT: &str = "els-fixture/1";

/// The repository root, found at run time from the package's manifest directory.
pub fn repo_root() -> PathBuf {
    let manifest = std::env::var_os("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR");
    let root = Path::new(&manifest).join("../..");
    root.canonicalize()
        .unwrap_or_else(|error| panic!("the repository root {}: {error}", root.display()))
}

/// The path of built-in protocol `name` at major version `major`, relative to the repository root.
#[allow(dead_code)] // later-story API: the protocol stories compile protocols/<name>/<major>.yaml
pub fn protocol_path(name: &str, major: u64) -> String {
    format!("protocols/{name}/{major}.yaml")
}

/// Why a fixture did not load.
#[derive(Debug)]
pub enum FixtureError {
    /// The fixture path is absolute, leaves the repository, or cannot be read.
    Unreadable(String),
    /// The text is not an `els-fixture/1` document.
    Malformed(String),
    /// The document is well-formed but cannot be evaluated as written.
    Refused(String),
}

impl fmt::Display for FixtureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FixtureError::Unreadable(why) | FixtureError::Refused(why) => f.write_str(why),
            FixtureError::Malformed(why) => write!(f, "not an {FORMAT} document: {why}"),
        }
    }
}

impl std::error::Error for FixtureError {}

/// Why a protocol did not compile.
#[derive(Debug)]
pub enum CompileError {
    /// The protocol path is absolute, leaves the repository, or cannot be read.
    Unreadable(String),
    /// The text is not a `protocol/1` document.
    Parse(ParseError),
    /// Canon's validator rejects the document; its problems, unchanged.
    Invalid(Vec<Problem>),
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompileError::Unreadable(why) => f.write_str(why),
            CompileError::Parse(error) => write!(f, "{error}"),
            CompileError::Invalid(problems) => {
                let lines: Vec<String> = problems.iter().map(ToString::to_string).collect();
                write!(f, "the protocol is invalid: {}", lines.join("; "))
            }
        }
    }
}

impl std::error::Error for CompileError {}

/// A protocol Canon validated and compiled.
#[derive(Debug, Clone)]
pub struct Compiled {
    ir: Ir,
    canon_ir: String,
}

impl Compiled {
    /// The compiled protocol, as Canon's evaluator takes it.
    pub fn ir(&self) -> &Ir {
        &self.ir
    }

    /// The `canon-ir/1` document, the bytes `canon compile` prints.
    pub fn canon_ir(&self) -> &str {
        &self.canon_ir
    }
}

/// Validates and compiles the `protocol/1` document at `path`, relative to the repository root.
pub fn compile_protocol(path: &str) -> Result<Compiled, CompileError> {
    let (_, text) = read_confined(path, "protocol path").map_err(CompileError::Unreadable)?;
    let protocol = b10x_canon::model::parse(&text).map_err(CompileError::Parse)?;
    b10x_canon::validate::validate(&protocol).map_err(CompileError::Invalid)?;
    let ir = b10x_canon::ir::compile(&protocol).map_err(CompileError::Invalid)?;
    let canon_ir = ir.canonical_json();
    Ok(Compiled { ir, canon_ir })
}

/// One loaded `els-fixture/1` document.
#[derive(Debug, Clone)]
pub struct Fixture {
    id: String,
    protocol: String,
    at: String,
    case: Case,
    states: Vec<State>,
}

/// One state as loaded: the evidence it adds, read by Canon, and its expectation.
#[derive(Debug, Clone)]
struct State {
    id: String,
    add_evidence: Vec<EvidenceRecord>,
    expect: Expectation,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Expectation {
    #[serde(deserialize_with = "claims_once")]
    claims: BTreeMap<String, Truth>,
}

/// The document as written. The case and the evidence records stay YAML values until Canon's
/// readers read them. Every key is required and must have a value.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Written {
    #[serde(deserialize_with = "required")]
    format: String,
    #[serde(deserialize_with = "required")]
    id: String,
    #[serde(deserialize_with = "required")]
    protocol: String,
    #[serde(deserialize_with = "required")]
    at: String,
    #[serde(deserialize_with = "required")]
    case: Value,
    #[serde(deserialize_with = "required")]
    states: Vec<WrittenState>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WrittenState {
    #[serde(deserialize_with = "required")]
    id: String,
    #[serde(deserialize_with = "required")]
    add_evidence: Vec<WrittenObservation>,
    #[serde(deserialize_with = "required")]
    expect: Expectation,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WrittenObservation {
    #[serde(deserialize_with = "required")]
    observed_at: String,
    #[serde(deserialize_with = "required")]
    record: Value,
}

/// Reads a key's value, refusing a key written with no value (`add_evidence:`) instead of reading
/// it as an empty list, an empty text or `~`, as Canon refuses one.
fn required<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<T, D::Error> {
    Option::<T>::deserialize(deserializer)?
        .ok_or_else(|| D::Error::custom("a key is written with no value"))
}

/// Reads `expect.claims`, refusing a claim written more than once instead of keeping the last.
fn claims_once<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<BTreeMap<String, Truth>, D::Error> {
    struct Claims;

    impl<'de> Visitor<'de> for Claims {
        type Value = BTreeMap<String, Truth>;

        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("a map of claim ids to true, false or unknown")
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut claims = BTreeMap::new();
            while let Some((claim, value)) = map.next_entry::<String, Truth>()? {
                if claims.contains_key(&claim) {
                    return Err(A::Error::custom(format!(
                        "claim `{claim}` is expected more than once"
                    )));
                }
                claims.insert(claim, value);
            }
            Ok(claims)
        }
    }

    deserializer.deserialize_map(Claims)
}

impl Fixture {
    /// Loads the fixture at `path`, relative to the repository root. The file must be named
    /// `<id>.fixture.yaml` after the id the fixture declares; a symbolic link is judged by the
    /// name of the file it resolves to, which is the fixture it loads.
    pub fn load(path: &str) -> Result<Self, FixtureError> {
        let (resolved, text) =
            read_confined(path, "fixture path").map_err(FixtureError::Unreadable)?;
        let fixture = Self::from_yaml(&text)?;
        let file = resolved
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();
        let named = format!("{}.fixture.yaml", fixture.id);
        if file != named {
            return Err(FixtureError::Refused(format!(
                "fixture file `{file}` declares id `{}`; it must be named `{named}`",
                fixture.id
            )));
        }
        Ok(fixture)
    }

    /// Reads an `els-fixture/1` document from its text.
    pub fn from_yaml(text: &str) -> Result<Self, FixtureError> {
        let written: Written = serde_yaml_ng::from_str(text)
            .map_err(|error| FixtureError::Malformed(error.to_string()))?;
        let refuse = |why: String| Err(FixtureError::Refused(why));
        if written.format != FORMAT {
            return refuse(format!(
                "format is `{}`, expected `{FORMAT}`",
                written.format
            ));
        }
        if written.protocol.is_empty() || !b10x_canon::conform::is_confined(&written.protocol) {
            return refuse(format!(
                "protocol path `{}` is absolute or leaves the repository",
                written.protocol
            ));
        }
        if !is_instant(&written.at) {
            return refuse(format!(
                "evaluation instant `{}` is not written YYYY-MM-DDTHH:MM:SSZ",
                written.at
            ));
        }
        let case = eval::case_from_value(&written.case).map_err(|refusal| {
            FixtureError::Refused(format!("case: {}: {refusal}", refusal.code()))
        })?;
        if written.states.is_empty() {
            return refuse("the fixture lists no states".to_owned());
        }
        let mut seen = BTreeSet::new();
        let mut states = Vec::with_capacity(written.states.len());
        for state in written.states {
            if !seen.insert(state.id.clone()) {
                return refuse(format!("state `{}` is given more than once", state.id));
            }
            let mut add_evidence = Vec::with_capacity(state.add_evidence.len());
            for (index, observation) in state.add_evidence.iter().enumerate() {
                let record = eval::evidence_from_value(&observation.record).map_err(|refusal| {
                    FixtureError::Refused(format!(
                        "state `{}`: evidence {}: {}: {refusal}",
                        state.id,
                        index + 1,
                        refusal.code()
                    ))
                })?;
                let evidence = record.id.as_str();
                if !is_instant(&observation.observed_at) {
                    return refuse(format!(
                        "state `{}`: evidence `{evidence}` observation instant `{}` is not \
                         written YYYY-MM-DDTHH:MM:SSZ",
                        state.id, observation.observed_at
                    ));
                }
                if observation.observed_at > written.at {
                    return refuse(format!(
                        "state `{}`: evidence `{evidence}` is observed at `{}`, after the \
                         evaluation instant `{}`",
                        state.id, observation.observed_at, written.at
                    ));
                }
                add_evidence.push(record);
            }
            states.push(State {
                id: state.id,
                add_evidence,
                expect: state.expect,
            });
        }
        Ok(Fixture {
            id: written.id,
            protocol: written.protocol,
            at: written.at,
            case,
            states,
        })
    }

    /// The fixture's name.
    #[allow(dead_code)] // later-story API: protocol stories name the fixture in their messages
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The protocol the fixture evaluates, relative to the repository root.
    pub fn protocol_path(&self) -> &str {
        &self.protocol
    }

    /// The evaluation instant.
    pub fn at(&self) -> &str {
        &self.at
    }

    /// The case snapshot every state is evaluated for.
    #[allow(dead_code)] // later-story API: protocol stories read the case they evaluate
    pub fn case(&self) -> &Case {
        &self.case
    }

    /// The state ids, in order.
    #[allow(dead_code)] // later-story API: protocol stories walk a fixture state by state
    pub fn states(&self) -> Vec<&str> {
        self.states.iter().map(|state| state.id.as_str()).collect()
    }

    /// Validates and compiles the fixture's protocol.
    pub fn compile(&self) -> Result<Compiled, CompileError> {
        compile_protocol(&self.protocol)
    }

    /// The evidence state `state` is evaluated over: its own and that of every state before it.
    /// Panics when the fixture has no such state.
    pub fn evidence(&self, state: &str) -> Vec<EvidenceRecord> {
        let end = self
            .states
            .iter()
            .position(|candidate| candidate.id == state)
            .unwrap_or_else(|| panic!("fixture `{}` has no state `{state}`", self.id));
        self.states[..=end]
            .iter()
            .flat_map(|state| state.add_evidence.iter().cloned())
            .collect()
    }

    /// Canon's evaluation of state `state`, unchanged. Panics when the fixture has no such state.
    pub fn evaluate(&self, compiled: &Compiled, state: &str) -> Result<Decision, Refusal> {
        eval::evaluate(compiled.ir(), &self.case, &self.evidence(state))
    }

    /// Evaluates every state and compares each decision with the state's expectation: every
    /// claim the decision holds must be expected with its value, and every expected claim must
    /// be in the decision. Returns one line per difference, in state order, then claim order.
    pub fn check(&self, compiled: &Compiled) -> Result<(), Vec<String>> {
        let mut differences = Vec::new();
        for state in &self.states {
            let decision = match self.evaluate(compiled, &state.id) {
                Ok(decision) => decision,
                Err(refusal) => {
                    differences.push(format!(
                        "state `{}`: evaluation refused: {}: {refusal}",
                        state.id,
                        refusal.code()
                    ));
                    continue;
                }
            };
            let found: BTreeMap<String, Truth> = decision
                .claims
                .iter()
                .map(|(id, entry)| (id.as_str().to_owned(), entry.value))
                .collect();
            let expected = &state.expect.claims;
            let claims: BTreeSet<&String> = found.keys().chain(expected.keys()).collect();
            for claim in claims {
                let line = match (found.get(claim), expected.get(claim)) {
                    (Some(found), Some(expected)) if found == expected => continue,
                    (Some(found), Some(expected)) => format!("is {found}, expected {expected}"),
                    (Some(found), None) => format!("is {found}, expected nothing"),
                    (None, Some(expected)) => format!("is not declared, expected {expected}"),
                    (None, None) => unreachable!("the claim comes from one of the two"),
                };
                differences.push(format!("state `{}`: claim `{claim}` {line}", state.id));
            }
        }
        if differences.is_empty() {
            Ok(())
        } else {
            Err(differences)
        }
    }
}

/// Reads `path`, relative to the repository root, and returns the file it resolves to and its
/// text, refusing a path that is absolute, has a `..` component or resolves outside the
/// repository through a symbolic link.
fn read_confined(path: &str, what: &str) -> Result<(PathBuf, String), String> {
    if path.is_empty() || !b10x_canon::conform::is_confined(path) {
        return Err(format!(
            "{what} `{path}` is absolute or leaves the repository"
        ));
    }
    let root = repo_root();
    let resolved = root
        .join(path)
        .canonicalize()
        .map_err(|error| format!("{what} `{path}`: {error}"))?;
    if !resolved.starts_with(&root) {
        return Err(format!("{what} `{path}` resolves outside the repository"));
    }
    let text =
        std::fs::read_to_string(&resolved).map_err(|error| format!("{what} `{path}`: {error}"))?;
    Ok((resolved, text))
}

/// Whether `text` is an instant written `YYYY-MM-DDTHH:MM:SSZ` naming a real day (leap years
/// counted) and time, the one form in which instants compare correctly as text.
fn is_instant(text: &str) -> bool {
    let bytes = text.as_bytes();
    if bytes.len() != 20 {
        return false;
    }
    let shape = b"dddd-dd-ddTdd:dd:ddZ";
    let shaped = bytes
        .iter()
        .zip(shape)
        .all(|(byte, expected)| match expected {
            b'd' => byte.is_ascii_digit(),
            literal => byte == literal,
        });
    if !shaped {
        return false;
    }
    let field = |from: usize, to: usize| text[from..to].parse::<u32>().unwrap_or(u32::MAX);
    let (year, month) = (field(0, 4), field(5, 7));
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return false,
    };
    (1..=days).contains(&field(8, 10))
        && field(11, 13) <= 23
        && field(14, 16) <= 59
        && field(17, 19) <= 59
}
