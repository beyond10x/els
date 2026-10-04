---
format: aep.planning-md/3
id: story:software-change-protocol
kind: story
status: draft
title: Define software.change/1 on Canon
summary: Delivery protocol compiled and evaluated by Canon, with revision-bound implementation evidence and merge authority.
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
  reference: taskboard:C-006
- provider: taskboard
  reference: E-002
relations:
- decomposes: epic:els-first-domain
- depends_on: story:els-vocabulary
- serves: vision:O2
- serves: vision:governed-autonomy
revision: 1
---
## Outcome

`software.change/1` exists as a Canon `protocol/1` source in this repository, is compiled by Canon to `canon-ir/1` and is evaluated by the Canon evaluator. It declares the delivery artifacts (intent, system specification, plan, implementation, release, deployment), revision-bound implementation evidence (`test_result`, `code_review`, `operational_observation` with a freshness horizon), the claims built on them (`tests.pass`, `implementation.reviewed`, `implementation.verified`, `release.proven`, `deployment.healthy`, `objective.realized`), the actions with their authority requirements (`repository.merge` among them) and the `accepted` outcome, all named from the E-001 vocabulary.

Out of this story: risk profiles (E-003), negative outcomes (E-006), ESS conformance evidence (E-007), independence (E-008). ELS adds no evaluator of its own and no hidden clock, network or model call: case, evidence, authority decisions and evaluation instant are fixture inputs, and the fixture runs as a Rust test under `task check`.

`docs/design/engineering-lifecycle-specification-design.md` predates the Canon split: its `els/1` format (§9) and `els-parse` / `els-eval` crates (§29–§30) are now Canon’s. Express the §9 lifecycle in Canon `protocol/1`; do not build those crates here.

## Canon capability

`protocol/1` source (C-001), `canon-ir/1` (C-002), three-valued truth (C-003), evidence applicability and revision binding (C-004), action admissibility with authority (C-006).

## Domain relations

- ELS protocol → Canon protocol model: many-to-one; Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses — inferable (inferred from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`; no ess/1 document declares it, because ELS opts out of ESS for protocol semantics, `AGENTS.md` § ESS).

## Acceptance

Compiling `software.change/1` with Canon and evaluating the CHG-1842 fixture from `docs/examples/software-change.md` reports `tests.pass = UNKNOWN` and `repository.merge` blocked while test evidence exists only for R1 and the implementation is R2, `tests.pass = TRUE` with `repository.merge` approval-required after R2 test evidence, and `repository.merge` admissible once an authority decision is supplied.

## Source

TASKBOARD E-002; Atlas ADR 0068; `docs/design/engineering-lifecycle-specification-design.md` §9, §26; `docs/examples/software-change.md`.
