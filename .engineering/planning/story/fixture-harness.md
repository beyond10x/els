---
format: aep.planning-md/3
id: story:fixture-harness
kind: story
status: draft
title: Add the compile-and-evaluate fixture harness
summary: One harness loads a fixture, compiles a protocol through Canon and evaluates it; proven on a smoke fixture.
refs:
- provider: canon
  reference: taskboard:C-003
- provider: taskboard
  reference: E-001
relations:
- decomposes: epic:els-first-domain
- depends_on: story:els-vocabulary
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: cited
  path: crates/els/src/lib.rs
- confidence: cited
  path: crates/els/src/protocols/incident_response.rs
- confidence: cited
  path: crates/els/src/protocols/mod.rs
- confidence: cited
  path: crates/els/src/protocols/software_change.rs
- confidence: cited
  path: crates/els/tests/fixture_harness.rs
- confidence: cited
  path: crates/els/tests/support/mod.rs
- confidence: cited
  path: fixtures/smoke/
revision: 2
---
## Outcome

ELS has one compile-and-evaluate fixture harness, `crates/els/tests/support/mod.rs`, that every protocol test calls. It loads one fixture (case inputs, artifact revisions, evidence, authority and independence decisions, and the evaluation instant), compiles a protocol source through Canon to `canon-ir/1`, evaluates it with the Canon evaluator and returns Canon's evaluation result unchanged: claim truth values, and the action admissibility, open obligations and outcome that Canon adds to the same result as those capabilities land. ELS adds no evaluator of its own and no hidden clock, network or model call; the evaluation instant is a fixture input. The story decides and documents the fixture file format and the convention that a protocol's fixtures live under `fixtures/<protocol-name>/`; each protocol story creates its own directory.

Later stories call the harness without editing it; a story that needs a harness change records that as a scope change on itself before it starts.

### Layout this story owns

So that `story:software-change-protocol` and `story:incident-response-protocol`, which run at the same time, each edit only their own source file:

- `crates/els/src/protocols/mod.rs`, with `crates/els/src/protocols/software_change.rs` and `crates/els/src/protocols/incident_response.rs` created as empty modules;
- the `protocols` module declaration in `crates/els/src/lib.rs`, which `story:els-vocabulary` edits before it for the `vocabulary` declaration. The existing protocol-id functions in `crates/els/src/lib.rs` stay where they are.

## Shared surface

`crates/els/src/lib.rs` is edited by `story:els-vocabulary` and then by this story, which depends on it. `crates/els/tests/support/mod.rs`, `crates/els/src/protocols/mod.rs` and `fixtures/smoke/` are this story's alone. The empty `crates/els/src/protocols/software_change.rs` and `crates/els/src/protocols/incident_response.rs` are filled by `story:software-change-protocol` and `story:incident-response-protocol`, which both depend on this story.

## Canon capability

Compile and evaluate: `protocol/1` source (C-001), `canon-ir/1` (C-002) and three-valued claim evaluation (C-003). Canon delivers `canon evaluate` and its library function with its story `story:three-valued-claims` (Canon TASKBOARD C-003), which depends on its `story:canon-ir`. **This story cannot start until Canon's evaluate exists**: until then there is nothing for the harness to call, and its acceptance cannot run.

## Domain relations

- ELS protocol → Canon protocol model: many-to-one; Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses — inferable (inferred from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`; no ess/1 document declares it, because ELS opts out of ESS for protocol semantics, `AGENTS.md` § ESS).

## Acceptance

The test `harness_compiles_and_evaluates_smoke_fixture` in `crates/els/tests/fixture_harness.rs` passes under `task check`. Its fixture is `smoke` in `fixtures/smoke/`: a minimal `protocol/1` source declaring the one claim `service.healthy` on `operational_observation` evidence, both core terms from `story:els-vocabulary`, and a case with evaluation instant T0. Through the harness it expects:

1. Starting from the smoke protocol source, compiling it through Canon yields a `canon-ir/1` document and no error.
2. Starting from the fixture's initial state (no evidence), evaluation reports `service.healthy = UNKNOWN`, not `FALSE`.
3. Starting from the state of 2 and adding one `operational_observation` reporting the service healthy, observed before T0, evaluation reports `service.healthy = TRUE`.

## Source

TASKBOARD E-001 (Atlas `docs/design/governed-autonomy/TASKBOARD.md` § ELS); Canon TASKBOARD C-003; `docs/design/engineering-lifecycle-specification-design.md` § 9; round-2 review `review-result:els-first-domain-design-r2` (moved out of `story:els-vocabulary`).
