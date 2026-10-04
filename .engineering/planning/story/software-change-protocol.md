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
- depends_on: story:fixture-harness
scope:
- confidence: cited
  path: crates/els/src/protocols/software_change.rs
- confidence: cited
  path: crates/els/tests/software_change_protocol.rs
- confidence: cited
  path: fixtures/software-change/
revision: 4
---
## Outcome

`software.change/1` exists as a Canon `protocol/1` source in `crates/els/src/protocols/software_change.rs`, is compiled by Canon to `canon-ir/1` and is evaluated by the Canon evaluator. It declares, all named from the vocabulary `story:els-vocabulary` declares (this story adds no term to `crates/els/src/vocabulary.rs`):

- the delivery artifacts `intent`, `system_specification`, `plan`, `implementation`, `release`, `deployment`;
- the evidence kinds `test_result` (bound to the implementation revision), `code_review`, `operational_observation`, `build_provenance` and `objective_observation`;
- the claims `tests.pass`, `implementation.reviewed`, `implementation.verified`, `release.proven`, `deployment.healthy` and `objective.realized`;
- the actions `repository.inspect`, `repository.edit`, `tests.run` and `repository.merge`, with the authority requirement on `repository.merge`;
- the `accepted` outcome.

### `implementation.verified` is defined here and extended later

This story defines `implementation.verified` with its first conjunct only: a passing `test_result` bound to the current implementation revision (design § 9). Two later stories extend this definition by editing it in this source, each adding one conjunct: `story:ess-conformance-evidence` adds `implementation.conforms` when `affects_runtime_behavior` holds, and `story:security-independence-rules` adds `security.reviewed` when `affects_security_boundary` holds. Neither adds a free-standing claim.

### What stays out

Risk profiles (E-003, `story:software-change-profiles`); negative outcomes, the `release.rollback` action on this protocol and `rollback_result` (E-006, `story:software-change-negative-outcomes`); ESS conformance evidence (E-007); security review and independence (E-008, `story:security-independence-rules`); and the staleness rules for `code_review` (binding to the implementation revision) and `operational_observation` (freshness horizon), which `story:stale-evidence-fixtures` declares in this source together with their fixtures, so no staleness rule lands without one (`AGENTS.md` § Rules). The R1/R2 `test_result` case stays here.

ELS adds no evaluator of its own and no hidden clock, network or model call: case, evidence, authority decisions and evaluation instant are fixture inputs, loaded by the harness `crates/els/tests/support/mod.rs` that `story:fixture-harness` creates; this story does not edit the harness.

`docs/design/engineering-lifecycle-specification-design.md` predates the Canon split: its `els/1` format (§ 9) and `els-parse` / `els-eval` crates (§ 29–§ 30) are now Canon's. Express the § 9 lifecycle in Canon `protocol/1`; do not build those crates here.

## Shared surface

`crates/els/src/protocols/software_change.rs` and `fixtures/software-change/` are one source edited by six stories, so they form one `depends_on` chain, of which this story is the head: `story:software-change-protocol` → `story:software-change-profiles` → `story:software-change-negative-outcomes` → `story:ess-conformance-evidence` → `story:security-independence-rules` → `story:stale-evidence-fixtures`. This story runs beside `story:incident-response-protocol`, which edits different files (`crates/els/src/protocols/incident_response.rs`, `fixtures/incident-response/`); neither edits `crates/els/src/lib.rs`, `crates/els/src/protocols/mod.rs`, `crates/els/src/vocabulary.rs` or `crates/els/tests/support/mod.rs`, which `story:els-vocabulary` and `story:fixture-harness` own. Both depend on `story:fixture-harness`, which creates their empty source files and the harness their tests call.

## Canon capability

`protocol/1` source (C-001), `canon-ir/1` (C-002), three-valued truth (C-003), evidence applicability and revision binding (C-004), action admissibility with authority (C-006).

## Domain relations

- ELS protocol → Canon protocol model: many-to-one; Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses — inferable (inferred from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`; no ess/1 document declares it, because ELS opts out of ESS for protocol semantics, `AGENTS.md` § ESS).

## Acceptance

The test `chg_1842_merge_waits_for_current_revision_tests_and_authority` in `crates/els/tests/software_change_protocol.rs` passes under `task check`. Its fixture is `chg-1842` in `fixtures/software-change/`, transcribed from `docs/examples/software-change.md`; it expects:

1. Starting from the source in `crates/els/src/protocols/software_change.rs`, compiling it through Canon yields a `canon-ir/1` document and no error.
2. Starting from the fixture's initial state (implementation revision R2, one passing `test_result` bound to R1, no authority decision), evaluation reports `tests.pass = UNKNOWN` (not `FALSE`); `repository.inspect`, `repository.edit` and `tests.run` admissible; and `repository.merge` blocked.
3. Starting from the state of 2 and adding a passing `test_result` bound to R2, evaluation reports `tests.pass = TRUE`, `implementation.verified = TRUE` and `repository.merge` approval-required.
4. Starting from the state of 3 and adding an authority decision approving `repository.merge`, evaluation reports `repository.merge` admissible.

## Source

TASKBOARD E-002; Atlas ADR 0068; `docs/design/engineering-lifecycle-specification-design.md` § 9, § 26; `docs/examples/software-change.md`; round-1 reviews `review-result:els-first-domain-design-r1`, `review-result:els-first-domain-parallel-safety-r1`.
