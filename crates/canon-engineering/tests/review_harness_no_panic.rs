//! Independent review of `story:incident-response-protocol` (wave 2026-10-04-w8), at 0d4eccc.
//!
//! Invariant under check: a malformed fixture is refused or reported, never a panic. The one
//! `expect` on fixture input in the harness is the re-serialization of `add_authority` entries
//! (`crates/canon-engineering/tests/support/mod.rs:515`); these probes drive it with YAML values
//! that are not `canon-authority/1` decisions, including a mapping whose key is itself a sequence
//! or a mapping.

#[allow(dead_code)] // uses part of the harness; fixture_harness.rs uses all of it
mod support;

use std::panic::{AssertUnwindSafe, catch_unwind};

use support::Fixture;

const INC_492: &str = "fixtures/incident-response/inc-492.fixture.yaml";

#[test]
fn odd_authority_entries_are_refused_or_reported_without_a_panic() {
    let text = std::fs::read_to_string(support::repo_root().join(INC_492)).expect("inc-492 reads");
    let granted = "      - {capability: release.rollback, decision: granted}\n";
    assert!(text.contains(granted), "inc-492 grants release.rollback");
    let compiled = support::compile_protocol("protocols/incident-response/1.yaml")
        .unwrap_or_else(|error| panic!("{error}"));

    let mut panicked = Vec::new();
    for entry in [
        "      - {[a, b]: c}\n",
        "      - {{x: 1}: y}\n",
        "      - !custom {capability: release.rollback, decision: granted}\n",
        "      - .nan\n",
        "      - ~\n",
        "      - [release.rollback, granted]\n",
        "      - {capability: release.rollback, decision: granted, decision: denied}\n",
    ] {
        let text = text.replacen(granted, entry, 1);
        let run = catch_unwind(AssertUnwindSafe(|| {
            if let Ok(fixture) = Fixture::from_yaml(&text) {
                let _ = fixture.check(&compiled);
            }
        }));
        if run.is_err() {
            panicked.push(entry.trim().to_owned());
        }
    }
    assert!(
        panicked.is_empty(),
        "the harness panicked on add_authority entries: {panicked:?}"
    );
}
