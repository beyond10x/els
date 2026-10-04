---
format: aep.planning-md/3
id: story:incident-response-subject-binding
kind: story
status: implemented
title: Bind incident.response/1 evidence matches to their artifacts
relations:
- decomposes: epic:els-first-domain
- depends_on: story:software-change-protocol
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: cited
  path: crates/els/tests/adversary2_incident_harness.rs
- confidence: cited
  path: crates/els/tests/adversary2_incident_subject.rs
- confidence: cited
  path: crates/els/tests/adversary_incident_mutants.rs
- confidence: cited
  path: crates/els/tests/adversary_incident_subject.rs
- confidence: cited
  path: crates/els/tests/adversary_incident_website.rs
- confidence: cited
  path: crates/els/tests/incident_response_protocol.rs
- confidence: cited
  path: crates/els/tests/review_incident_subject.rs
- confidence: cited
  path: fixtures/incident-response/
- confidence: cited
  path: protocols/incident-response/1.yaml
- confidence: cited
  path: website/
revision: 7
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T08:50:29Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-04T08:50:29Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-04T09:15:36Z", actor: "human:timo", revision: 7, decided_on: {"recorded":{"test_result":1,"review_outcome":3,"verification":1}}}
---
## Outcome

`incident.response/1` binds each evidence match to the artifact the evidence is about, as
`software.change/1` does since story:software-change-protocol (wave 2026-10-04-w9): it declares
`release` again, `impact_assessment` and `operational_observation` are about `service`, and an
observation of the release no longer reaches `service.healthy`. `cause_analysis` stays unbound
unless the design names its subject (`release.inspect` produces it from a release).

## Found by

story:software-change-protocol F1 (wave 2026-10-04-w9). Tried against Canon 8d1599e: 12 incident
tests change. inc-492's case must give `release` a revision (otherwise Canon refuses every state
with `missing-artifact`), and `review_incident_subject` changes from expecting a refusal to checking
that `restore_service` stays open.

## Protocol first

The binding in `protocols/incident-response/1.yaml` and the fixture's case revisions; red: a fixture
state where an operational observation of the release would discharge `restore_service`.

## Acceptance

`inc_492_leaves_emergency_while_cause_unknown` passes with `impact_assessment` and
`operational_observation` bound to `service` (`cause_analysis` stays unbound: the design gives it no
subject), and a record about the release moves no claim about the service.

## Coordinator decisions (wave 2026-10-04-w12)

- Phase 1 is the fixture state `release-observed` and `release` declared again, red because unbound
  matches read the release records; the two `subject: service` lines are the phase-2 change (with
  them in phase 1 nothing is red on Canon 8d1599e).
