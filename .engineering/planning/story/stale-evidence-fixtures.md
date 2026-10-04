---
format: aep.planning-md/3
id: story:stale-evidence-fixtures
kind: story
status: draft
title: Add stale-evidence fixtures across both protocols
summary: Revision-stale reviews and freshness-expired observations evaluate UNKNOWN, never FALSE, in delivery and incident fixtures.
refs:
- provider: canon
  reference: taskboard:C-004
- provider: canon
  reference: taskboard:C-008
- provider: taskboard
  reference: E-005
relations:
- decomposes: epic:els-first-domain
- depends_on: story:software-change-protocol
- depends_on: story:incident-response-protocol
- serves: vision:O2
- serves: vision:governed-autonomy
- depends_on: story:security-independence-rules
- depends_on: story:rollback-verification-rule
scope:
- confidence: cited
  path: crates/els/src/protocols/incident_response.rs
- confidence: cited
  path: crates/els/src/protocols/software_change.rs
- confidence: cited
  path: crates/els/tests/stale_evidence.rs
- confidence: cited
  path: fixtures/incident-response/
- confidence: cited
  path: fixtures/software-change/
revision: 4
---
## Outcome

This story owns the staleness rules for review, observation and health evidence in both protocols. It declares each rule in the protocol source and lands the fixture that exercises it in the same change, so no staleness rule lands without its fixture (`AGENTS.md` § Rules):

- in `crates/els/src/protocols/software_change.rs`: `code_review` evidence is bound to the implementation revision it reviewed, so `implementation.reviewed` holds only on a review of the current revision; and `operational_observation` evidence carries a 30-minute freshness horizon, so `deployment.healthy` holds only on a fresh observation (design § 9, § 11.3);
- in `crates/els/src/protocols/incident_response.rs`: `operational_observation` health evidence carries a 30-minute freshness horizon, so `service.healthy` holds only on a fresh observation. Design § 11.3 states 30 minutes for operational evidence and gives no separate incident horizon, so the incident horizon takes the same value.

Each stale case evaluates `UNKNOWN`, never `FALSE` (`AGENTS.md` § Rules). The R1/R2 `test_result` case stays in `story:software-change-protocol`; staleness of conformance evidence after a specification revision stays in `story:ess-conformance-evidence`. The story introduces no term, so it does not edit `crates/els/src/vocabulary.rs`.

The fixtures exercise Canon behaviour through ELS protocols; ELS does not implement applicability or freshness. The evaluation instant is passed explicitly.

## Shared surface

This story edits both protocol sources and both fixture directories, so it is the last link of both chains: on `crates/els/src/protocols/software_change.rs` and `fixtures/software-change/`, `story:software-change-protocol` → `story:software-change-profiles` → `story:software-change-negative-outcomes` → `story:ess-conformance-evidence` → `story:security-independence-rules` → `story:stale-evidence-fixtures`; on `crates/els/src/protocols/incident_response.rs` and `fixtures/incident-response/`, `story:incident-response-protocol` → `story:rollback-verification-rule` → `story:stale-evidence-fixtures`. It depends on `story:software-change-protocol`, `story:incident-response-protocol`, `story:security-independence-rules` and `story:rollback-verification-rule`.

## Canon capability

Evidence applicability and revision binding (C-004); invalidation and freshness (C-008).

## Domain relations

- ELS protocol → Canon protocol model: many-to-one; Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses — inferable (inferred from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`; no ess/1 document declares it, because ELS opts out of ESS for protocol semantics, `AGENTS.md` § ESS).

## Acceptance

The test `stale_evidence_is_unknown_and_fresh_evidence_holds` in `crates/els/tests/stale_evidence.rs` passes under `task check`; it expects:

1. Starting from the fixture `stale-review` in `fixtures/software-change/` (implementation revision R2, a passing `test_result` bound to R2, one approving `code_review` bound to R1), evaluation reports `implementation.reviewed = UNKNOWN`, not `FALSE`.
2. Starting from the same fixture with the approving `code_review` bound to R2 instead, evaluation reports `implementation.reviewed = TRUE`.
3. Starting from the fixture `stale-observation` in `fixtures/software-change/` (a `deployment` with one `operational_observation` reporting healthy at T0) evaluated at T0 + 31 minutes, evaluation reports `deployment.healthy = UNKNOWN`, not `FALSE`.
4. Starting from the same fixture evaluated at T0 + 29 minutes, evaluation reports `deployment.healthy = TRUE`.
5. Starting from the fixture `stale-health` in `fixtures/incident-response/` (the INC-492 case with `release.rollback` approved and one `operational_observation` reporting the service healthy at T0) evaluated at T0 + 31 minutes, evaluation reports `service.healthy = UNKNOWN`, not `FALSE`; `restore_service` open; and `emergency.leave` blocked.
6. Starting from the same fixture evaluated at T0 + 29 minutes, evaluation reports `service.healthy = TRUE`, `restore_service` satisfied and `emergency.leave` admissible.

## Source

TASKBOARD E-005; `docs/design/engineering-lifecycle-specification-design.md` § 9, § 11, § 39.7; `AGENTS.md` § Rules; round-1 reviews `review-result:els-first-domain-acceptance-r1`, `review-result:els-first-domain-design-r1`, `review-result:els-first-domain-parallel-safety-r1`.
