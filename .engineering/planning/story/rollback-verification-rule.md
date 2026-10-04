---
format: aep.planning-md/3
id: story:rollback-verification-rule
kind: story
status: draft
title: Add the independent rollback-verification rule to incident.response/1
summary: rollback.verified requires a complete rollback_result from a verifier independent of the actor who rolled back.
refs:
- provider: canon
  reference: taskboard:C-004
- provider: canon
  reference: taskboard:C-010
- provider: taskboard
  reference: E-008
relations:
- decomposes: epic:els-first-domain
- depends_on: story:incident-response-protocol
- depends_on: story:software-change-negative-outcomes
- depends_on: story:security-independence-rules
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: cited
  path: crates/els/src/protocols/incident_response.rs
- confidence: cited
  path: crates/els/src/vocabulary.rs
- confidence: cited
  path: crates/els/tests/rollback_verification_rule.rs
- confidence: cited
  path: fixtures/incident-response/
revision: 2
---
## Outcome

An independent rollback-verification rule in `incident.response/1` (`crates/els/src/protocols/incident_response.rs`): the claim `rollback.verified` holds on `rollback_result` evidence with verdict complete from a verifier independent of the actor who performed `release.rollback` (`different_principal`) (design § 39.9). `rollback_result` is the evidence kind `story:software-change-negative-outcomes` adds to the vocabulary; this story declares it in the incident source. `different_principal` is the independence dimension `story:security-independence-rules` adds.

ELS declares which independence dimensions a requirement needs. It does not resolve identities: the fixture supplies the independence decision as trusted input (design § 12, § 45.3; Canon design § 39.4). Canon owns the requirement form and the rule that evidence failing it does not satisfy (CANON-INDEPENDENCE-001).

The story adds the one term it introduces to `crates/els/src/vocabulary.rs`: the claim `rollback.verified`.

## Shared surface

`crates/els/src/protocols/incident_response.rs` and `fixtures/incident-response/` are edited by `story:incident-response-protocol` → `story:rollback-verification-rule` → `story:stale-evidence-fixtures`, so this story is the middle link of that chain and depends on `story:incident-response-protocol`. It depends on `story:software-change-negative-outcomes` because that story adds `rollback_result`, the evidence kind this rule rests on. It depends on `story:security-independence-rules` because both edit `crates/els/src/vocabulary.rs` (and `different_principal` arrives there with that story), so the two never edit the vocabulary at once. It edits neither `crates/els/src/protocols/software_change.rs` nor `fixtures/software-change/`.

## Canon capability

Evidence applicability (C-004) and an independence requirement form. No Canon TASKBOARD item C-001…C-011 names independence: CANON-INDEPENDENCE-001 appears only in the conformance list (C-010). If neither C-004 nor C-010 delivers the form, this story waits on Canon.

## Domain relations

- ELS protocol → Canon protocol model: many-to-one; Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses — inferable (inferred from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`; no ess/1 document declares it, because ELS opts out of ESS for protocol semantics, `AGENTS.md` § ESS).

## Acceptance

The test `rollback_verified_needs_an_independent_verifier` in `crates/els/tests/rollback_verification_rule.rs` passes under `task check`; it expects:

1. Starting from the fixture `rollback-verification` in `fixtures/incident-response/` (the INC-492 case with `release.rollback` approved and performed by actor A, no `rollback_result`), evaluation reports `rollback.verified = UNKNOWN`.
2. Starting from the state of 1 and adding `rollback_result` evidence with verdict complete from A, with an independence decision marking A not independent, evaluation reports `rollback.verified = UNKNOWN`.
3. Starting from the state of 1 and adding `rollback_result` evidence with verdict complete from verifier B, with an independence decision marking B independent on `different_principal`, evaluation reports `rollback.verified = TRUE`.
4. Starting from `crates/els/src/vocabulary.rs` as this story leaves it, `rollback.verified` resolves to exactly one vocabulary entry.

## Source

TASKBOARD E-008; `docs/design/engineering-lifecycle-specification-design.md` § 12, § 39.9; round-2 review `review-result:els-first-domain-design-r2` (split out of `story:security-independence-rules`).
