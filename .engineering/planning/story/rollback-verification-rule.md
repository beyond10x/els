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
  path: crates/canon-engineering/tests/rollback_verification_rule.rs
- confidence: cited
  path: fixtures/incident-response/
- confidence: cited
  path: protocols/incident-response/1.yaml
- confidence: cited
  path: protocols/vocabulary.yaml
revision: 7
---
## Outcome

`incident.response/1` gains an independent rollback-verification rule in `protocols/incident-response/1.yaml`. The claim `rollback.verified` holds on `rollback_result` evidence with verdict complete, from a verifier independent of the actor who performed `release.rollback` (`different_principal`) (design § 39.9). `rollback_result` is the evidence kind `story:software-change-negative-outcomes` adds to the vocabulary, and this story declares it in the incident document. `different_principal` is the independence dimension `story:security-independence-rules` adds.

ELS declares which independence dimensions a requirement needs; it does not resolve identities. The fixture supplies the independence decision as trusted input (design § 12, § 45.3; Canon design § 39.4). Canon owns the requirement form and the rule that evidence failing it does not satisfy (CANON-INDEPENDENCE-001).

The story adds the one term it introduces, the claim `rollback.verified`, to `protocols/vocabulary.yaml`. The claim category already exists in the typed reader, so this story does not edit `crates/canon-engineering/src/vocabulary.rs` (Atlas ADR 0077 point 3, draft).

## Scope

- `protocols/incident-response/1.yaml`
- `fixtures/incident-response/`
- `protocols/vocabulary.yaml`
- `crates/canon-engineering/tests/rollback_verification_rule.rs` (new)

## Shared surface

`protocols/incident-response/1.yaml` and `fixtures/incident-response/` are edited in the order `story:incident-response-protocol` → `story:rollback-verification-rule` → `story:stale-evidence-fixtures`. This story is the middle link and depends on `story:incident-response-protocol`.

It also depends on two stories on the `software.change/1` chain:

- `story:software-change-negative-outcomes`, which adds `rollback_result`, the evidence kind this rule rests on;
- `story:security-independence-rules`, which adds `different_principal` and the independence-dimension category. Both stories also edit `protocols/vocabulary.yaml`, so the dependency keeps them from editing it at once.

It edits neither `protocols/software-change/1.yaml` nor `fixtures/software-change/`.

## Protocol first

Atlas ADR 0080 (draft). The first commit changes the following, and nothing else:

- `protocols/incident-response/1.yaml`, adding `rollback_result`, `rollback.verified` and its independence requirement;
- the fixture `rollback-verification` in `fixtures/incident-response/`, with the expectations in § Acceptance;
- `crates/canon-engineering/tests/rollback_verification_rule.rs`.

On that commit `rollback_verified_needs_an_independent_verifier` fails at item 4: `rollback.verified` does not yet resolve in the vocabulary. The implementation commit adds the term to `protocols/vocabulary.yaml`.

## Canon capability

Evidence applicability (C-004) and an independence requirement form. No Canon TASKBOARD item C-001…C-011 names independence; CANON-INDEPENDENCE-001 appears only in the conformance list (C-010). If neither C-004 nor C-010 delivers the form, this story waits on Canon.

## Domain relations

- ELS protocol → Canon protocol model: many-to-one. Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses. Inferable: from `crates/canon-engineering/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/canon-engineering/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`. No ess/1 document declares it, because ELS opts out of ESS for protocol semantics (`AGENTS.md` § ESS).

## Acceptance

The test `rollback_verified_needs_an_independent_verifier` in `crates/canon-engineering/tests/rollback_verification_rule.rs` passes under `task check`. It loads `protocols/incident-response/1.yaml` through Canon by way of the harness. It expects:

1. Starting from the fixture `rollback-verification` in `fixtures/incident-response/` (the INC-492 case with `release.rollback` approved and performed by actor A, no `rollback_result`), evaluation reports `rollback.verified = UNKNOWN`.
2. Starting from the state of 1 and adding `rollback_result` evidence with verdict complete from A, with an independence decision marking A not independent, evaluation reports `rollback.verified = UNKNOWN`.
3. Starting from the state of 1 and adding `rollback_result` evidence with verdict complete from verifier B, with an independence decision marking B independent on `different_principal`, evaluation reports `rollback.verified = TRUE`.
4. Starting from `protocols/vocabulary.yaml` as this story leaves it, read through `crates/canon-engineering/src/vocabulary.rs`, `rollback.verified` resolves to exactly one vocabulary entry.

## Source

TASKBOARD E-008; Atlas ADR 0077 point 3 (draft); `docs/design/engineering-lifecycle-specification-design.md` § 12, § 39.9; round-2 review `review-result:els-first-domain-design-r2` (split out of `story:security-independence-rules`).
