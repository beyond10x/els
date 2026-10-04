---
format: aep.planning-md/3
id: story:fixture-harness
kind: story
status: implemented
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
- depends_on: story:vocabulary-yaml-source
scope:
- confidence: cited
  path: crates/els/tests/fixture_harness.rs
- confidence: cited
  path: crates/els/tests/support/mod.rs
- confidence: cited
  path: fixtures/smoke/
revision: 7
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T03:45:03Z", actor: "human:timo", revision: 5}
- {from: "proposed", to: "active", at: "2026-10-04T03:45:03Z", actor: "human:timo", revision: 6}
- {from: "active", to: "implemented", at: "2026-10-04T04:11:39Z", actor: "human:timo", revision: 7, decided_on: {"recorded":{"test_result":1,"review_outcome":3,"verification":1}}}
---
## Outcome

ELS has one compile-and-evaluate fixture harness, `crates/els/tests/support/mod.rs`, that every protocol test calls. ELS protocols are data (Atlas ADR 0077 point 3, draft): each is a Canon `protocol/1` YAML document at `protocols/<name>/<major>.yaml`. The harness takes such a document, or a fixture protocol under `fixtures/`, and:

- parses and validates it through Canon (`b10x_canon::model::parse` and `b10x_canon::validate::validate`, the checks `canon validate` runs);
- compiles it to `canon-ir/1` (`b10x_canon::ir::compile`, as `canon compile` does);
- loads one fixture (case inputs, artifact revisions, evidence, authority and independence decisions, and the evaluation instant);
- evaluates the compiled protocol with the Canon evaluator and returns Canon's evaluation result unchanged. That result holds claim truth values, and the action admissibility, open obligations and outcome that Canon adds to it as those capabilities land.

ELS adds no evaluator of its own and no hidden clock, network or model call. The evaluation instant is a fixture input. The harness is Rust test code under `crates/els/tests/`; it creates no module under `crates/els/src/` and no protocol file.

The story decides and documents the fixture file format, YAML read with the dependencies `story:vocabulary-yaml-source` adds to `crates/els/Cargo.toml`. It also documents two conventions. A built-in protocol lives at `protocols/<name>/<major>.yaml`, and each protocol story creates its own file. A protocol's fixtures live under `fixtures/<protocol-name>/`, and each protocol story creates its own directory.

Later stories call the harness without editing it. A story that needs a harness change records that as a scope change on itself before it starts.

## Scope

- `crates/els/tests/support/mod.rs` (new)
- `crates/els/tests/fixture_harness.rs` (new)
- `fixtures/smoke/` (new: the smoke protocol YAML and its fixture)

## Shared surface

`crates/els/tests/support/mod.rs`, `crates/els/tests/fixture_harness.rs` and `fixtures/smoke/` belong to this story alone. It edits no file under `crates/els/src/`, nothing under `protocols/`, and not `crates/els/Cargo.toml`. It depends on `story:vocabulary-yaml-source` because it parses fixtures with the YAML dependency that story adds to `crates/els/Cargo.toml`, and because the smoke fixture's terms resolve against the vocabulary that story moves to `protocols/vocabulary.yaml`. Each of `story:software-change-protocol` and `story:incident-response-protocol` creates its own new file (`protocols/software-change/1.yaml`, `protocols/incident-response/1.yaml`), so they need no skeleton from this story to run at the same time. Both depend on this story for the harness their tests call.

## Protocol first

Atlas ADR 0080 (draft). The first commit adds `fixtures/smoke/`: the smoke protocol YAML and its fixture with the expectations in § Acceptance. It also adds `crates/els/tests/fixture_harness.rs`. On that commit `harness_compiles_and_evaluates_smoke_fixture` fails: `crates/els/tests/support/mod.rs` does not exist yet, so the test does not build. The implementation commit adds the harness and does not change the smoke fixture.

## Canon capability

Validate and compile exist in Canon today: `protocol/1` source (C-001, canon `story:protocol-source-model`, implemented) and `canon-ir/1` (C-002, canon `story:canon-ir`, implemented). Three-valued claim evaluation (C-003) does not: canon `crates/canon/src/eval/mod.rs` reads "Not built yet." Canon delivers `canon evaluate` and its library function with canon `story:three-valued-claims` (Canon TASKBOARD C-003, proposed). **This story cannot start until Canon's evaluate exists.** Until then the harness has nothing to call for evaluation, and acceptance items 2 and 3 cannot run.

## Domain relations

- ELS protocol → Canon protocol model: many-to-one. Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses. Inferable: from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`. No ess/1 document declares it, because ELS opts out of ESS for protocol semantics (`AGENTS.md` § ESS).

## Acceptance

The test `harness_compiles_and_evaluates_smoke_fixture` in `crates/els/tests/fixture_harness.rs` passes under `task check`. Its fixture is `smoke` in `fixtures/smoke/`: a minimal `protocol/1` YAML document declaring the one claim `service.healthy` on `operational_observation` evidence (both core terms in the vocabulary), and a case with evaluation instant T0. Through the harness it expects:

1. Starting from the smoke protocol YAML, validating it through Canon reports no error, and compiling it yields a `canon-ir/1` document and no error.
2. Starting from the fixture's initial state (no evidence), evaluation reports `service.healthy = UNKNOWN`, not `FALSE`.
3. Starting from the state of 2 and adding one `operational_observation` reporting the service healthy, observed before T0, evaluation reports `service.healthy = TRUE`.

## Source

TASKBOARD E-001 (Atlas `docs/design/governed-autonomy/TASKBOARD.md` § ELS); Canon TASKBOARD C-003; Atlas ADR 0077 point 3 (draft, operator decision 2026-10-04: ELS protocols are data); `docs/design/engineering-lifecycle-specification-design.md` § 9; round-2 review `review-result:els-first-domain-design-r2` (moved out of `story:els-vocabulary`).
