//! Acceptance for `story:incident-response-protocol`: `incident.response/1` is a Canon
//! `protocol/1` document at `protocols/incident-response/1.yaml`, validated, compiled and
//! evaluated by the same Canon kernel as every ELS protocol, through the fixture harness.
//!
//! The fixture `inc-492` (`fixtures/incident-response/`) is transcribed from
//! `docs/examples/incident-response.md`: the operational incident leaves emergency mode on
//! restoration evidence while its cause is still UNKNOWN. A state may set the case snapshot's
//! artifact revisions: the rollback produces a new revision of the service, and the observations
//! of the old revision no longer apply.

mod support;

use b10x_canon::model::{ActionId, ClaimId, Decision, ExclusionReason, Truth};
use b10x_els::vocabulary::{self, Category};
use support::Fixture;

/// The fixture, as a path relative to the repository root.
const INC_492: &str = "fixtures/incident-response/inc-492.fixture.yaml";

/// The value Canon's decision gives `claim`.
fn claim(decision: &Decision, claim: &str) -> Truth {
    decision
        .claims
        .get(&ClaimId::new(claim))
        .unwrap_or_else(|| panic!("the decision has no claim `{claim}`"))
        .value
}

/// The evidence Canon's decision lists as excluded from `claim`, and why, in evidence-id order.
fn excluded(decision: &Decision, claim: &str) -> Vec<(String, ExclusionReason)> {
    decision
        .claims
        .get(&ClaimId::new(claim))
        .unwrap_or_else(|| panic!("the decision has no claim `{claim}`"))
        .excluded_evidence
        .iter()
        .map(|exclusion| (exclusion.evidence.as_str().to_owned(), exclusion.reason))
        .collect()
}

/// The status Canon's decision gives obligation `id`: `open` or `discharged`.
fn obligation(decision: &Decision, id: &str) -> String {
    let section = decision
        .obligations
        .as_ref()
        .expect("the decision has an obligations section");
    let entries = section.as_array().expect("obligations is an array");
    let entry = entries
        .iter()
        .find(|entry| entry["id"] == id)
        .unwrap_or_else(|| panic!("the decision has no obligation `{id}`: {section}"));
    entry["status"]
        .as_str()
        .expect("an obligation's status is text")
        .to_owned()
}

/// The status Canon's decision gives action `id`: `admissible`, `approval-required` or `blocked`.
fn action(decision: &Decision, id: &str) -> String {
    let section = decision
        .actions
        .as_ref()
        .expect("the decision has an actions section");
    section[id]["status"]
        .as_str()
        .unwrap_or_else(|| panic!("the decision has no action `{id}`: {section}"))
        .to_owned()
}

#[test]
fn inc_492_leaves_emergency_while_cause_unknown() {
    // 1. The protocol validates through Canon and compiles to a `canon-ir/1` document.
    let path = support::protocol_path("incident-response", 1);
    let compiled = support::compile_protocol(&path)
        .unwrap_or_else(|error| panic!("{path} validates and compiles: {error}"));
    let reread = b10x_canon::eval::read_ir(compiled.canon_ir())
        .unwrap_or_else(|refusal| panic!("Canon reads the compiled protocol back: {refusal}"));
    assert_eq!(&reread, compiled.ir());
    let ir = compiled.ir();
    assert_eq!(ir.protocol.id.as_str(), "incident.response");

    // Every name the protocol declares is a term of the vocabulary, in its category; this story
    // adds none.
    let declared = |category: Category, names: Vec<&str>| {
        for name in names {
            let term = vocabulary::lookup(name).unwrap_or_else(|error| panic!("{error}"));
            assert_eq!(term.category, category, "`{name}`");
        }
    };
    declared(
        Category::ArtifactKind,
        ir.artifacts.keys().map(|id| id.as_str()).collect(),
    );
    declared(
        Category::EvidenceKind,
        ir.evidence_kinds.keys().map(|id| id.as_str()).collect(),
    );
    declared(
        Category::ClaimId,
        ir.claims.keys().map(|id| id.as_str()).collect(),
    );
    declared(
        Category::ObligationId,
        ir.obligations.keys().map(|id| id.as_str()).collect(),
    );
    declared(
        Category::ActionId,
        ir.actions.keys().map(|id| id.as_str()).collect(),
    );
    let names = |ids: Vec<&str>| ids.into_iter().map(str::to_owned).collect::<Vec<_>>();
    assert_eq!(
        names(ir.claims.keys().map(|id| id.as_str()).collect()),
        ["cause.identified", "impact.bounded", "service.healthy"]
    );
    assert_eq!(
        names(ir.obligations.keys().map(|id| id.as_str()).collect()),
        ["restore_service"]
    );
    assert_eq!(
        names(ir.actions.keys().map(|id| id.as_str()).collect()),
        [
            "emergency.leave",
            "logs.search",
            "metrics.inspect",
            "release.inspect",
            "release.rollback",
            "traffic.shift",
        ]
    );

    // Restoration and investigation progress independently: leaving emergency mode rests on
    // restoration claims and never on `cause.identified`.
    let leave = &ir.actions[&ActionId::new("emergency.leave")];
    let mut tested: Vec<&str> = leave
        .precondition
        .claim_references()
        .into_iter()
        .map(|id| id.as_str())
        .collect();
    tested.sort_unstable();
    assert_eq!(tested, ["impact.bounded", "service.healthy"]);
    assert!(leave.requires.is_empty());

    let fixture = Fixture::load(INC_492).unwrap_or_else(|error| panic!("{INC_492}: {error}"));
    assert_eq!(fixture.protocol_path(), path);
    assert_eq!(fixture.at(), "2026-10-04T12:00:00Z");
    let compiled_again = fixture
        .compile()
        .unwrap_or_else(|error| panic!("the fixture's protocol compiles: {error}"));
    assert_eq!(compiled_again.canon_ir(), compiled.canon_ir());
    assert_eq!(
        fixture.states(),
        ["initial", "rollback-approved", "service-restored"]
    );
    let decide = |state: &str| {
        fixture
            .evaluate(&compiled, state)
            .unwrap_or_else(|refusal| panic!("state `{state}` evaluates: {refusal}"))
    };

    // 2. Impact bounded, the service unhealthy, no cause analysis and no authority decision.
    let initial = decide("initial");
    assert_eq!(claim(&initial, "impact.bounded"), Truth::True);
    assert_eq!(claim(&initial, "service.healthy"), Truth::False);
    assert_eq!(claim(&initial, "cause.identified"), Truth::Unknown);
    assert_eq!(obligation(&initial, "restore_service"), "open");
    for read in ["metrics.inspect", "logs.search", "release.inspect"] {
        assert_eq!(action(&initial, read), "admissible", "{read}");
    }
    for gated in ["traffic.shift", "release.rollback"] {
        assert_eq!(action(&initial, gated), "approval-required", "{gated}");
    }
    assert_eq!(action(&initial, "emergency.leave"), "blocked");

    // 3. The rollback is approved; the service is still unhealthy and emergency mode holds.
    let approved = decide("rollback-approved");
    assert_eq!(action(&approved, "release.rollback"), "admissible");
    assert_eq!(claim(&approved, "service.healthy"), Truth::False);
    assert_eq!(action(&approved, "emergency.leave"), "blocked");

    // 4. The rollback produces a new revision of the service, and a healthy observation of that
    //    revision arrives, with the impact assessed again on it and still no cause analysis.
    //    The records of the old revision no longer apply: Canon lists them as excluded.
    let restored = decide("service-restored");
    assert_eq!(claim(&restored, "service.healthy"), Truth::True);
    assert_eq!(claim(&restored, "impact.bounded"), Truth::True);
    assert_eq!(claim(&restored, "cause.identified"), Truth::Unknown);
    assert_eq!(obligation(&restored, "restore_service"), "discharged");
    assert_eq!(action(&restored, "emergency.leave"), "admissible");
    for (claim, evidence) in [
        ("service.healthy", "health-1"),
        ("impact.bounded", "impact-1"),
    ] {
        assert_eq!(
            excluded(&restored, claim),
            [(evidence.to_owned(), ExclusionReason::RevisionMismatch)],
            "{claim}"
        );
    }
    assert_eq!(excluded(&restored, "cause.identified"), []);

    // The fixture's own expectations are the ones above.
    assert_eq!(fixture.check(&compiled), Ok(()));
}
