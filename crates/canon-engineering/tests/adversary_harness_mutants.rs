//! Adversary pass 1 on `story:fixture-harness`: cases that kill mutants of the harness the
//! acceptance suite leaves alive. Each names the mutant it kills.

#[allow(dead_code)] // uses part of the harness; fixture_harness.rs uses all of it
mod support;

use std::path::{Path, PathBuf};

use support::{CompileError, Fixture};

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

fn observation(id: &str, result: &str) -> String {
    format!(
        "      - observed_at: \"2026-10-03T22:00:00Z\"\n        record: {{format: canon-evidence/1, id: {id}, kind: operational_observation, result: {result}, subject: service, subject_revision: r1}}\n"
    )
}

/// States are cumulative. In the smoke fixture the first state adds nothing, so evaluating a
/// state over its own `add_evidence` only gives the same answers there. Here the evidence comes
/// first and a later state adds none, then a disagreeing record.
/// Kills: `self.states[..=end]` -> `self.states[end..=end]` in `Fixture::evidence`.
#[test]
fn adversary_harness_later_states_carry_earlier_evidence() {
    let states = format!(
        "  - id: observed\n    add_evidence:\n{}    expect:\n      claims:\n        service.healthy: true\n\
         \x20 - id: nothing-new\n    add_evidence: []\n    expect:\n      claims:\n        service.healthy: true\n\
         \x20 - id: disagreeing\n    add_evidence:\n{}    expect:\n      claims:\n        service.healthy: unknown\n",
        observation("observation-1", "healthy"),
        observation("observation-2", "unhealthy"),
    );
    let fixture = Fixture::from_yaml(&smoke_with_states(&states)).unwrap_or_else(|e| panic!("{e}"));
    let ids = |state: &str| -> Vec<String> {
        fixture
            .evidence(state)
            .iter()
            .map(|record| record.id.as_str().to_owned())
            .collect()
    };
    assert_eq!(ids("observed"), ["observation-1"]);
    assert_eq!(ids("nothing-new"), ["observation-1"]);
    assert_eq!(ids("disagreeing"), ["observation-1", "observation-2"]);
    let compiled = fixture.compile().unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(fixture.check(&compiled), Ok(()));
}

/// A state Canon refuses to evaluate is a difference `check` reports, carrying Canon's code and
/// message unchanged; it is never skipped as if it matched.
/// Kills: dropping the `differences.push` in `check`'s `Err(refusal)` arm.
#[test]
fn adversary_harness_check_reports_canon_refusals_verbatim() {
    let fixture = Fixture::from_yaml(&smoke_with("  protocol: smoke", "  protocol: other"))
        .unwrap_or_else(|e| panic!("{e}"));
    let compiled = fixture.compile().unwrap_or_else(|e| panic!("{e}"));
    let refusal = fixture
        .evaluate(&compiled, "initial")
        .expect_err("Canon refuses a case of another protocol");
    assert_eq!(refusal.code(), "protocol-mismatch");
    assert_eq!(
        refusal.to_string(),
        "case `SMOKE-1` is governed by protocol `other`, not by `smoke`"
    );
    let line = |state: &str| {
        format!(
            "state `{state}`: evaluation refused: protocol-mismatch: case `SMOKE-1` is governed \
             by protocol `other`, not by `smoke`"
        )
    };
    assert_eq!(
        fixture.check(&compiled),
        Err(vec![line("initial"), line("healthy-observed")])
    );

    // An evidence id reused by a later state: the earlier state evaluates, the later is refused.
    let states = format!(
        "  - id: first\n    add_evidence:\n{}    expect:\n      claims:\n        service.healthy: true\n\
         \x20 - id: again\n    add_evidence:\n{}    expect:\n      claims:\n        service.healthy: true\n",
        observation("observation-1", "healthy"),
        observation("observation-1", "healthy"),
    );
    let fixture = Fixture::from_yaml(&smoke_with_states(&states)).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(
        fixture.check(&compiled_smoke()),
        Err(vec![
            "state `again`: evaluation refused: duplicate-identifier: evidence `observation-1` is \
             given more than once"
                .to_owned()
        ])
    );
}

fn compiled_smoke() -> support::Compiled {
    support::compile_protocol("fixtures/smoke/protocol.yaml").unwrap_or_else(|e| panic!("{e}"))
}

/// A document that is not `protocol/1` is Canon's parse error, unchanged.
/// Kills: replacing `CompileError::Parse` with a message of the harness's own.
#[test]
fn adversary_harness_compile_passes_canon_parse_errors_through() {
    let path = "protocols/vocabulary.yaml";
    let text = std::fs::read_to_string(support::repo_root().join(path)).expect("reads");
    let canon = b10x_canon::model::parse(&text).expect_err("the vocabulary is not protocol/1");
    match support::compile_protocol(path) {
        Err(CompileError::Parse(error)) => assert_eq!(error, canon),
        other => panic!("expected Canon's parse error, got {other:?}"),
    }
}

/// The documented convention for a built-in protocol's path, which the next stories call.
/// Kills: any change to the `format!` in `protocol_path` (no test calls it).
#[test]
fn adversary_harness_protocol_path_follows_the_convention() {
    assert_eq!(
        support::protocol_path("software-change", 1),
        "protocols/software-change/1.yaml"
    );
    assert_eq!(
        support::protocol_path("incident-response", 1),
        "protocols/incident-response/1.yaml"
    );
}

/// Removes the links this test made inside the repository's ignored `target/` directory.
struct Links(PathBuf);

impl Drop for Links {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
        if let Some(parent) = self.0.parent() {
            let _ = std::fs::remove_dir(parent);
        }
    }
}

/// A path without `..` that leaves the repository through a symbolic link — to a file, or
/// through a linked directory — is refused; a link that stays inside loads.
/// Kills: dropping the `resolved.starts_with(&root)` check in `read_confined`.
#[cfg(unix)]
#[test]
fn adversary_harness_refuses_paths_that_leave_through_a_symlink() {
    use std::os::unix::fs::symlink;

    let root = support::repo_root();
    let outside = Path::new(env!("CARGO_TARGET_TMPDIR")).join("adversary-harness-outside");
    std::fs::create_dir_all(&outside).expect("scratch directory");
    std::fs::write(outside.join("smoke.fixture.yaml"), smoke_text()).expect("outside copy");

    let name = format!("adversary-harness-{}", std::process::id());
    let links = Links(root.join("target").join(&name));
    std::fs::create_dir_all(&links.0).expect("link directory under the ignored target/");
    symlink(
        outside.join("smoke.fixture.yaml"),
        links.0.join("file.yaml"),
    )
    .expect("file link");
    symlink(&outside, links.0.join("dir")).expect("directory link");
    symlink(root.join(SMOKE), links.0.join("inside.yaml")).expect("inside link");

    for leaf in ["file.yaml", "dir/smoke.fixture.yaml"] {
        let path = format!("target/{name}/{leaf}");
        let error = Fixture::load(&path)
            .map(|_| ())
            .expect_err(&format!("`{path}` leaves the repository"));
        assert_eq!(
            error.to_string(),
            format!("fixture path `{path}` resolves outside the repository")
        );
        let error = support::compile_protocol(&path)
            .map(|_| ())
            .expect_err(&format!("`{path}` leaves the repository"));
        assert_eq!(
            error.to_string(),
            format!("protocol path `{path}` resolves outside the repository")
        );
    }
    let inside = format!("target/{name}/inside.yaml");
    Fixture::load(&inside).unwrap_or_else(|e| panic!("a link that stays inside loads: {e}"));
}
