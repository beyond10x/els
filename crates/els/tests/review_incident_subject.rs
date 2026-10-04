//! Independent review of `story:incident-response-protocol` (wave 2026-10-04-w8), at 0d4eccc.
//!
//! Invariant under check: `incident.response/1` never discharges `restore_service`, and never
//! admits `emergency.leave`, while the state of the *service* is unknown.
//!
//! The protocol declares two artifacts, `service` and `release`, and the rollback does not move
//! the release (`inc-492` keeps `release: r42` throughout). A match that names no subject reads a
//! record about any declared artifact at its current revision, so a healthy
//! `operational_observation` about the release would decide `service.healthy` once the service's
//! own records are excluded by the rollback. `service.healthy` and `impact.bounded` match only
//! evidence about the service (`subject: service`, story:incident-response-subject-binding).
//!
//! The scenario is `inc-492` as shipped, with one change: in `rolled-back`, where nothing of the
//! service's new revision s2 has been observed, the evidence is a healthy observation and a bounded
//! impact assessment of the release r42, the release that stays current.

#[allow(dead_code)] // uses part of the harness; fixture_harness.rs uses all of it
mod support;

use b10x_canon::model::{ClaimId, Truth};
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
    // Canon admits the records about the release and decides; matches bound to the service do
    // not read them, so the service's health and impact stay UNKNOWN, the restoration stays open
    // and emergency mode holds.
    let decision = fixture
        .evaluate(&compiled, "rolled-back")
        .unwrap_or_else(|refusal| panic!("Canon decides the state: {refusal}"));
    for about_service in ["service.healthy", "impact.bounded"] {
        assert_eq!(
            decision
                .claims
                .get(&ClaimId::new(about_service))
                .expect("the claim is decided")
                .value,
            Truth::Unknown,
            "a record about the release decided `{about_service}`"
        );
    }
    let obligations = decision.obligations.as_ref().expect("obligations");
    let restore = obligations
        .as_array()
        .expect("obligations is an array")
        .iter()
        .find(|entry| entry["id"] == "restore_service")
        .expect("restore_service");
    assert_eq!(restore["status"], "open", "{obligations}");
    let actions = decision.actions.as_ref().expect("actions");
    assert_eq!(actions["emergency.leave"]["status"], "blocked", "{actions}");
}
