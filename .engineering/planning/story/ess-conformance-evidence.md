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
  path: crates/canon-engineering/tests/ess_conformance_evidence.rs
- confidence: cited
  path: fixtures/software-change/
- confidence: cited
  path: protocols/software-change/1.yaml
- confidence: cited
  path: protocols/vocabulary.yaml
revision: 8
---
## Outcome

`software.change/1` admits an ESS conformance report as `system_conformance` evidence bound to two revisions: the implementation and the system specification. The binding is a closed evidence contract. ELS reads `ess-conformance-report/2` (`spec_digest`, `implementation`, `conformance_status`) into Canon evidence with its own types, and takes no dependency on ESS crates or the ESS domain model (design § 19.1).

- `passed` establishes the claim.
- `failed` contradicts it.
- `inconclusive` leaves it undecided.
- A report for a previous specification digest does not establish the current claim (design § 11.2).

`ess-conformance-report/1` is not admitted, because ESS never upconverts it to a qualifying report (ESS `docs/design/review-conformance-coverage.md` § New standalone report).

The claim `implementation.conforms` is a conjunct that extends `implementation.verified`, which `story:software-change-protocol` defines (design § 9). This story edits that definition in `protocols/software-change/1.yaml` so that `implementation.verified` also requires `implementation.conforms` when the case input `affects_runtime_behavior` holds. It is not a free-standing claim. The story adds the terms it introduces to `protocols/vocabulary.yaml`: the evidence kind `system_conformance`, the claim `implementation.conforms` and the case input `affects_runtime_behavior`. All three categories are ones the typed reader already knows by then (the case-input category arrives with `story:software-change-profiles`), so this story does not edit `crates/canon-engineering/src/vocabulary.rs`.

Where the report-to-evidence mapping needs code rather than protocol data, that code is test or fixture code under `crates/canon-engineering/tests/` and `fixtures/`, not a module under `crates/canon-engineering/src/`.

This is a use of ESS output, not an ESS specification of ELS (`AGENTS.md` § ESS).

## Scope

- `protocols/software-change/1.yaml`
- `fixtures/software-change/`
- `protocols/vocabulary.yaml`
- `crates/canon-engineering/tests/ess_conformance_evidence.rs` (new)

## Shared surface

`protocols/software-change/1.yaml`, `fixtures/software-change/` and `protocols/vocabulary.yaml` are edited by every story on the `software.change/1` chain, and this story is its fourth link: `story:software-change-protocol` → `story:software-change-profiles` → `story:software-change-negative-outcomes` → `story:ess-conformance-evidence` → `story:security-independence-rules` → `story:stale-evidence-fixtures`. This story depends on `story:software-change-negative-outcomes`. It runs before `story:security-independence-rules`, which edits the same `implementation.verified` definition.

## Protocol first

Atlas ADR 0080 (draft). The first commit changes the following, and nothing else:

- `protocols/software-change/1.yaml`, adding `system_conformance` and `implementation.conforms` and the extended `implementation.verified`;
- the fixture `conformance` in `fixtures/software-change/`, with the expectations in § Acceptance;
- `crates/canon-engineering/tests/ess_conformance_evidence.rs`.

On that commit `implementation_conforms_follows_report_status_and_revisions` fails at item 9: `system_conformance`, `implementation.conforms` and `affects_runtime_behavior` do not yet resolve in the vocabulary. It also fails at item 7 if the report-admission check is not yet written. The implementation commit adds the terms to `protocols/vocabulary.yaml` and the admission code under `crates/canon-engineering/tests/`.

## Canon capability

Evidence applicability and revision binding (C-004), with evidence bound to two subject revisions at once. If C-004 binds only a single subject revision, this story waits on Canon. Case inputs wait on Canon too (see `story:software-change-profiles` § Canon capability).

## Domain relations

- ELS protocol → Canon protocol model: many-to-one. Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses. Inferable: from `crates/canon-engineering/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/canon-engineering/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`. No ess/1 document declares it, because ELS opts out of ESS for protocol semantics (`AGENTS.md` § ESS).

## Acceptance

The test `implementation_conforms_follows_report_status_and_revisions` in `crates/canon-engineering/tests/ess_conformance_evidence.rs` passes under `task check`. It loads `protocols/software-change/1.yaml` through Canon by way of the harness. Its fixture is `conformance` in `fixtures/software-change/`, whose initial state is:

- implementation revision R2;
- system specification at digest D2;
- `affects_runtime_behavior = true`;
- a passing `test_result` bound to R2;
- no conformance report.

Each of expectations 2 to 7 starts from that initial state and adds one report. It expects:

1. Starting from the initial state, evaluation reports `implementation.conforms = UNKNOWN` and `implementation.verified = UNKNOWN`.
2. After adding an `ess-conformance-report/2` with `conformance_status` `passed`, `spec_digest` D2 and implementation R2, evaluation reports `implementation.conforms = TRUE` and `implementation.verified = TRUE`.
3. After adding the same report with `conformance_status` `failed`, evaluation reports `implementation.conforms = FALSE` and `implementation.verified = FALSE`.
4. After adding the same report with `conformance_status` `inconclusive`, evaluation reports `implementation.conforms = UNKNOWN` and `implementation.verified = UNKNOWN`.
5. After adding a `passed` report with `spec_digest` D1 (the previous specification) and implementation R2, evaluation reports `implementation.conforms = UNKNOWN`.
6. After adding a `passed` report with `spec_digest` D2 and implementation R1, evaluation reports `implementation.conforms = UNKNOWN`.
7. After adding an `ess-conformance-report/1` that says `passed` for D2 and R2, admission refuses it with a message naming `ess-conformance-report/1`, and evaluation reports `implementation.conforms = UNKNOWN`.
8. Starting from the initial state with `affects_runtime_behavior = false` and no report, evaluation reports `implementation.verified = TRUE`.
9. Starting from `protocols/vocabulary.yaml` as this story leaves it, read through `crates/canon-engineering/src/vocabulary.rs`, each of `system_conformance`, `implementation.conforms` and `affects_runtime_behavior` resolves to exactly one vocabulary entry.

## Source

TASKBOARD E-007; Atlas ADR 0068; Atlas ADR 0077 point 3 (draft); `docs/design/engineering-lifecycle-specification-design.md` § 9, § 11.2, § 19; Canon `docs/design/canon-protocol-calculus-design.md` § 22; round-1 reviews `review-result:els-first-domain-design-r1`, `review-result:els-first-domain-parallel-safety-r1`.
