---
format: aep.planning-md/3
id: story:incident-response-protocol
kind: story
status: draft
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
  path: crates/els/src/protocols/incident_response.rs
- confidence: cited
  path: crates/els/tests/incident_response_protocol.rs
- confidence: cited
  path: fixtures/incident-response/
revision: 4
---
## Outcome

`incident.response/1` exists as a Canon `protocol/1` source in `crates/els/src/protocols/incident_response.rs`, compiled and evaluated by the same Canon kernel as `software.change/1`. It declares, all named from the vocabulary `story:els-vocabulary` declares (this story adds no term to `crates/els/src/vocabulary.rs`; spelling as settled there, after Canon design § 18):

- the claims `impact.bounded`, `service.healthy` and `cause.identified`, on the evidence kinds `impact_assessment`, `operational_observation` (health) and `cause_analysis`;
- the urgent obligation `restore_service`, satisfied when `service.healthy = TRUE`;
- the read actions `metrics.inspect`, `logs.search` and `release.inspect`, and the authority-gated actions `traffic.shift` and `release.rollback`;
- the action `emergency.leave`, the declared step that leaves emergency mode, admissible when `restore_service` is satisfied and `impact.bounded = TRUE`.

This protocol exists to keep the kernel from becoming a delivery workflow. Operational restoration and causal investigation progress independently: `emergency.leave` rests on restoration claims and never on `cause.identified`. Canon design § 39.1 leaves open whether a scalar state is fundamental, so emergency mode is derived from claims and obligations, not held in one state field (Canon design § 18).

What stays out: the freshness horizon on health evidence, which `story:stale-evidence-fixtures` declares in this source with its fixture; and rollback verification (`rollback.verified` on `rollback_result` evidence, independent of the actor who rolled back), which `story:rollback-verification-rule` declares. Here a rollback is shown working by the health evidence that follows it, not by a `rollback_result`.

No hidden clock: the evaluation instant is a fixture input, loaded by the harness `crates/els/tests/support/mod.rs` that `story:fixture-harness` creates; this story does not edit the harness.

## Shared surface

`crates/els/src/protocols/incident_response.rs` and `fixtures/incident-response/` are edited after this story by `story:rollback-verification-rule` and then `story:stale-evidence-fixtures`; both depend on this story and the second depends on the first, so no two edit them at once. This story runs beside `story:software-change-protocol` (both depend only on `story:els-vocabulary` and `story:fixture-harness`), which edits different files (`crates/els/src/protocols/software_change.rs`, `fixtures/software-change/`); neither edits `crates/els/src/lib.rs`, `crates/els/src/protocols/mod.rs`, `crates/els/src/vocabulary.rs` or `crates/els/tests/support/mod.rs`, which `story:els-vocabulary` and `story:fixture-harness` own. Both depend on `story:fixture-harness`, which creates their empty source files and the harness their tests call.

## Canon capability

`protocol/1` (C-001), `canon-ir/1` (C-002), three-valued truth (C-003), evidence applicability (C-004), obligations (C-005), action admissibility with authority (C-006). Freshness (C-008) is used by `story:stale-evidence-fixtures`.

## Domain relations

- ELS protocol → Canon protocol model: many-to-one; Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses — inferable (inferred from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`; no ess/1 document declares it, because ELS opts out of ESS for protocol semantics, `AGENTS.md` § ESS).

## Acceptance

The test `inc_492_leaves_emergency_while_cause_unknown` in `crates/els/tests/incident_response_protocol.rs` passes under `task check`. Its fixture is `inc-492` in `fixtures/incident-response/`, transcribed from `docs/examples/incident-response.md`; it expects:

1. Starting from the source in `crates/els/src/protocols/incident_response.rs`, compiling it through the same Canon crate and evaluator `story:software-change-protocol` uses yields a `canon-ir/1` document and no error.
2. Starting from the fixture's initial state (an `impact_assessment` reporting impact bounded, one `operational_observation` reporting the service unhealthy, no `cause_analysis`, no authority decision), evaluation reports `impact.bounded = TRUE`, `service.healthy = FALSE`, `cause.identified = UNKNOWN`; `restore_service` open; `metrics.inspect`, `logs.search` and `release.inspect` admissible; `traffic.shift` and `release.rollback` approval-required; and `emergency.leave` blocked.
3. Starting from the state of 2 and adding an authority decision approving `release.rollback`, evaluation reports `release.rollback` admissible, `service.healthy = FALSE` and `emergency.leave` still blocked.
4. Starting from the state of 3 and adding an `operational_observation` reporting the service healthy, observed after the unhealthy one and before the evaluation instant, with still no `cause_analysis`, evaluation reports `service.healthy = TRUE`, `impact.bounded = TRUE`, `cause.identified = UNKNOWN`, `restore_service` satisfied and `emergency.leave` admissible.

## Source

TASKBOARD E-004; Atlas ADR 0068; Canon `docs/design/canon-protocol-calculus-design.md` § 18; `docs/examples/incident-response.md`; round-1 reviews `review-result:els-first-domain-acceptance-r1`, `review-result:els-first-domain-parallel-safety-r1`.
