//! Adversary pass on `story:incident-investigation-obligation`: `investigate_cause` read against
//! the documents the unit wrote about it. The protocol's description says the obligation "stays
//! open after emergency mode ends until the cause of the service's current revision is
//! identified", and `docs/examples/incident-response.md` says it "stays open until a cause
//! analysis of the restored service revision identifies the cause".
//!
//! Each case starts from `inc-492` and changes only the cause analysis its last state adds. No file
//! in the repository is written.

#[allow(dead_code)] // uses part of the harness; fixture_harness.rs uses all of it
mod support;

use b10x_canon::model::{ClaimId, Decision, Truth};
use support::Fixture;

const INC_492: &str = "fixtures/incident-response/inc-492.fixture.yaml";

/// The cause analysis `cause-identified-after-restore` adds, as the fixture writes it.
const CAUSE_2: &str = "          id: cause-2
          kind: cause_analysis
          result: identified
          subject: service
          subject_revision: s2
";

/// `inc-492`'s last state with its cause analysis replaced by `record` (same indentation).
fn last_state_with(record: &str) -> Decision {
    let text = std::fs::read_to_string(support::repo_root().join(INC_492)).expect("inc-492 reads");
    assert!(text.contains(CAUSE_2), "inc-492 holds `cause-2` as written");
    let fixture = Fixture::from_yaml(&text.replacen(CAUSE_2, record, 1))
        .unwrap_or_else(|error| panic!("{error}"));
    let compiled = fixture.compile().unwrap_or_else(|error| panic!("{error}"));
    fixture
        .evaluate(&compiled, "cause-identified-after-restore")
        .unwrap_or_else(|refusal| panic!("evaluates: {refusal}"))
}

fn obligation(decision: &Decision, id: &str) -> String {
    decision
        .obligations
        .as_ref()
        .and_then(|section| section.as_array())
        .and_then(|entries| entries.iter().find(|entry| entry["id"] == id))
        .and_then(|entry| entry["status"].as_str())
        .unwrap_or_else(|| panic!("no obligation `{id}`"))
        .to_owned()
}

fn action(decision: &Decision, id: &str) -> String {
    decision.actions.as_ref().expect("actions")[id]["status"]
        .as_str()
        .expect("a status")
        .to_owned()
}

fn cause(decision: &Decision) -> Truth {
    decision
        .claims
        .get(&ClaimId::new("cause.identified"))
        .expect("cause.identified")
        .value
}

/// The protocol's own description: `investigate_cause` stays open "until a cause analysis of a
/// current revision of the service or the release identifies the cause". `cause.identified`
/// carries no subject binding, because the cause can be in the release. The release, which the
/// rollback did not move, is at `r42`; a cause analysis of it — what `release.inspect` declares
/// it may produce — identifies the cause and discharges the investigation.
#[test]
fn a_cause_analysis_of_the_release_discharges_the_investigation() {
    let decision = last_state_with(
        "          id: cause-2
          kind: cause_analysis
          result: identified
          subject: release
          subject_revision: r42
",
    );
    assert_eq!(cause(&decision), Truth::True);
    assert_eq!(obligation(&decision, "investigate_cause"), "discharged");
    assert_eq!(action(&decision, "emergency.leave"), "admissible");
}

/// The claim under attack, stated directly: a cause analysis of the stale revision `s1`, observed
/// after the restoration on `s2`, does not discharge the investigation.
#[test]
fn a_late_cause_analysis_of_the_stale_service_revision_leaves_it_open() {
    let decision = last_state_with(
        "          id: cause-2
          kind: cause_analysis
          result: identified
          subject: service
          subject_revision: s1
",
    );
    assert_eq!(cause(&decision), Truth::Unknown);
    assert_eq!(obligation(&decision, "investigate_cause"), "open");
    assert_eq!(obligation(&decision, "restore_service"), "discharged");
    assert_eq!(action(&decision, "emergency.leave"), "admissible");
}

/// A cause analysis of `s2` that did not identify the cause does not discharge the investigation,
/// and does not move emergency mode either.
#[test]
fn an_inconclusive_cause_analysis_leaves_it_open() {
    let decision = last_state_with(
        "          id: cause-2
          kind: cause_analysis
          result: inconclusive
          subject: service
          subject_revision: s2
",
    );
    assert_ne!(cause(&decision), Truth::True);
    assert_eq!(obligation(&decision, "investigate_cause"), "open");
    assert_eq!(action(&decision, "emergency.leave"), "admissible");
}
