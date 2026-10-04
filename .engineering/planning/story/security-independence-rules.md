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
- confidence: cited
  path: crates/els/src/protocols/software_change.rs
- confidence: cited
  path: crates/els/src/vocabulary.rs
- confidence: cited
  path: crates/els/tests/security_independence_rules.rs
- confidence: cited
  path: fixtures/software-change/
revision: 6
---
## Outcome

An independent security-review rule in `software.change/1` (`crates/els/src/protocols/software_change.rs`): the claim `security.reviewed` holds on an approving `security_review` bound to the current implementation revision from a producer independent of the implementing principal (`different_principal`), and a change with the case input `affects_security_boundary` derives a security-review obligation (design § 10, § 12). `security.reviewed` is a conjunct that extends `implementation.verified`, which `story:software-change-protocol` defines (design § 9): this story edits that definition so that, when `affects_security_boundary` holds, `implementation.verified` also requires `security.reviewed`. It is not a free-standing claim.

ELS declares which independence dimensions a requirement needs. It does not resolve identities: the fixture supplies the independence decision as trusted input (design § 12, § 45.3; Canon design § 39.4). Canon owns the requirement form and the rule that evidence failing it does not satisfy (CANON-INDEPENDENCE-001).

The story adds the terms it introduces to `crates/els/src/vocabulary.rs`: the evidence kind `security_review`, the claim `security.reviewed`, the case input `affects_security_boundary`, and the independence dimensions `different_principal` and `different_agent_run`.

What stays out: the rollback-verification rule in `incident.response/1`, which `story:rollback-verification-rule` declares using the dimension `different_principal` added here.

## Shared surface

`crates/els/src/protocols/software_change.rs`, `fixtures/software-change/` and `crates/els/src/vocabulary.rs` are edited by every story on the `software.change/1` chain, so this story is its fifth link: `story:software-change-protocol` → `story:software-change-profiles` → `story:software-change-negative-outcomes` → `story:ess-conformance-evidence` → `story:security-independence-rules` → `story:stale-evidence-fixtures`. It depends on `story:ess-conformance-evidence`, which edits the same `implementation.verified` definition before it. It edits neither `crates/els/src/protocols/incident_response.rs` nor `fixtures/incident-response/`. `story:rollback-verification-rule` depends on this story because both edit `crates/els/src/vocabulary.rs`; `story:stale-evidence-fixtures` depends on both.

## Canon capability

Evidence applicability (C-004) and an independence requirement form. No Canon TASKBOARD item C-001…C-011 names independence: CANON-INDEPENDENCE-001 appears only in the conformance list (C-010). If neither C-004 nor C-010 delivers the form, this story waits on Canon.

## Domain relations

- ELS protocol → Canon protocol model: many-to-one; Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses — inferable (inferred from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`; no ess/1 document declares it, because ELS opts out of ESS for protocol semantics, `AGENTS.md` § ESS).

## Acceptance

The test `independent_security_review_gates_implementation_verified` in `crates/els/tests/security_independence_rules.rs` passes under `task check`; it expects:

1. Starting from the fixture `security-boundary` in `fixtures/software-change/` (implementation revision R1 by principal P1, `affects_security_boundary = true`, a passing `test_result` bound to R1, no `security_review`), evaluation reports `security.reviewed = UNKNOWN` and `implementation.verified = UNKNOWN`.
2. Starting from the state of 1 and adding an approving `security_review` bound to R1 from P1, with an independence decision marking P1 not independent, evaluation reports `security.reviewed = UNKNOWN` and `implementation.verified = UNKNOWN`.
3. Starting from the state of 1 and adding an approving `security_review` bound to R1 from principal P2, with an independence decision marking P2 independent on `different_principal`, evaluation reports `security.reviewed = TRUE` and `implementation.verified = TRUE`.
4. Starting from the state of 1 with `affects_security_boundary = false`, evaluation reports `implementation.verified = TRUE` with no `security_review`.
5. Starting from `crates/els/src/vocabulary.rs` as this story leaves it, `security_review`, `security.reviewed`, `affects_security_boundary`, `different_principal` and `different_agent_run` each resolve to exactly one vocabulary entry.

## Source

TASKBOARD E-008; `docs/design/engineering-lifecycle-specification-design.md` § 9, § 10, § 12, § 39.5; round-1 reviews `review-result:els-first-domain-acceptance-r1`, `review-result:els-first-domain-design-r1`, `review-result:els-first-domain-parallel-safety-r1`; round-2 review `review-result:els-first-domain-design-r2` (split into this story and `story:rollback-verification-rule`).
