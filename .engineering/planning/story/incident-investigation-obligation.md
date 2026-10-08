---
format: aep.planning-md/3
id: story:incident-investigation-obligation
kind: story
status: active
title: incident.response/1 keeps the cause investigation open after emergency.leave
summary: Obligation investigate_cause, discharged by cause.identified, stays open after emergency mode ends; inc-492 shows it.
relations:
- decomposes: epic:els-first-domain
- depends_on: story:incident-response-protocol
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: cited
  path: crates/canon-engineering/tests/incident_response_protocol.rs
- confidence: cited
  path: docs/examples/incident-response.md
- confidence: cited
  path: fixtures/incident-response/inc-492.fixture.yaml
- confidence: cited
  path: protocols/incident-response/1.yaml
- confidence: cited
  path: protocols/vocabulary.yaml
- confidence: cited
  path: website/data/protocol-graphs/
- confidence: cited
  path: website/docs/protocols/incident-response/1.mdx
- confidence: cited
  path: website/docs/vocabulary.md
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-10-08T18:53:46Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-08T18:53:47Z", actor: "human:timo", revision: 4}
---
## Outcome

`incident.response/1` declares the investigation of an incident's cause as an obligation of its own,
`investigate_cause`, discharged when `cause.identified` is `TRUE`. Restoration and investigation are
two obligations that progress independently: `emergency.leave` stays admissible on restoration
evidence alone, and `investigate_cause` stays open after it, until a cause analysis of the current
service revision exists.

Today the protocol declares one obligation, `restore_service`
(`protocols/incident-response/1.yaml`, `obligations:`). Its description says the investigation
"progresses on its own", but nothing in the protocol carries it, so a consumer evaluating a case has
no open item telling it the investigation is still owed once emergency mode ends. Loom's
incident-investigation work needs that item from the protocol, not from its own code.

- `protocols/incident-response/1.yaml`: add obligation `investigate_cause`, discharged when
  `cause.identified` is `TRUE`. `emergency.leave`'s precondition is unchanged and still never rests
  on `cause.identified` or `cause_analysis`.
- `protocols/vocabulary.yaml`: add `investigate_cause`, category `obligation_id`.
- `fixtures/incident-response/inc-492.fixture.yaml`: every state expects `investigate_cause`, and
  a final state records a cause analysis of revision `s2`.
- The generated protocol page, graph and vocabulary page are regenerated with `task docs`.

## Scope

- `protocols/incident-response/1.yaml`
- `protocols/vocabulary.yaml`
- `fixtures/incident-response/inc-492.fixture.yaml`
- `crates/canon-engineering/tests/incident_response_protocol.rs`
- `website/docs/protocols/incident-response/1.mdx`, `website/data/protocol-graphs/`,
  `website/docs/vocabulary.md` (generated)
- `docs/examples/incident-response.md` (the worked example names the open investigation)

`CHANGELOG.md` is not edited by this unit; the release commit records the change.

## Shared surface

`story:rollback-verification-rule` and `story:stale-evidence-fixtures` (both draft) edit the same
protocol and fixture tree; they depend on this story and rebase onto it.

## Protocol first

The first commit changes the protocol YAML, the vocabulary entry, the fixture expectations and the
test assertions, and nothing else. The red test is
`incident_response_protocol::inc_492_investigation_stays_open_after_emergency_leave`: on a tree
where the protocol is unchanged it fails because the decision has no obligation
`investigate_cause`. The story has no Rust beyond the test; Canon already evaluates obligations.

## Acceptance

The test `inc_492_investigation_stays_open_after_emergency_leave` in
`crates/canon-engineering/tests/incident_response_protocol.rs` passes under `task check`, together
with the existing `inc_492_leaves_emergency_while_cause_unknown`, and expects on fixture `inc-492`:

1. `initial`: `investigate_cause` open, `restore_service` open.
2. `cause-identified` (cause analysis of `s1`): `investigate_cause` discharged, `restore_service`
   open, `emergency.leave` blocked.
3. `rolled-back`: the service is at `s2`, the analysis of `s1` is excluded as a revision mismatch,
   so `cause.identified` is `UNKNOWN` and `investigate_cause` is open again.
4. `service-restored`: `restore_service` discharged, `emergency.leave` admissible, and
   `investigate_cause` open.
5. A new final state `cause-identified-after-restore` adds a `cause_analysis` with result
   `identified`, subject `service`, revision `s2`: `investigate_cause` discharged,
   `restore_service` discharged, `emergency.leave` admissible.

The decision's obligation ids are exactly `investigate_cause` and `restore_service`, and
`vocabulary::lookup("investigate_cause")` returns category `ObligationId`.

## Source

Loom's incident-investigation story asks for this obligation; 0.1.0 and 0.2.1 declare none.
`docs/examples/incident-response.md:30` ("The operational incident can leave emergency mode while
the investigation remains open").
