---
format: aep.planning-md/3
id: story:incident-response-subject-binding
kind: story
status: draft
title: Bind incident.response/1 evidence matches to their artifacts
relations:
- decomposes: epic:els-first-domain
- depends_on: story:software-change-protocol
- serves: vision:O2
- serves: vision:governed-autonomy
revision: 1
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

`inc_492_leaves_emergency_while_cause_unknown` passes with every evidence match bound, and a record
about the release moves no claim about the service.
