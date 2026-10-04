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
revision: 1
---
## Outcome

`incident.response/1` exists as a Canon `protocol/1` source, compiled and evaluated by the same Canon kernel as `software.change/1`. It declares the claims `customer_impact_bounded`, `service_healthy` and `cause_identified` (spelling as settled by E-001), the urgent obligation `restore_service`, read actions `metrics.inspect`, `logs.search` and `release.inspect`, authority-gated `traffic.shift` and `release.rollback`, health evidence with a freshness horizon, and a declared step that leaves emergency mode.

This protocol exists to keep the kernel from becoming a delivery workflow. Operational restoration and causal investigation progress independently: leaving emergency mode rests on restoration claims and never on `cause_identified`. Canon design §39.1 leaves open whether a scalar state is fundamental, so emergency mode is derived from claims and obligations, not held in one state field (Canon design §18).

No hidden clock: the evaluation instant is a fixture input, and the fixture runs as a Rust test under `task check`.

## Canon capability

`protocol/1` (C-001), `canon-ir/1` (C-002), three-valued truth (C-003), evidence applicability (C-004), obligations (C-005), action admissibility with authority (C-006), freshness (C-008).

## Domain relations

- ELS protocol → Canon protocol model: many-to-one; Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses — inferable (inferred from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`; no ess/1 document declares it, because ELS opts out of ESS for protocol semantics, `AGENTS.md` § ESS).

## Acceptance

Compiling `incident.response/1` with Canon and evaluating the INC-492 fixture from `docs/examples/incident-response.md` reports `traffic.shift` and `release.rollback` approval-required at the start, and after rollback-success and fresh health evidence reports `service_healthy = TRUE`, `customer_impact_bounded = TRUE`, `cause_identified = UNKNOWN`, `restore_service` satisfied and the protocol’s leave-emergency step admissible.

## Source

TASKBOARD E-004; Atlas ADR 0068; Canon `docs/design/canon-protocol-calculus-design.md` §18; `docs/examples/incident-response.md`.
