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
  path: crates/els/tests/software_change_protocol.rs
- confidence: cited
  path: fixtures/software-change/
- confidence: cited
  path: protocols/software-change/1.yaml
revision: 6
---
## Outcome

`software.change/1` exists as a Canon `protocol/1` YAML document at `protocols/software-change/1.yaml` (Atlas ADR 0077 point 3, draft: ELS protocols are data, released as versioned data, loaded and validated by Canon). Canon validates it, compiles it to `canon-ir/1` and evaluates it. It declares the following, all named from the vocabulary `story:els-vocabulary` declares. This story adds no term to `protocols/vocabulary.yaml`.

- the delivery artifacts `intent`, `system_specification`, `plan`, `implementation`, `release`, `deployment`;
- the evidence kinds `test_result` (bound to the implementation revision), `code_review`, `operational_observation`, `build_provenance` and `objective_observation`;
- the claims `tests.pass`, `implementation.reviewed`, `implementation.verified`, `release.proven`, `deployment.healthy` and `objective.realized`;
- the actions `repository.inspect`, `repository.edit`, `tests.run` and `repository.merge`, with the authority requirement on `repository.merge`;
- the `accepted` outcome.

### `implementation.verified` is defined here and extended later

This story defines `implementation.verified` with its first conjunct only: a passing `test_result` bound to the current implementation revision (design § 9). Two later stories each edit this definition in `protocols/software-change/1.yaml` to add one conjunct. `story:ess-conformance-evidence` adds `implementation.conforms` when `affects_runtime_behavior` holds. `story:security-independence-rules` adds `security.reviewed` when `affects_security_boundary` holds. Neither adds a free-standing claim.

### What stays out

- Risk profiles (E-003, `story:software-change-profiles`).
- Negative outcomes, the `release.rollback` action on this protocol and `rollback_result` (E-006, `story:software-change-negative-outcomes`).
- ESS conformance evidence (E-007, `story:ess-conformance-evidence`).
- Security review and independence (E-008, `story:security-independence-rules`).
- The staleness rules for `code_review` (binding to the implementation revision) and `operational_observation` (freshness horizon). `story:stale-evidence-fixtures` declares them in this document together with their fixtures, so no staleness rule lands without one (`AGENTS.md` § Rules).

The R1/R2 `test_result` case stays here.

ELS adds no evaluator of its own and no hidden clock, network or model call. The case, evidence, authority decisions and evaluation instant are fixture inputs. They are loaded by the harness `crates/els/tests/support/mod.rs`, which `story:fixture-harness` creates; this story does not edit the harness. Rust in this story is test code only (`crates/els/tests/`).

`docs/design/engineering-lifecycle-specification-design.md` predates the Canon split: its `els/1` format (§ 9) and its `els-parse` / `els-eval` crates (§ 29–§ 30) are now Canon's. Express the § 9 lifecycle in Canon `protocol/1` YAML, and do not build those crates here.

## Scope

- `protocols/software-change/1.yaml` (new)
- `crates/els/tests/software_change_protocol.rs` (new)
- `fixtures/software-change/` (new)

## Shared surface

`protocols/software-change/1.yaml` and `fixtures/software-change/` are one document and one fixture tree, edited by six stories. They therefore form one `depends_on` chain, and this story is its head: `story:software-change-protocol` → `story:software-change-profiles` → `story:software-change-negative-outcomes` → `story:ess-conformance-evidence` → `story:security-independence-rules` → `story:stale-evidence-fixtures`.

The chain cannot be split into one file per profile or rule, because Canon's `protocol/1` has no import or composition today. canon `crates/canon/src/model/mod.rs` reads one closed document (`deny_unknown_fields`, no import section), and protocol composition is an open question (Canon design § 39.5).

This story runs beside `story:incident-response-protocol`, which creates a different file (`protocols/incident-response/1.yaml`, `fixtures/incident-response/`). Neither edits `protocols/vocabulary.yaml`, `crates/els/src/` or `crates/els/tests/support/mod.rs`. Both depend on `story:fixture-harness` for the harness their tests call. `story:protocol-registry` depends on both.

## Protocol first

Atlas ADR 0080 (draft). The first commit adds `protocols/software-change/1.yaml`, the fixture `chg-1842` in `fixtures/software-change/` with the expectations in § Acceptance, and `crates/els/tests/software_change_protocol.rs`. The red test is `chg_1842_merge_waits_for_current_revision_tests_and_authority`. **This story has no ELS implementation beyond that data.** It adds no vocabulary term and no Rust outside the test, so the test fails on the first commit only if Canon or the harness does not yet evaluate what the YAML declares. If the test passes on the first commit, the story records that run as its baseline and says no implementation commit follows. ADR 0080 does not yet say how it applies to a story whose whole change is protocol data; that question is open.

## Canon capability

`protocol/1` source (C-001), `canon-ir/1` (C-002), three-valued truth (C-003), evidence applicability and revision binding (C-004), and action admissibility with authority (C-006).

## Domain relations

- ELS protocol → Canon protocol model: many-to-one. Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses. Inferable: from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`. No ess/1 document declares it, because ELS opts out of ESS for protocol semantics (`AGENTS.md` § ESS).

## Acceptance

The test `chg_1842_merge_waits_for_current_revision_tests_and_authority` in `crates/els/tests/software_change_protocol.rs` passes under `task check`. It loads `protocols/software-change/1.yaml` through Canon by way of the harness. Its fixture is `chg-1842` in `fixtures/software-change/`, transcribed from `docs/examples/software-change.md`. It expects:

1. Starting from `protocols/software-change/1.yaml`, validating it through Canon reports no error, and compiling it yields a `canon-ir/1` document and no error.
2. Starting from the fixture's initial state (implementation revision R2, one passing `test_result` bound to R1, no authority decision), evaluation reports:
   - `tests.pass = UNKNOWN` (not `FALSE`);
   - `repository.inspect`, `repository.edit` and `tests.run` admissible;
   - `repository.merge` blocked.
3. Starting from the state of 2 and adding a passing `test_result` bound to R2, evaluation reports `tests.pass = TRUE`, `implementation.verified = TRUE` and `repository.merge` approval-required.
4. Starting from the state of 3 and adding an authority decision approving `repository.merge`, evaluation reports `repository.merge` admissible.

## Source

TASKBOARD E-002; Atlas ADR 0068; Atlas ADR 0077 point 3 (draft); `docs/design/engineering-lifecycle-specification-design.md` § 9, § 26; `docs/examples/software-change.md`; round-1 reviews `review-result:els-first-domain-design-r1`, `review-result:els-first-domain-parallel-safety-r1`.
