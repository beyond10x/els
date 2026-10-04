//! Independent review of `story:incident-response-protocol` (wave 2026-10-04-w8), at 0d4eccc.
//!
//! Invariant under check: `incident.response/1` never discharges `restore_service`, and never
//! admits `emergency.leave`, while the state of the *service* is unknown.
//!
//! `service.healthy` is `true_when: {evidence: {kind: operational_observation, result: healthy}}`
//! (`protocols/incident-response/1.yaml:33-37`). A Canon evidence match names a kind and a result,
//! not a subject (`b10x_canon::model::EvidenceMatch`), and revision binding keeps every record
//! whose subject is at its current revision. The protocol declares two artifacts, `service` and
//! `release`. So a healthy `operational_observation` about the release, which the rollback does
//! not move (`inc-492` keeps `release: r42` throughout), applies to `service.healthy` once the
//! service's own records are excluded by the rollback.
//!
//! The scenario is `inc-492` as shipped, with one change: in `rolled-back`, where nothing of the
//! service's new revision s2 has been observed, the evidence is a healthy observation and a bounded
//! impact assessment of the release r42, the release that stays current.

#[allow(dead_code)] // uses part of the harness; fixture_harness.rs uses all of it
mod support;

// Independent review F1: the decision helpers and their Canon types went with the decision the
// test no longer reaches; Canon refuses the scenario before deciding anything.
use support::Fixture;

const INC_492: &str = "fixtures/incident-response/inc-492.fixture.yaml";

/// The `rolled-back` state as `inc-492` ships it, up to its expectation.
const ROLLED_BACK: &str = "  - id: rolled-back
    set_revisions:
      service: s2
    add_authority: []
    add_evidence: []
";

/// The same state, with evidence about the release only: nothing about the service.
const ROLLED_BACK_RELEASE_OBSERVED: &str = "  - id: rolled-back
    set_revisions:
      service: s2
    add_authority: []
    add_evidence:
      - observed_at: \"2026-10-04T11:35:00Z\"
        record:
          format: canon-evidence/1
          id: release-health-1
          kind: operational_observation
          result: healthy
          subject: release
          subject_revision: r42
      - observed_at: \"2026-10-04T11:36:00Z\"
        record:
          format: canon-evidence/1
          id: release-impact-1
          kind: impact_assessment
          result: bounded
          subject: release
          subject_revision: r42
";

#[test]
fn restore_service_is_not_discharged_by_an_observation_of_the_release() {
    let text = std::fs::read_to_string(support::repo_root().join(INC_492)).expect("inc-492 reads");
    assert!(
        text.contains(ROLLED_BACK),
        "inc-492 holds the rolled-back state as shipped"
    );
    let text = text.replacen(ROLLED_BACK, ROLLED_BACK_RELEASE_OBSERVED, 1);

    let fixture = Fixture::from_yaml(&text).unwrap_or_else(|error| panic!("loads: {error}"));
    let compiled = fixture
        .compile()
        .unwrap_or_else(|error| panic!("the protocol compiles: {error}"));
    // Independent review F1: the protocol declares only the service, so Canon refuses a record
    // about the release as `undeclared-artifact` instead of letting it decide `service.healthy`,
    // `restore_service` or `emergency.leave`.
    let refusal = fixture
        .evaluate(&compiled, "rolled-back")
        .expect_err("Canon refuses an observation of the release");
    assert_eq!(refusal.code(), "undeclared-artifact", "{refusal}");
    assert_eq!(
        refusal.to_string(),
        "evidence `release-health-1` is about artifact `release`, which the protocol does not \
         declare"
    );
}
