---
format: aep.planning-md/3
id: story:ess-conformance-evidence
kind: story
status: draft
title: Admit ESS conformance reports as system-conformance evidence
summary: ess-conformance-report/2 bound to implementation and specification revisions through a closed evidence contract.
refs:
- provider: canon
  reference: taskboard:C-004
- provider: taskboard
  reference: E-007
relations:
- decomposes: epic:els-first-domain
- serves: vision:O2
- serves: vision:governed-autonomy
- depends_on: story:software-change-negative-outcomes
scope:
- confidence: cited
  path: crates/els/src/protocols/software_change.rs
- confidence: cited
  path: crates/els/src/vocabulary.rs
- confidence: cited
  path: crates/els/tests/ess_conformance_evidence.rs
- confidence: cited
  path: fixtures/software-change/
revision: 3
---
## Outcome

`software.change/1` admits an ESS conformance report as `system_conformance` evidence bound to two revisions — implementation and system specification. The binding is a closed evidence contract: ELS reads `ess-conformance-report/2` (`spec_digest`, `implementation`, `conformance_status`) into Canon evidence with its own types and takes no dependency on ESS crates or the ESS domain model (design § 19.1). `passed` establishes, `failed` contradicts, `inconclusive` leaves the claim undecided, and a report for a previous specification digest does not establish the current claim (design § 11.2). `ess-conformance-report/1` is not admitted: ESS never upconverts it to a qualifying report (ESS `docs/design/review-conformance-coverage.md` § New standalone report).

The claim `implementation.conforms` is a conjunct that extends `implementation.verified`, which `story:software-change-protocol` defines (design § 9): this story edits that definition in `crates/els/src/protocols/software_change.rs` so that, when the case input `affects_runtime_behavior` holds, `implementation.verified` also requires `implementation.conforms`. It is not a free-standing claim. The story adds the terms it introduces to `crates/els/src/vocabulary.rs`: the evidence kind `system_conformance`, the claim `implementation.conforms` and the case input `affects_runtime_behavior`.

This is a use of ESS output, not an ESS specification of ELS (`AGENTS.md` § ESS).

## Shared surface

`crates/els/src/protocols/software_change.rs`, `fixtures/software-change/` and `crates/els/src/vocabulary.rs` are edited by every story on the `software.change/1` chain, so this story is its fourth link: `story:software-change-protocol` → `story:software-change-profiles` → `story:software-change-negative-outcomes` → `story:ess-conformance-evidence` → `story:security-independence-rules` → `story:stale-evidence-fixtures`. It depends on `story:software-change-negative-outcomes` and runs before `story:security-independence-rules`, which edits the same `implementation.verified` definition.

## Canon capability

Evidence applicability and revision binding (C-004), with evidence bound to two subject revisions at once. If C-004 binds a single subject revision only, this story waits on Canon.

## Domain relations

- ELS protocol → Canon protocol model: many-to-one; Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses — inferable (inferred from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`; no ess/1 document declares it, because ELS opts out of ESS for protocol semantics, `AGENTS.md` § ESS).

## Acceptance

The test `implementation_conforms_follows_report_status_and_revisions` in `crates/els/tests/ess_conformance_evidence.rs` passes under `task check`. Its fixture is `conformance` in `fixtures/software-change/`, whose initial state is: implementation revision R2, system specification at digest D2, `affects_runtime_behavior = true`, a passing `test_result` bound to R2, and no conformance report. Each expectation from 2 to 7 starts from that initial state and adds one report; it expects:

1. Starting from the initial state, evaluation reports `implementation.conforms = UNKNOWN` and `implementation.verified = UNKNOWN`.
2. Adding an `ess-conformance-report/2` with `conformance_status` `passed`, `spec_digest` D2 and implementation R2, evaluation reports `implementation.conforms = TRUE` and `implementation.verified = TRUE`.
3. Adding the same report with `conformance_status` `failed`, evaluation reports `implementation.conforms = FALSE` and `implementation.verified = FALSE`.
4. Adding the same report with `conformance_status` `inconclusive`, evaluation reports `implementation.conforms = UNKNOWN` and `implementation.verified = UNKNOWN`.
5. Adding a `passed` report with `spec_digest` D1 (the previous specification) and implementation R2, evaluation reports `implementation.conforms = UNKNOWN`.
6. Adding a `passed` report with `spec_digest` D2 and implementation R1, evaluation reports `implementation.conforms = UNKNOWN`.
7. Adding an `ess-conformance-report/1` that says `passed` for D2 and R2, admission refuses it with a message naming `ess-conformance-report/1`, and evaluation reports `implementation.conforms = UNKNOWN`.
8. Starting from the initial state with `affects_runtime_behavior = false` and no report, evaluation reports `implementation.verified = TRUE`.
9. Starting from `crates/els/src/vocabulary.rs` as this story leaves it, `system_conformance`, `implementation.conforms` and `affects_runtime_behavior` each resolve to exactly one vocabulary entry.

## Source

TASKBOARD E-007; Atlas ADR 0068; `docs/design/engineering-lifecycle-specification-design.md` § 9, § 11.2, § 19; Canon `docs/design/canon-protocol-calculus-design.md` § 22; round-1 reviews `review-result:els-first-domain-design-r1`, `review-result:els-first-domain-parallel-safety-r1`.
