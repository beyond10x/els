//! Adversary pass 2 on `story:fixture-harness`: what a fixture can write that the harness reads as
//! something other than what was written — a key given twice, a key given no value, a fixture whose
//! name is not its file — and how YAML anchors, aliases and merge keys reach Canon's readers.

#[allow(dead_code)] // uses part of the harness; fixture_harness.rs uses all of it
mod support;

use std::path::PathBuf;

use support::Fixture;

const SMOKE: &str = "fixtures/smoke/smoke.fixture.yaml";

fn smoke_text() -> String {
    std::fs::read_to_string(support::repo_root().join(SMOKE)).expect("the smoke fixture reads")
}

fn smoke_with(from: &str, to: &str) -> String {
    let text = smoke_text();
    assert!(text.contains(from), "the smoke fixture contains `{from}`");
    text.replacen(from, to, 1)
}

/// The smoke fixture's head (everything before `states:`) followed by `states`.
fn smoke_with_states(states: &str) -> String {
    let text = smoke_text();
    let (head, _) = text
        .split_once("\nstates:\n")
        .expect("the smoke fixture lists states");
    format!("{head}\nstates:\n{states}")
}

/// A directory inside the repository's ignored `target/`, removed when dropped.
struct Scratch(PathBuf);

impl Scratch {
    fn new(tag: &str) -> (Self, String) {
        let name = format!("adversary2-harness-{tag}-{}", std::process::id());
        let dir = support::repo_root().join("target").join(&name);
        std::fs::create_dir_all(&dir).expect("scratch directory under the ignored target/");
        (Scratch(dir), format!("target/{name}"))
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// The format says "Unknown keys are refused, so a fixture cannot carry an input the harness would
/// drop." A claim written twice in `expect.claims` is such an input: `claims` is a `BTreeMap`, and
/// serde keeps the last value without a word. Here the fixture expects `service.healthy` to be both
/// `false` and `unknown` in the initial state; the harness drops `false` and `check` passes.
/// (Everywhere else a repeated key is refused: struct fields by serde, the case and the records by
/// `serde_yaml_ng::Mapping`.)
#[test]
fn adversary2_harness_refuses_a_claim_expected_twice() {
    let text = smoke_with(
        "        service.healthy: unknown\n",
        "        service.healthy: false\n        service.healthy: unknown\n",
    );
    match Fixture::from_yaml(&text) {
        Err(_) => {}
        Ok(fixture) => {
            let compiled = fixture.compile().unwrap_or_else(|e| panic!("{e}"));
            panic!(
                "a fixture expecting `service.healthy` twice (false, then unknown) loads, and \
                 check gives {:?}: the `false` expectation was dropped",
                fixture.check(&compiled)
            );
        }
    }
}

/// Canon refuses a key written with no value rather than read it as its default
/// (`b10x_canon::model::present`: "an author who wrote a key and forgot its value would silently
/// get the default"). The harness's own `add_evidence:` with nothing after it is read as an empty
/// list: a state whose evidence was forgotten loads as a state that adds none.
#[test]
fn adversary2_harness_refuses_add_evidence_written_without_a_value() {
    let text = smoke_with("    add_evidence: []\n", "    add_evidence:\n");
    if let Ok(fixture) = Fixture::from_yaml(&text) {
        panic!(
            "`add_evidence:` with no value loads as {} records for state `initial`",
            fixture.evidence("initial").len()
        );
    }
}

/// The format names one `<fixture-id>.fixture.yaml` file per fixture. A file named otherwise loads
/// under the name it declares, so two files can declare the same fixture id and nothing notices:
/// here `other.fixture.yaml` declares `id: smoke`, the id of `fixtures/smoke/smoke.fixture.yaml`.
#[test]
fn adversary2_harness_refuses_a_fixture_whose_id_is_not_its_file_name() {
    let (scratch, relative) = Scratch::new("ids");
    std::fs::write(scratch.0.join("other.fixture.yaml"), smoke_text()).expect("copy");
    let path = format!("{relative}/other.fixture.yaml");
    let smoke = Fixture::load(SMOKE).unwrap_or_else(|e| panic!("{e}"));
    if let Ok(other) = Fixture::load(&path) {
        panic!(
            "`{path}` loads as fixture `{}`, the id `{SMOKE}` also declares (`{}`)",
            other.id(),
            smoke.id()
        );
    }
}

/// Anchors and aliases are expanded before Canon's readers see a record, so an aliased record is
/// the same record (Canon refuses its id the second time); a merge key is not applied, so Canon's
/// reader refuses `<<` as an unknown field. Both match what Canon's text reader does.
#[test]
fn adversary2_harness_aliases_and_merge_keys_reach_canon_as_written() {
    let record = "{format: canon-evidence/1, id: observation-1, kind: operational_observation, \
                  result: healthy, subject: service, subject_revision: r1}";
    let states = |second: &str| {
        format!(
            "  - id: first\n    add_evidence:\n      - observed_at: \"2026-10-03T22:00:00Z\"\n        \
             record: &obs {record}\n    expect:\n      claims:\n        service.healthy: true\n\
             \x20 - id: again\n    add_evidence:\n      - observed_at: \"2026-10-03T22:00:00Z\"\n        \
             record: {second}\n    expect:\n      claims:\n        service.healthy: true\n"
        )
    };

    let fixture =
        Fixture::from_yaml(&smoke_with_states(&states("*obs"))).unwrap_or_else(|e| panic!("{e}"));
    let ids: Vec<String> = fixture
        .evidence("again")
        .iter()
        .map(|r| r.id.as_str().to_owned())
        .collect();
    assert_eq!(ids, ["observation-1", "observation-1"]);
    let compiled = fixture.compile().unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(
        fixture.check(&compiled),
        Err(vec![
            "state `again`: evaluation refused: duplicate-identifier: evidence `observation-1` is \
             given more than once"
                .to_owned()
        ])
    );

    let merged = Fixture::from_yaml(&smoke_with_states(&states("{<<: *obs, id: observation-2}")))
        .map(|_| ())
        .expect_err("a merge key is not applied");
    let message = merged.to_string();
    assert!(
        message.starts_with("state `again`: evidence 1: malformed-input: ")
            && message.contains("unknown field `<<`"),
        "{message}"
    );
}
