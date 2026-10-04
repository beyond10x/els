//! Acceptance for `story:fixture-harness`: one harness, `tests/support/mod.rs`, loads a fixture,
//! validates and compiles its protocol through Canon and evaluates the fixture's states with
//! Canon's evaluator, proven on the `smoke` fixture in `fixtures/smoke/`.
//!
//! The negative cases below decide what the harness refuses or reports instead of evaluating
//! something other than what the fixture says.

mod support;

use b10x_canon::model::{ClaimId, Decision, Truth};
use canon_engineering::vocabulary::{self, Category, Marking};
use support::Fixture;

/// The smoke fixture, as a path relative to the repository root.
const SMOKE: &str = "fixtures/smoke/smoke.fixture.yaml";

/// The value Canon's decision gives `claim`.
fn value(decision: &Decision, claim: &str) -> Truth {
    decision
        .claims
        .get(&ClaimId::new(claim))
        .unwrap_or_else(|| panic!("the decision has no claim `{claim}`"))
        .value
}

/// The smoke fixture's text, read at run time.
fn smoke_text() -> String {
    std::fs::read_to_string(support::repo_root().join(SMOKE)).expect("the smoke fixture reads")
}

/// The smoke fixture's text with `from` replaced by `to`, which must occur in it.
fn smoke_with(from: &str, to: &str) -> String {
    let text = smoke_text();
    assert!(text.contains(from), "the smoke fixture contains `{from}`");
    text.replacen(from, to, 1)
}

#[test]
fn harness_compiles_and_evaluates_smoke_fixture() {
    // The smoke protocol's claim and evidence kind are core terms of the vocabulary.
    let claim = vocabulary::lookup("service.healthy").expect("service.healthy is a term");
    assert_eq!(
        (claim.category, claim.marking),
        (Category::ClaimId, Marking::Core)
    );
    let kind = vocabulary::lookup("operational_observation").expect("a term");
    assert_eq!(
        (kind.category, kind.marking),
        (Category::EvidenceKind, Marking::Core)
    );

    let fixture = Fixture::load(SMOKE).unwrap_or_else(|error| panic!("{SMOKE}: {error}"));
    assert_eq!(fixture.protocol_path(), "fixtures/smoke/protocol.yaml");
    assert_eq!(fixture.at(), "2026-10-04T00:00:00Z");

    // 1. The smoke protocol validates through Canon and compiles to a `canon-ir/1` document.
    let compiled = fixture
        .compile()
        .unwrap_or_else(|error| panic!("the smoke protocol compiles: {error}"));
    let reread = b10x_canon::eval::read_ir(compiled.canon_ir())
        .unwrap_or_else(|refusal| panic!("Canon reads the compiled protocol back: {refusal}"));
    assert_eq!(&reread, compiled.ir());

    // 2. The initial state holds no evidence: `service.healthy` is UNKNOWN, not FALSE.
    assert!(fixture.evidence("initial").is_empty());
    let initial = fixture
        .evaluate(&compiled, "initial")
        .unwrap_or_else(|refusal| panic!("state `initial` evaluates: {refusal}"));
    assert_eq!(value(&initial, "service.healthy"), Truth::Unknown);

    // 3. One operational observation of a healthy service, observed before T0, added to it:
    //    `service.healthy` is TRUE.
    let observed = fixture.evidence("healthy-observed");
    assert_eq!(observed.len(), 1);
    assert_eq!(observed[0].kind.as_str(), "operational_observation");
    assert_eq!(observed[0].result.as_deref(), Some("healthy"));
    let healthy = fixture
        .evaluate(&compiled, "healthy-observed")
        .unwrap_or_else(|refusal| panic!("state `healthy-observed` evaluates: {refusal}"));
    assert_eq!(value(&healthy, "service.healthy"), Truth::True);

    // The fixture's own expectations are the ones above.
    assert_eq!(fixture.check(&compiled), Ok(()));
}

#[test]
fn harness_reports_a_state_whose_decision_differs_from_its_expectation() {
    let text = smoke_with("service.healthy: unknown", "service.healthy: false");
    let fixture = Fixture::from_yaml(&text).unwrap_or_else(|error| panic!("{error}"));
    let compiled = fixture.compile().unwrap_or_else(|error| panic!("{error}"));
    assert_eq!(
        fixture.check(&compiled),
        Err(vec![
            "state `initial`: claim `service.healthy` is unknown, expected false".to_owned()
        ])
    );
}

#[test]
fn harness_reports_a_claim_the_expectation_does_not_list() {
    let text = smoke_with(
        "        service.healthy: unknown\n",
        "        service.unlisted: unknown\n",
    );
    let fixture = Fixture::from_yaml(&text).unwrap_or_else(|error| panic!("{error}"));
    let compiled = fixture.compile().unwrap_or_else(|error| panic!("{error}"));
    assert_eq!(
        fixture.check(&compiled),
        Err(vec![
            "state `initial`: claim `service.healthy` is unknown, expected nothing".to_owned(),
            "state `initial`: claim `service.unlisted` is not declared, expected unknown"
                .to_owned(),
        ])
    );
}

#[test]
fn harness_refuses_a_fixture_it_cannot_evaluate_as_written() {
    for (from, to, refusal) in [
        (
            "format: els-fixture/1",
            "format: els-fixture/2",
            "format is `els-fixture/2`, expected `els-fixture/1`",
        ),
        (
            "at: \"2026-10-04T00:00:00Z\"",
            "at: \"2026-10-04\"",
            "evaluation instant `2026-10-04` is not written YYYY-MM-DDTHH:MM:SSZ",
        ),
        (
            "observed_at: \"2026-10-03T23:00:00Z\"",
            "observed_at: \"2026-10-04T00:00:01Z\"",
            "state `healthy-observed`: evidence `observation-1` is observed at \
             `2026-10-04T00:00:01Z`, after the evaluation instant `2026-10-04T00:00:00Z`",
        ),
        (
            "observed_at: \"2026-10-03T23:00:00Z\"",
            "observed_at: \"yesterday\"",
            "state `healthy-observed`: evidence `observation-1` observation instant `yesterday` \
             is not written YYYY-MM-DDTHH:MM:SSZ",
        ),
        (
            "protocol: fixtures/smoke/protocol.yaml",
            "protocol: ../smoke/protocol.yaml",
            "protocol path `../smoke/protocol.yaml` is absolute or leaves the repository",
        ),
        (
            "protocol: fixtures/smoke/protocol.yaml",
            "protocol: /etc/protocol.yaml",
            "protocol path `/etc/protocol.yaml` is absolute or leaves the repository",
        ),
        (
            "  - id: healthy-observed",
            "  - id: initial",
            "state `initial` is given more than once",
        ),
        (
            "states:\n",
            "authority: []\nstates:\n",
            "unknown field `authority`",
        ),
    ] {
        let text = smoke_with(from, to);
        let error = match Fixture::from_yaml(&text) {
            Ok(_) => panic!("a fixture with `{to}` loads; expected `{refusal}`"),
            Err(error) => error.to_string(),
        };
        assert!(
            error.contains(refusal),
            "a fixture with `{to}` is refused as `{error}`, expected `{refusal}`"
        );
    }
}

#[test]
fn harness_refuses_a_fixture_without_states() {
    let text = smoke_text();
    let (head, _) = text
        .split_once("\nstates:\n")
        .expect("the smoke fixture lists states");
    let text = format!("{head}\nstates: []\n");
    let error = Fixture::from_yaml(&text)
        .map(|_| ())
        .expect_err("no states is refused");
    assert_eq!(error.to_string(), "the fixture lists no states");
}
