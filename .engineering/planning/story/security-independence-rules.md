---
format: aep.planning-md/3
id: story:security-independence-rules
kind: story
status: draft
title: Add the independent security-review rule to software.change/1
summary: security.reviewed requires an approving security_review from a principal independent of the implementer when the change affects a security boundary.
refs:
- provider: canon
  reference: taskboard:C-004
- provider: canon
  reference: taskboard:C-010
- provider: taskboard
  reference: E-008
relations:
- decomposes: epic:els-first-domain
- serves: vision:O2
- serves: vision:governed-autonomy
- depends_on: story:ess-conformance-evidence
scope:
- confidence: inferred
  path: crates/els/src/vocabulary.rs
- confidence: cited
  path: crates/els/tests/security_independence_rules.rs
- confidence: cited
  path: fixtures/software-change/
- confidence: cited
  path: protocols/software-change/1.yaml
- confidence: cited
  path: protocols/vocabulary.yaml
revision: 9
---
## Outcome

`software.change/1` gains an independent security-review rule in `protocols/software-change/1.yaml`:

- The claim `security.reviewed` holds on an approving `security_review` that is bound to the current implementation revision and comes from a producer independent of the implementing principal (`different_principal`).
- A change with the case input `affects_security_boundary` derives a security-review obligation (design § 10, § 12).

`security.reviewed` is a conjunct that extends `implementation.verified`, which `story:software-change-protocol` defines (design § 9). This story edits that definition so that `implementation.verified` also requires `security.reviewed` when `affects_security_boundary` holds. It is not a free-standing claim.

ELS declares which independence dimensions a requirement needs; it does not resolve identities. The fixture supplies the independence decision as trusted input (design § 12, § 45.3; Canon design § 39.4). Canon owns the requirement form and the rule that evidence failing it does not satisfy (CANON-INDEPENDENCE-001).

The story adds the terms it introduces to `protocols/vocabulary.yaml`: the evidence kind `security_review`, the claim `security.reviewed`, the case input `affects_security_boundary`, and the independence dimensions `different_principal` and `different_agent_run`. The typed reader `crates/els/src/vocabulary.rs` has no category for an independence dimension, so this story adds that category and adds no term to the Rust (Atlas ADR 0077 point 3, draft).

What stays out: the rollback-verification rule in `incident.response/1`. `story:rollback-verification-rule` declares it, using the dimension `different_principal` added here.

## Scope

- `protocols/software-change/1.yaml`
- `fixtures/software-change/`
- `protocols/vocabulary.yaml`
- `crates/els/src/vocabulary.rs` (inferred: the reader's independence-dimension category)
- `crates/els/tests/security_independence_rules.rs` (new)

## Shared surface

`protocols/software-change/1.yaml`, `fixtures/software-change/` and `protocols/vocabulary.yaml` are edited by every story on the `software.change/1` chain, and this story is its fifth link: `story:software-change-protocol` → `story:software-change-profiles` → `story:software-change-negative-outcomes` → `story:ess-conformance-evidence` → `story:security-independence-rules` → `story:stale-evidence-fixtures`. It depends on `story:ess-conformance-evidence`, which edits the same `implementation.verified` definition before it. It edits neither `protocols/incident-response/1.yaml` nor `fixtures/incident-response/`.

`story:rollback-verification-rule` depends on this story for two reasons. It uses `different_principal` and the independence-dimension category this story adds, and both stories edit `protocols/vocabulary.yaml`. `story:stale-evidence-fixtures` depends on both.

## Protocol first

Atlas ADR 0080 (draft). The first commit changes the following, and nothing else:

- `protocols/software-change/1.yaml`, adding `security_review`, `security.reviewed`, the derived obligation and the extended `implementation.verified`;
- the fixture `security-boundary` in `fixtures/software-change/`, with the expectations in § Acceptance;
- `crates/els/tests/security_independence_rules.rs`.

On that commit `independent_security_review_gates_implementation_verified` fails at item 5. The five new terms do not yet resolve in the vocabulary, and the reader has no independence-dimension category. The implementation commit adds the terms to `protocols/vocabulary.yaml` and the category to `crates/els/src/vocabulary.rs`.

## Canon capability

Evidence applicability (C-004) and an independence requirement form. No Canon TASKBOARD item C-001…C-011 names independence; CANON-INDEPENDENCE-001 appears only in the conformance list (C-010). If neither C-004 nor C-010 delivers the form, this story waits on Canon. Case inputs wait on Canon too (see `story:software-change-profiles` § Canon capability).

## Domain relations

- ELS protocol → Canon protocol model: many-to-one. Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses. Inferable: from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`. No ess/1 document declares it, because ELS opts out of ESS for protocol semantics (`AGENTS.md` § ESS).

## Acceptance

The test `independent_security_review_gates_implementation_verified` in `crates/els/tests/security_independence_rules.rs` passes under `task check`. It loads `protocols/software-change/1.yaml` through Canon by way of the harness. It expects:

1. Starting from the fixture `security-boundary` in `fixtures/software-change/` (implementation revision R1 by principal P1, `affects_security_boundary = true`, a passing `test_result` bound to R1, no `security_review`), evaluation reports `security.reviewed = UNKNOWN` and `implementation.verified = UNKNOWN`.
2. Starting from the state of 1 and adding an approving `security_review` bound to R1 from P1, with an independence decision marking P1 not independent, evaluation reports `security.reviewed = UNKNOWN` and `implementation.verified = UNKNOWN`.
3. Starting from the state of 1 and adding an approving `security_review` bound to R1 from principal P2, with an independence decision marking P2 independent on `different_principal`, evaluation reports `security.reviewed = TRUE` and `implementation.verified = TRUE`.
4. Starting from the state of 1 with `affects_security_boundary = false`, evaluation reports `implementation.verified = TRUE` with no `security_review`.
5. Starting from `protocols/vocabulary.yaml` as this story leaves it, read through `crates/els/src/vocabulary.rs`, each of `security_review`, `security.reviewed`, `affects_security_boundary`, `different_principal` and `different_agent_run` resolves to exactly one vocabulary entry.

## Source

TASKBOARD E-008; Atlas ADR 0077 point 3 (draft); `docs/design/engineering-lifecycle-specification-design.md` § 9, § 10, § 12, § 39.5; round-1 reviews `review-result:els-first-domain-acceptance-r1`, `review-result:els-first-domain-design-r1`, `review-result:els-first-domain-parallel-safety-r1`; round-2 review `review-result:els-first-domain-design-r2` (split into this story and `story:rollback-verification-rule`).
