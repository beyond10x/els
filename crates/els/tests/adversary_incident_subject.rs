//! Adversary pass 1 on `story:incident-response-subject-binding`: `incident.response/1` binds its
//! `impact_assessment` and `operational_observation` matches to the service and declares the
//! release again, and `cause_analysis` stays unbound.
//!
//! The cases here read the shipped protocol, `inc-492` and the concept pages at run time. A
//! protocol mutant is compiled from text in memory and replayed with Canon directly, as
//! `adversary_incident_mutants.rs` does; no file in the repository is written.

#[allow(dead_code)] // uses part of the harness; fixture_harness.rs uses all of it
mod support;

use b10x_canon::eval::{self, Supplied};
use b10x_canon::ir::Ir;
use b10x_canon::model::{ClaimId, Decision, ExclusionReason, Truth};
use serde_yaml_ng::Value;

const PROTOCOL: &str = "protocols/incident-response/1.yaml";
const INC_492: &str = "fixtures/incident-response/inc-492.fixture.yaml";

/// The line that binds a match to the service, as the shipped protocol writes it.
const BINDING: &str = "        subject: service\n";

/// The two records about the release that `release-observed` adds, as the fixture writes them.
const RELEASE_RECORDS: &str = "    add_evidence:
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

fn read(path: &str) -> String {
    std::fs::read_to_string(support::repo_root().join(path))
        .unwrap_or_else(|error| panic!("{path}: {error}"))
}

/// `inc-492` with `from` replaced by `to` once; `from` must occur in it.
fn inc_492_with(from: &str, to: &str) -> String {
    let text = read(INC_492);
    assert!(text.contains(from), "inc-492 contains `{from}`");
    text.replacen(from, to, 1)
}

fn compile(text: &str) -> Ir {
    let protocol = b10x_canon::model::parse(text).unwrap_or_else(|error| panic!("{error}"));
    b10x_canon::validate::validate(&protocol).unwrap_or_else(|problems| panic!("{problems:?}"));
    b10x_canon::ir::compile(&protocol).unwrap_or_else(|problems| panic!("{problems:?}"))
}

fn shipped() -> Ir {
    compile(&read(PROTOCOL))
}

/// The shipped protocol with both `subject: service` lines removed: every match unbound.
fn unbound() -> Ir {
    let text = read(PROTOCOL);
    assert_eq!(
        text.matches(BINDING).count(),
        2,
        "the protocol binds two matches"
    );
    compile(&text.replace(BINDING, ""))
}

/// Canon's decision of every state of fixture `text` under `ir`, in state order: each state's
/// case snapshot with the revisions set so far, the evidence so far (read by the harness) and the
/// authority decisions so far.
fn replay(ir: &Ir, text: &str) -> Vec<(String, Decision)> {
    let fixture = support::Fixture::from_yaml(text).unwrap_or_else(|error| panic!("{error}"));
    let document: Value = serde_yaml_ng::from_str(text).expect("the fixture is YAML");
    let mut case = document["case"].clone();
    let mut authority: Option<Vec<Value>> = None;
    let mut decisions = Vec::new();
    for state in document["states"].as_sequence().expect("states") {
        let id = state["id"].as_str().expect("a state id");
        if let Some(revisions) = state.get("set_revisions").and_then(Value::as_mapping) {
            for (artifact, revision) in revisions {
                let slot = case
                    .get_mut("artifacts")
                    .and_then(|artifacts| artifacts.get_mut(artifact.as_str().unwrap()))
                    .and_then(|entry| entry.get_mut("revision"))
                    .expect("the case lists the artifact");
                *slot = revision.clone();
            }
        }
        if let Some(added) = state.get("add_authority").and_then(Value::as_sequence) {
            authority
                .get_or_insert_with(Vec::new)
                .extend(added.iter().cloned());
        }
        let authority_text = authority
            .as_ref()
            .map(|decisions| serde_yaml_ng::to_string(decisions).unwrap());
        let snapshot = eval::case_from_value(&case).unwrap_or_else(|refusal| panic!("{refusal}"));
        let supplied = Supplied {
            authority: authority_text.as_deref(),
            ..Supplied::default()
        };
        let decision = eval::evaluate_with(ir, &snapshot, &fixture.evidence(id), supplied)
            .unwrap_or_else(|refusal| panic!("state `{id}`: {refusal}"));
        decisions.push((id.to_owned(), decision));
    }
    decisions
}

fn state<'a>(decisions: &'a [(String, Decision)], id: &str) -> &'a Decision {
    &decisions
        .iter()
        .find(|(state, _)| state == id)
        .unwrap_or_else(|| panic!("no state `{id}`"))
        .1
}

fn claim(decision: &Decision, id: &str) -> Truth {
    decision
        .claims
        .get(&ClaimId::new(id))
        .expect("the claim")
        .value
}

fn excluded(decision: &Decision, id: &str) -> Vec<(String, ExclusionReason)> {
    decision
        .claims
        .get(&ClaimId::new(id))
        .expect("the claim")
        .excluded_evidence
        .iter()
        .map(|exclusion| (exclusion.evidence.as_str().to_owned(), exclusion.reason))
        .collect()
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
        .unwrap_or_else(|| panic!("no action `{id}`"))
        .to_owned()
}

/// What a decision says, without its explanation: claim values with their exclusions, and the
/// obligations, actions and outcomes sections.
fn verdict(decision: &Decision) -> String {
    let claims: Vec<String> = decision
        .claims
        .iter()
        .map(|(id, entry)| {
            format!(
                "{}={} excluded {:?}",
                id.as_str(),
                entry.value,
                entry.excluded_evidence
            )
        })
        .collect();
    format!(
        "claims {claims:?}\nobligations {:?}\nactions {:?}\noutcomes {:?}",
        decision.obligations, decision.actions, decision.outcomes
    )
}

/// A YAML value on one line.
fn flow(value: &Value) -> String {
    serde_yaml_ng::to_string(value)
        .expect("the value writes")
        .trim_end()
        .replace('\n', " ")
}

/// Contract drift: `website/docs/concepts/protocols-and-compositions.mdx` quotes
/// `protocols/incident-response/1.yaml` as an excerpt. An excerpt may leave out an entry's keys,
/// but each key it shows is quoted whole: a predicate without its `subject` is a different
/// predicate (it reads records about every declared artifact).
#[test]
fn the_concept_pages_quote_the_shipped_incident_protocol() {
    let protocol: Value = serde_yaml_ng::from_str(&read(PROTOCOL)).expect("the protocol is YAML");
    let title = "```yaml title=\"protocols/incident-response/1.yaml (excerpt)\"\n";
    let directory = support::repo_root().join("website/docs/concepts");
    let mut pages: Vec<_> = std::fs::read_dir(&directory)
        .expect("the concept pages")
        .map(|entry| entry.expect("an entry").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|ext| ext == "mdx" || ext == "md")
        })
        .collect();
    pages.sort();
    let mut excerpts = 0;
    let mut wrong = Vec::new();
    for page in pages {
        let text = std::fs::read_to_string(&page).expect("the page reads");
        let name = page.file_name().unwrap().to_string_lossy().into_owned();
        let mut rest = text.as_str();
        while let Some(start) = rest.find(title) {
            let body = &rest[start + title.len()..];
            let end = body.find("```").expect("the excerpt is closed");
            let excerpt: Value = serde_yaml_ng::from_str(&body[..end])
                .unwrap_or_else(|error| panic!("{name}: the excerpt is YAML: {error}"));
            excerpts += 1;
            for (section, entries) in excerpt.as_mapping().expect("sections") {
                let section = section.as_str().expect("a section name");
                for (entry, keys) in entries.as_mapping().expect("entries") {
                    let entry = entry.as_str().expect("an entry name");
                    let Some(original) = protocol.get(section).and_then(|s| s.get(entry)) else {
                        wrong.push(format!("{name}: {section}.{entry} is not in the protocol"));
                        continue;
                    };
                    for (key, value) in keys.as_mapping().expect("keys") {
                        let key = key.as_str().expect("a key");
                        let shipped = original.get(key);
                        if shipped != Some(value) {
                            wrong.push(format!(
                                "{name}: {section}.{entry}.{key} quotes {} but the protocol says {}",
                                flow(value),
                                shipped.map(flow).unwrap_or_else(|| "nothing".to_owned())
                            ));
                        }
                    }
                }
            }
            rest = &body[end + 3..];
        }
    }
    assert_eq!(excerpts, 2, "the concept pages quote the protocol twice");
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// `release-observed` is the state an unbound match leaves emergency mode in: with both matches
/// unbound, the release's records establish the service's health and bounded impact, discharge
/// `restore_service` and admit `emergency.leave`. The fixture's claim expectations cannot tell a
/// record about the release from a record about the service at a revision it no longer has (both
/// leave the claims UNKNOWN); this case can.
#[test]
fn release_observed_separates_the_bound_protocol_from_the_unbound() {
    let text = read(INC_492);
    let bound = replay(&shipped(), &text);
    let free = replay(&unbound(), &text);
    let (bound, free) = (
        state(&bound, "release-observed"),
        state(&free, "release-observed"),
    );

    assert_eq!(claim(free, "service.healthy"), Truth::True);
    assert_eq!(claim(free, "impact.bounded"), Truth::True);
    assert_eq!(obligation(free, "restore_service"), "discharged");
    assert_eq!(action(free, "emergency.leave"), "admissible");

    assert_eq!(claim(bound, "service.healthy"), Truth::Unknown);
    assert_eq!(claim(bound, "impact.bounded"), Truth::Unknown);
    assert_eq!(obligation(bound, "restore_service"), "open");
    assert_eq!(action(bound, "emergency.leave"), "blocked");
}

/// A record about the release neither establishes nor contradicts a claim about the service, in
/// any state: `inc-492` decides every state the same with the release's records removed, and
/// with their results turned against the service (unhealthy, unbounded) it still meets every
/// expectation, `service-restored` included.
#[test]
fn records_about_the_release_move_nothing_in_any_state() {
    let ir = shipped();
    let text = read(INC_492);
    let with = replay(&ir, &text);

    // The replay is the harness's evaluation.
    let fixture = support::Fixture::from_yaml(&text).expect("loads");
    let compiled = fixture.compile().expect("compiles");
    for (id, decision) in &with {
        let harness = fixture.evaluate(&compiled, id).expect("decides");
        assert_eq!(verdict(decision), verdict(&harness), "state `{id}`");
    }

    let without = replay(
        &ir,
        &inc_492_with(RELEASE_RECORDS, "    add_evidence: []\n"),
    );
    assert_eq!(with.len(), without.len());
    for ((id, with), (_, without)) in with.iter().zip(&without) {
        assert_eq!(verdict(with), verdict(without), "state `{id}`");
    }

    let against = RELEASE_RECORDS
        .replace("result: healthy", "result: unhealthy")
        .replace("result: bounded", "result: unbounded");
    let against =
        support::Fixture::from_yaml(&inc_492_with(RELEASE_RECORDS, &against)).expect("loads");
    assert_eq!(against.check(&compiled), Ok(()));
}

/// Coordinator decision: `cause_analysis` names no subject, since `release.inspect` produces one
/// from a release. So a cause analysis of the case's release establishes `cause.identified`; one
/// of another revision of the release is excluded. Neither touches the restoration or emergency
/// mode, which never rest on the cause.
#[test]
fn a_cause_analysis_of_the_release_decides_the_cause_and_nothing_else() {
    let anchor = "          id: release-impact-1
          kind: impact_assessment
          result: bounded
          subject: release
          subject_revision: r42
";
    let added = format!(
        "{anchor}      - observed_at: \"2026-10-04T11:37:00Z\"
        record:
          format: canon-evidence/1
          id: release-cause-1
          kind: cause_analysis
          result: identified
          subject: release
          subject_revision: r42
      - observed_at: \"2026-10-04T11:38:00Z\"
        record:
          format: canon-evidence/1
          id: release-cause-old
          kind: cause_analysis
          result: identified
          subject: release
          subject_revision: r41
"
    );
    let decisions = replay(&shipped(), &inc_492_with(anchor, &added));

    let observed = state(&decisions, "release-observed");
    assert_eq!(claim(observed, "cause.identified"), Truth::True);
    assert_eq!(
        excluded(observed, "cause.identified"),
        [
            ("cause-1".to_owned(), ExclusionReason::RevisionMismatch),
            (
                "release-cause-old".to_owned(),
                ExclusionReason::RevisionMismatch
            ),
        ]
    );
    assert_eq!(claim(observed, "service.healthy"), Truth::Unknown);
    assert_eq!(claim(observed, "impact.bounded"), Truth::Unknown);
    assert_eq!(obligation(observed, "restore_service"), "open");
    assert_eq!(action(observed, "emergency.leave"), "blocked");

    let restored = state(&decisions, "service-restored");
    assert_eq!(claim(restored, "cause.identified"), Truth::True);
    assert_eq!(obligation(restored, "restore_service"), "discharged");
    assert_eq!(action(restored, "emergency.leave"), "admissible");
}
