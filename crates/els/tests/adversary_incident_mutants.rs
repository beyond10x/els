//! Adversary pass 1 on `story:incident-response-protocol`: protocol mutants that are wrong by the
//! story's Outcome and still pass every check the acceptance makes — the structural independence
//! check of `inc_492_leaves_emergency_while_cause_unknown` and every expectation of `inc-492`.
//!
//! The fixture's expectations are replayed here with Canon directly (the harness's `Compiled`
//! is built only from a file in the repository, and a mutant must not be written there): each
//! state's case snapshot with the revisions set so far, the evidence so far (read by the harness)
//! and the authority decisions so far, compared both ways with the state's `expect` maps, as
//! `Fixture::check` compares them.

#[allow(dead_code)] // uses part of the harness; fixture_harness.rs uses all of it
mod support;

use std::collections::BTreeMap;

use b10x_canon::eval::{self, Supplied};
use b10x_canon::ir::Ir;
use b10x_canon::model::{ActionId, Decision, EvidenceRecord};
use serde_yaml_ng::Value;

const PROTOCOL: &str = "protocols/incident-response/1.yaml";
const INC_492: &str = "fixtures/incident-response/inc-492.fixture.yaml";

fn read(path: &str) -> String {
    std::fs::read_to_string(support::repo_root().join(path))
        .unwrap_or_else(|error| panic!("{path}: {error}"))
}

fn compile(text: &str) -> Ir {
    let protocol = b10x_canon::model::parse(text).unwrap_or_else(|error| panic!("{error}"));
    b10x_canon::validate::validate(&protocol).unwrap_or_else(|problems| panic!("{problems:?}"));
    b10x_canon::ir::compile(&protocol).unwrap_or_else(|problems| panic!("{problems:?}"))
}

/// The protocol with `from` replaced by `to` once, compiled by Canon.
fn mutant(from: &str, to: &str) -> Ir {
    let text = read(PROTOCOL);
    assert!(text.contains(from), "the protocol contains `{from}`");
    compile(&text.replacen(from, to, 1))
}

/// The acceptance test's structural check, as written at
/// `incident_response_protocol.rs:133-142`.
fn independence_check_passes(ir: &Ir) -> bool {
    let leave = &ir.actions[&ActionId::new("emergency.leave")];
    let mut tested: Vec<&str> = leave
        .precondition
        .claim_references()
        .into_iter()
        .map(|id| id.as_str())
        .collect();
    tested.sort_unstable();
    tested == ["impact.bounded", "service.healthy"] && leave.requires.is_empty()
}

fn text(value: &Value) -> String {
    match value {
        Value::Bool(flag) => flag.to_string(),
        Value::String(text) => text.clone(),
        other => panic!("unexpected expectation {other:?}"),
    }
}

fn expected(map: &Value) -> BTreeMap<String, String> {
    map.as_mapping()
        .map(|entries| {
            entries
                .iter()
                .map(|(key, value)| (text(key), text(value)))
                .collect()
        })
        .unwrap_or_default()
}

fn statuses(decision: &Decision) -> [BTreeMap<String, String>; 3] {
    let claims = decision
        .claims
        .iter()
        .map(|(id, entry)| (id.as_str().to_owned(), entry.value.to_string()))
        .collect();
    let obligations = decision
        .obligations
        .as_ref()
        .and_then(|section| section.as_array())
        .map(|entries| {
            entries
                .iter()
                .map(|entry| {
                    (
                        entry["id"].as_str().unwrap().to_owned(),
                        entry["status"].as_str().unwrap().to_owned(),
                    )
                })
                .collect()
        })
        .unwrap_or_default();
    let actions = decision
        .actions
        .as_ref()
        .and_then(|section| section.as_object())
        .map(|entries| {
            entries
                .iter()
                .map(|(id, entry)| (id.clone(), entry["status"].as_str().unwrap().to_owned()))
                .collect()
        })
        .unwrap_or_default();
    [claims, obligations, actions]
}

fn decide(ir: &Ir, case: &Value, evidence: &[EvidenceRecord], authority: Option<&str>) -> Decision {
    let case = eval::case_from_value(case).unwrap_or_else(|refusal| panic!("{refusal}"));
    let supplied = Supplied {
        authority,
        ..Supplied::default()
    };
    eval::evaluate_with(ir, &case, evidence, supplied).unwrap_or_else(|refusal| panic!("{refusal}"))
}

/// Every difference between `inc-492`'s expectations and Canon's decisions under `ir`.
fn fixture_differences(ir: &Ir) -> Vec<String> {
    let fixture = support::Fixture::load(INC_492).unwrap_or_else(|error| panic!("{error}"));
    let document: Value = serde_yaml_ng::from_str(&read(INC_492)).unwrap();
    let mut case = document["case"].clone();
    let mut authority: Option<Vec<Value>> = None;
    let mut differences = Vec::new();
    for state in document["states"].as_sequence().unwrap() {
        let id = state["id"].as_str().unwrap();
        if let Some(revisions) = state.get("set_revisions").and_then(Value::as_mapping) {
            for (artifact, revision) in revisions {
                let slot = case
                    .get_mut("artifacts")
                    .and_then(|artifacts| artifacts.get_mut(artifact.as_str().unwrap()))
                    .and_then(|entry| entry.get_mut("revision"))
                    .unwrap();
                *slot = revision.clone();
            }
        }
        if let Some(decisions) = state.get("add_authority").and_then(Value::as_sequence) {
            authority
                .get_or_insert_with(Vec::new)
                .extend(decisions.iter().cloned());
        }
        let authority_text = authority
            .as_ref()
            .map(|decisions| serde_yaml_ng::to_string(decisions).unwrap());
        let decision = decide(ir, &case, &fixture.evidence(id), authority_text.as_deref());
        let found = statuses(&decision);
        let expect = &state["expect"];
        for (index, (noun, key)) in [
            ("claim", "claims"),
            ("obligation", "obligations"),
            ("action", "actions"),
        ]
        .into_iter()
        .enumerate()
        {
            let Some(wanted) = expect.get(key) else {
                continue;
            };
            let wanted = expected(wanted);
            if wanted != found[index] {
                differences.push(format!(
                    "state `{id}`: {noun}s {:?}, expected {wanted:?}",
                    found[index]
                ));
            }
        }
    }
    differences
}

fn evidence(id: &str, kind: &str, result: &str, revision: &str) -> EvidenceRecord {
    let value: Value = serde_yaml_ng::from_str(&format!(
        "{{format: canon-evidence/1, id: {id}, kind: {kind}, result: {result}, subject: service, \
         subject_revision: {revision}}}"
    ))
    .unwrap();
    eval::evidence_from_value(&value).unwrap_or_else(|refusal| panic!("{refusal}"))
}

fn case(service: &str) -> Value {
    serde_yaml_ng::from_str(&format!(
        "{{format: canon-case/1, id: INC-492, protocol: incident.response, artifacts: \
         {{service: {{revision: {service}}}, release: {{revision: r42}}}}}}"
    ))
    .unwrap()
}

fn action(decision: &Decision, id: &str) -> String {
    statuses(decision)[2][id].clone()
}

fn obligation(decision: &Decision, id: &str) -> String {
    statuses(decision)[1][id].clone()
}

/// The protocol as shipped passes the replayed checks: the replay is faithful.
fn real_protocol_passes() -> Ir {
    let real = compile(&read(PROTOCOL));
    assert!(independence_check_passes(&real));
    assert_eq!(fixture_differences(&real), Vec::<String>::new());
    real
}

const LEAVE: &str = "    precondition:
      all:
        - claim: service.healthy
          is: true
        - claim: impact.bounded
          is: true
";

/// `emergency.leave` admissible on restoration OR on an identified cause.
const LEAVE_ON_CAUSE: &str = "    precondition:
      any:
        - all:
            - claim: service.healthy
              is: true
            - claim: impact.bounded
              is: true
        - evidence:
            kind: cause_analysis
            result: identified
";

const DISCHARGE: &str = "    discharged_when:
      claim: service.healthy
      is: true
";

/// `restore_service` discharged while the service's health is TRUE or not known.
const DISCHARGE_ON_UNKNOWN: &str = "    discharged_when:
      any:
        - claim: service.healthy
          is: true
        - claim: service.healthy
          is: unknown
";

/// Story Outcome: "`emergency.leave` rests on restoration claims and never on
/// `cause.identified`". A protocol whose `emergency.leave` is admissible on a `cause_analysis`
/// with result `identified` while the service is unhealthy breaks that, and passes every check the
/// acceptance makes: `claim_references` does not see an evidence match, and no fixture state holds
/// a `cause_analysis`.
#[test]
fn acceptance_rejects_a_leave_that_rests_on_cause_analysis() {
    let real = real_protocol_passes();
    let wrong = mutant(LEAVE, LEAVE_ON_CAUSE);

    // The mutant is wrong: cause identified, service unhealthy, emergency left.
    let mut unhealthy_cause_known = support::Fixture::load(INC_492).unwrap().evidence("initial");
    unhealthy_cause_known.push(evidence("cause-1", "cause_analysis", "identified", "s1"));
    let shipped = decide(&real, &case("s1"), &unhealthy_cause_known, None);
    let mutated = decide(&wrong, &case("s1"), &unhealthy_cause_known, None);
    assert_eq!(action(&shipped, "emergency.leave"), "blocked");
    assert_eq!(action(&mutated, "emergency.leave"), "admissible");

    let independence = independence_check_passes(&wrong);
    let differences = fixture_differences(&wrong);
    assert!(
        !independence || !differences.is_empty(),
        "a protocol whose emergency.leave is admissible on cause analysis alone passes the \
         acceptance: independence check passes = {independence}, inc-492 differences = \
         {differences:?}"
    );
}

/// Option C makes a state reachable between the rollback and the first observation of the new
/// revision: the old observations are excluded and `service.healthy` is UNKNOWN. No state of
/// `inc-492` is that state, and the acceptance checks `restore_service`'s discharge only where the
/// service is known healthy or unhealthy, so a protocol that discharges it on UNKNOWN health
/// passes.
#[test]
fn acceptance_rejects_restore_service_discharged_on_unknown_health() {
    let real = real_protocol_passes();
    let wrong = mutant(DISCHARGE, DISCHARGE_ON_UNKNOWN);

    // The mutant is wrong: rolled back to s2, nothing observed of s2 yet, service health
    // unknown, and the urgent obligation is reported discharged.
    let rolled_back_unobserved = support::Fixture::load(INC_492)
        .unwrap()
        .evidence("rollback-approved");
    let shipped = decide(&real, &case("s2"), &rolled_back_unobserved, None);
    let mutated = decide(&wrong, &case("s2"), &rolled_back_unobserved, None);
    assert_eq!(statuses(&shipped)[0]["service.healthy"], "unknown");
    assert_eq!(obligation(&shipped, "restore_service"), "open");
    assert_eq!(obligation(&mutated, "restore_service"), "discharged");

    let independence = independence_check_passes(&wrong);
    let differences = fixture_differences(&wrong);
    assert!(
        !independence || !differences.is_empty(),
        "a protocol that discharges restore_service while the service's health is unknown passes \
         the acceptance: independence check passes = {independence}, inc-492 differences = \
         {differences:?}"
    );
}

/// The two fixture states that would catch both mutants, green against the shipped protocol and
/// red against each mutant: this is the shape of the fix (two states added to `inc-492`).
#[test]
fn discriminating_states_separate_the_shipped_protocol_from_both_mutants() {
    let real = compile(&read(PROTOCOL));
    let leave_on_cause = mutant(LEAVE, LEAVE_ON_CAUSE);
    let discharge_on_unknown = mutant(DISCHARGE, DISCHARGE_ON_UNKNOWN);
    let fixture = support::Fixture::load(INC_492).unwrap();

    let mut cause_known = fixture.evidence("initial");
    cause_known.push(evidence("cause-1", "cause_analysis", "identified", "s1"));
    let unobserved = fixture.evidence("rollback-approved");

    let shipped_cause = decide(&real, &case("s1"), &cause_known, None);
    assert_eq!(statuses(&shipped_cause)[0]["cause.identified"], "true");
    assert_eq!(action(&shipped_cause, "emergency.leave"), "blocked");
    let shipped_unobserved = decide(&real, &case("s2"), &unobserved, None);
    assert_eq!(obligation(&shipped_unobserved, "restore_service"), "open");
    assert_eq!(action(&shipped_unobserved, "emergency.leave"), "blocked");

    assert_eq!(
        action(
            &decide(&leave_on_cause, &case("s1"), &cause_known, None),
            "emergency.leave"
        ),
        "admissible"
    );
    assert_eq!(
        obligation(
            &decide(&discharge_on_unknown, &case("s2"), &unobserved, None),
            "restore_service"
        ),
        "discharged"
    );
}
