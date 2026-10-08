//! Adversary pass 2 on `story:incident-investigation-obligation`. The protocol, its obligation
//! and `docs/examples/incident-response.md` now say `investigate_cause` "stays open after emergency
//! mode ends until a cause analysis of a current revision of the service or the release identifies
//! the cause". Each case gives inc-492's last state a cause analysis that is *not* of a current
//! revision of the service or the release, and asserts the investigation is not discharged by it:
//! either Canon (or the harness) refuses the record, or the obligation stays open.
//!
//! No file in the repository is written.

#[allow(dead_code)] // uses part of the harness; fixture_harness.rs uses all of it
mod support;

use support::Fixture;

const INC_492: &str = "fixtures/incident-response/inc-492.fixture.yaml";

/// The cause analysis `cause-identified-after-restore` adds, as the fixture writes it.
const CAUSE_2: &str = "          id: cause-2
          kind: cause_analysis
          result: identified
          subject: service
          subject_revision: s2
";

/// `investigate_cause`'s status in inc-492's last state with `cause-2` replaced by `record`, or
/// the refusal (load, compile or evaluation) that record met.
fn investigation_with(record: &str) -> Result<String, String> {
    let text = std::fs::read_to_string(support::repo_root().join(INC_492)).expect("inc-492 reads");
    assert!(text.contains(CAUSE_2), "inc-492 holds `cause-2` as written");
    let fixture = Fixture::from_yaml(&text.replacen(CAUSE_2, record, 1))
        .map_err(|error| format!("load: {error}"))?;
    let compiled = fixture
        .compile()
        .map_err(|error| format!("compile: {error}"))?;
    let decision = fixture
        .evaluate(&compiled, "cause-identified-after-restore")
        .map_err(|refusal| format!("evaluate: {refusal}"))?;
    Ok(decision
        .obligations
        .as_ref()
        .and_then(|section| section.as_array())
        .and_then(|entries| {
            entries
                .iter()
                .find(|entry| entry["id"] == "investigate_cause")
        })
        .and_then(|entry| entry["status"].as_str())
        .expect("investigate_cause has a status")
        .to_owned())
}

fn assert_not_discharged(record: &str, what: &str) {
    let outcome = investigation_with(record);
    eprintln!("{what}: {outcome:?}");
    assert!(
        outcome.as_deref() != Ok("discharged"),
        "{what} discharged investigate_cause: {outcome:?}"
    );
}

/// An analysis of the release at a revision the case no longer has (the case holds `r42`).
#[test]
fn a_cause_analysis_of_a_stale_release_revision_does_not_discharge_it() {
    assert_not_discharged(
        "          id: cause-2
          kind: cause_analysis
          result: identified
          subject: release
          subject_revision: r41
",
        "a cause analysis of release r41",
    );
}

/// An analysis of an artifact that is neither the service nor the release.
#[test]
fn a_cause_analysis_of_another_artifact_does_not_discharge_it() {
    assert_not_discharged(
        "          id: cause-2
          kind: cause_analysis
          result: identified
          subject: deployment
          subject_revision: d1
",
        "a cause analysis of artifact `deployment`",
    );
}

/// An analysis of the service that names no revision: not of a *current* revision.
#[test]
fn a_cause_analysis_of_the_service_without_a_revision_does_not_discharge_it() {
    assert_not_discharged(
        "          id: cause-2
          kind: cause_analysis
          result: identified
          subject: service
",
        "a cause analysis of the service without a revision",
    );
}

/// An analysis that names no subject at all: of neither the service nor the release.
#[test]
fn a_cause_analysis_with_no_subject_does_not_discharge_it() {
    assert_not_discharged(
        "          id: cause-2
          kind: cause_analysis
          result: identified
",
        "a cause analysis with no subject",
    );
}

/// An analysis of the service at `s2` observed after the fixture's instant (`12:00`): it does not
/// exist yet at evaluation time, so it identifies nothing.
#[test]
fn a_cause_analysis_observed_after_the_instant_does_not_discharge_it() {
    let text = std::fs::read_to_string(support::repo_root().join(INC_492)).expect("inc-492 reads");
    let from = "      - observed_at: \"2026-10-04T11:50:00Z\"\n";
    assert_eq!(
        text.matches(from).count(),
        1,
        "inc-492 has one 11:50 record"
    );
    let fixture = match Fixture::from_yaml(&text.replacen(
        from,
        "      - observed_at: \"2026-10-04T12:30:00Z\"\n",
        1,
    )) {
        Ok(fixture) => fixture,
        Err(error) => {
            eprintln!("a cause analysis observed after the instant: refused at load: {error}");
            return;
        }
    };
    let compiled = fixture.compile().expect("compiles");
    let outcome = fixture
        .evaluate(&compiled, "cause-identified-after-restore")
        .map(|decision| {
            decision
                .obligations
                .as_ref()
                .and_then(|section| section.as_array())
                .and_then(|entries| {
                    entries
                        .iter()
                        .find(|entry| entry["id"] == "investigate_cause")
                })
                .and_then(|entry| entry["status"].as_str())
                .expect("investigate_cause has a status")
                .to_owned()
        })
        .map_err(|refusal| refusal.to_string());
    eprintln!("a cause analysis observed after the instant: {outcome:?}");
    assert!(
        outcome.as_deref() != Ok("discharged"),
        "a cause analysis observed after the instant discharged investigate_cause: {outcome:?}"
    );
}
