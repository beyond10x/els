---
format: aep.planning-md/3
id: story:incident-response-protocol
kind: story
status: implemented
title: Define incident.response/1 on Canon
summary: Incident protocol on the same kernel; leaves emergency mode while the cause is still UNKNOWN.
refs:
- provider: canon
  reference: taskboard:C-001
- provider: canon
  reference: taskboard:C-002
- provider: canon
  reference: taskboard:C-003
- provider: canon
  reference: taskboard:C-004
- provider: canon
  reference: taskboard:C-005
- provider: canon
  reference: taskboard:C-006
- provider: canon
  reference: taskboard:C-008
- provider: taskboard
  reference: E-004
relations:
- decomposes: epic:els-first-domain
- depends_on: story:els-vocabulary
- serves: vision:O2
- serves: vision:governed-autonomy
- depends_on: story:fixture-harness
scope:
- confidence: cited
  path: crates/els-docs/
- confidence: cited
  path: crates/els/tests/incident_response_protocol.rs
- confidence: cited
  path: crates/els/tests/support/mod.rs
- confidence: cited
  path: fixtures/incident-response/
- confidence: cited
  path: protocols/incident-response/1.yaml
- confidence: cited
  path: website/
revision: 16
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T06:15:14Z", actor: "human:timo", revision: 9, decided_on: {"recorded":{"review_outcome":2}}}
- {from: "proposed", to: "active", at: "2026-10-04T06:15:14Z", actor: "human:timo", revision: 10, decided_on: {"recorded":{"review_outcome":2}}}
- {from: "active", to: "implemented", at: "2026-10-04T07:04:55Z", actor: "human:timo", revision: 16, decided_on: {"recorded":{"test_result":1,"review_outcome":4,"verification":1}}}
---
## Outcome

`incident.response/1` exists as a Canon `protocol/1` YAML document at `protocols/incident-response/1.yaml` (Atlas ADR 0077 point 3, draft: ELS protocols are data). The same Canon kernel that handles `software.change/1` validates, compiles and evaluates it. It declares the following, all named from the vocabulary `story:els-vocabulary` declares, spelled as settled there after Canon design § 18. This story adds no term to `protocols/vocabulary.yaml`.

- The claims `impact.bounded`, `service.healthy` and `cause.identified`, on the evidence kinds `impact_assessment`, `operational_observation` (health) and `cause_analysis`.
- The urgent obligation `restore_service`, satisfied when `service.healthy = TRUE`.
- The read actions `metrics.inspect`, `logs.search` and `release.inspect`.
- The authority-gated actions `traffic.shift` and `release.rollback`.
- The action `emergency.leave`, the declared step that leaves emergency mode. It is admissible when `restore_service` is satisfied and `impact.bounded = TRUE`.

This protocol exists to keep the kernel from becoming a delivery workflow. Operational restoration and causal investigation progress independently: `emergency.leave` rests on restoration claims and never on `cause.identified`. Canon design § 39.1 leaves open whether a scalar state is fundamental, so emergency mode is derived from claims and obligations rather than held in one state field (Canon design § 18).

What stays out:

- The freshness horizon on health evidence. `story:stale-evidence-fixtures` declares it in this document with its fixture.
- Rollback verification (`rollback.verified` on `rollback_result` evidence, independent of the actor who rolled back). `story:rollback-verification-rule` declares it. Here a rollback is shown working by the health evidence that follows it, not by a `rollback_result`.

No hidden clock: the evaluation instant is a fixture input. It is loaded by the harness `crates/els/tests/support/mod.rs`, which `story:fixture-harness` creates; this story does not edit the harness. Rust in this story is test code only.

## Scope

- `protocols/incident-response/1.yaml` (new)
- `crates/els/tests/incident_response_protocol.rs` (new)
- `fixtures/incident-response/` (new)

## Shared surface

After this story, `protocols/incident-response/1.yaml` and `fixtures/incident-response/` are edited by `story:rollback-verification-rule` and then `story:stale-evidence-fixtures`. Both depend on this story and the second depends on the first, so no two edit them at once.

This story runs beside `story:software-change-protocol`; both depend on `story:els-vocabulary` and `story:fixture-harness`. That story creates different files (`protocols/software-change/1.yaml`, `fixtures/software-change/`). Neither story edits `protocols/vocabulary.yaml`, `crates/els/src/` or `crates/els/tests/support/mod.rs`. `story:protocol-registry` depends on both.

## Protocol first

Atlas ADR 0080 (draft). The first commit adds `protocols/incident-response/1.yaml`, the fixture `inc-492` in `fixtures/incident-response/` with the expectations in § Acceptance, and `crates/els/tests/incident_response_protocol.rs`. The red test is `inc_492_leaves_emergency_while_cause_unknown`. **This story has no ELS implementation beyond that data.** It adds no vocabulary term and no Rust outside the test, so the test fails on the first commit only if Canon (obligations C-005, admissibility C-006) or the harness does not yet evaluate what the YAML declares. If the test passes on the first commit, the story records that run as its baseline and says no implementation commit follows. ADR 0080 does not yet say how it applies to a story whose whole change is protocol data; that question is open.

## Canon capability

`protocol/1` (C-001), `canon-ir/1` (C-002), three-valued truth (C-003), evidence applicability (C-004), obligations (C-005), and action admissibility with authority (C-006). `story:stale-evidence-fixtures` uses freshness (C-008).

## Domain relations

- ELS protocol → Canon protocol model: many-to-one. Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses. Inferable: from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`. No ess/1 document declares it, because ELS opts out of ESS for protocol semantics (`AGENTS.md` § ESS).

## Acceptance

The test `inc_492_leaves_emergency_while_cause_unknown` in `crates/els/tests/incident_response_protocol.rs` passes under `task check`. It loads `protocols/incident-response/1.yaml` through Canon by way of the harness. Its fixture is `inc-492` in `fixtures/incident-response/`, transcribed from `docs/examples/incident-response.md`. It expects:

1. Starting from `protocols/incident-response/1.yaml`, validating it through Canon reports no error, and compiling it through the same Canon crate and evaluator that `story:software-change-protocol` uses yields a `canon-ir/1` document and no error.
2. Starting from the fixture's initial state (an `impact_assessment` reporting impact bounded, one `operational_observation` reporting the service unhealthy, no `cause_analysis`, no authority decision), evaluation reports:
   - `impact.bounded = TRUE`, `service.healthy = FALSE` and `cause.identified = UNKNOWN`;
   - `restore_service` open;
   - `metrics.inspect`, `logs.search` and `release.inspect` admissible;
   - `traffic.shift` and `release.rollback` approval-required;
   - `emergency.leave` blocked.
3. Starting from the state of 2 and adding an authority decision approving `release.rollback`, evaluation reports `release.rollback` admissible, `service.healthy = FALSE` and `emergency.leave` still blocked.
4. Starting from the state of 3 and adding an `operational_observation` reporting the service healthy (observed after the unhealthy one and before the evaluation instant, still with no `cause_analysis`), evaluation reports:
   - `service.healthy = TRUE`, `impact.bounded = TRUE` and `cause.identified = UNKNOWN`;
   - `restore_service` satisfied;
   - `emergency.leave` admissible.

## Source

TASKBOARD E-004; Atlas ADR 0068; Atlas ADR 0077 point 3 (draft); Canon `docs/design/canon-protocol-calculus-design.md` § 18; `docs/examples/incident-response.md`; round-1 reviews `review-result:els-first-domain-acceptance-r1`, `review-result:els-first-domain-parallel-safety-r1`.


## From wave 2026-10-04-w6 (fixture-harness, adversary pass 1, F8)

`els-fixture/1` as built expresses only claim expectations, refuses `authority:` keys, and never
passes `observed_at` to Canon. This story's acceptance items on actions, obligations, authority and
newer-observation-wins need the harness extended, so `crates/els/tests/support/mod.rs` is in its
scope, and it needs the Canon capabilities behind them (canon story:obligations, story:action-admissibility,
story:outcomes, story:evidence-freshness) on canon `main` before it starts.

## Coordinator decisions (wave 2026-10-04-w8)

- Canon is pinned at 8fc260a (wave 7: obligations, action admissibility with `--authority`,
  outcomes, revision binding, freshness).
- Phase 1 showed Canon has no "newer observation wins" rule: two observations of one kind that
  disagree leave the claim UNKNOWN. Option C is taken: the rollback produces a new revision of the
  service; the healthy observation is bound to it and the unhealthy observation of the old revision
  is excluded by revision binding. The fixture carries per-state case artifact revisions.
- Acceptance item 4, reworded: starting from the state of 3, the rollback produces a new revision
  of the service, and a healthy `operational_observation` and an `impact_assessment` reporting
  impact bounded, both of that revision, arrive (still no `cause_analysis`). Evaluation reports
  `service.healthy = TRUE`, `impact.bounded = TRUE` and `cause.identified = UNKNOWN`;
  `restore_service` discharged; `emergency.leave` admissible. The unhealthy observation and the
  impact assessment of the old revision are each listed as excluded (`revision_mismatch`) under the
  claim they bear on. The impact is re-assessed because the assessment of the old revision is
  excluded with it.
- Canon spells obligation statuses `open`/`discharged`; `emergency.leave` requires the claims that
  discharge `restore_service` (Canon cannot reference an obligation from a precondition);
  capabilities `traffic.shift` and `release.rollback`; urgency only in the obligation description.

### Adversary pass 1 decisions (wave 2026-10-04-w8)

inc-492 now has five states, in this order: `initial` (unhealthy, impact bounded), `cause-identified`
(cause identified while still unhealthy; the emergency stays), `rollback-approved` (`release.rollback`
granted), `rolled-back` (service at s2, no s2 evidence: every claim UNKNOWN, `restore_service` open,
the three s1 records excluded) and `service-restored` (healthy and bounded on s2; `restore_service`
discharged, `emergency.leave` admissible).

- F1 (fixed): the independence check follows the precondition through every claim to the evidence
  kinds it reaches; an `emergency.leave` that rests on cause analysis fails it.
- F2 (fixed): the `rolled-back` state pins that unknown health keeps the obligation open.
- F3 (fixed): the protocol page shows what discharges each obligation; the graph reuses the
  `requires` edge kind, as the pinned docs-system schema has no `discharges` kind.
- Consequence: the cause analysis is bound to service s1, so after the rollback `cause.identified`
  returns to UNKNOWN; item 4's "no cause_analysis" means none of the new revision.

### Adversary pass 2 and independent review (wave 2026-10-04-w8)

- Pass 2 (all fixed): a record's own `observed_at` is refused unless it matches; graph edges keep
  `not` polarity; each action's effect is asserted; the index counts obligations; the harness docs say a
  grant cannot be revoked; two page wordings corrected. The effects adversary case was removed as
  vacuous: Canon never reads `effect`, and the acceptance now asserts it (mutant shown red).
- Independent review (all fixed): the protocol declared `release`, so an observation of the
  unchanged release discharged `restore_service`; the protocol now declares only `service`. Canon
  story:subject-bound-evidence-match is drafted for subject-bound matches. The harness refuses an
  unwritable authority entry; the library protocol id matches the shipped protocol.
