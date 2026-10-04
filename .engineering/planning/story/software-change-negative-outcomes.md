---
format: aep.planning-md/3
id: story:software-change-negative-outcomes
kind: story
status: draft
title: Add declined, rolled-back and superseded outcomes to software.change/1
summary: Terminal outcomes other than accepted, each earned by its own decision or evidence.
refs:
- provider: canon
  reference: taskboard:C-007
- provider: taskboard
  reference: E-006
relations:
- decomposes: epic:els-first-domain
- serves: vision:O2
- serves: vision:governed-autonomy
- depends_on: story:software-change-profiles
scope:
- confidence: inferred
  path: crates/canon-engineering/src/vocabulary.rs
- confidence: cited
  path: crates/canon-engineering/tests/software_change_negative_outcomes.rs
- confidence: cited
  path: fixtures/software-change/
- confidence: cited
  path: protocols/software-change/1.yaml
- confidence: cited
  path: protocols/vocabulary.yaml
revision: 12
---
## Outcome

`software.change/1` declares terminal outcomes beyond `accepted`:

- `declined`: earned on an `explicitly_declined` decision, and reachable without any implementation artifact (design § 6.7, § 9 `completion.declined`, ELS-OUTCOME-001);
- `rolled_back`: earned only with `rollback_result` evidence whose verdict is complete. Invoking a rollback is not enough (design § 9 `completion.rolled_back`, § 15, ELS-RECOVERY-001);
- `superseded`: earned on an `explicitly_superseded` decision recorded on this case.

To make `rolled_back` reachable, this story also declares two things in `protocols/software-change/1.yaml`. One is the action `release.rollback` on `software.change/1` with an authority requirement; the term is already in the vocabulary from `story:els-vocabulary`. The other is the evidence kind `rollback_result` with subject `release`. The story adds the terms it introduces to `protocols/vocabulary.yaml`: the outcomes `declined`, `rolled_back` and `superseded`, the evidence kind `rollback_result`, and the decisions `explicitly_declined` and `explicitly_superseded`. The typed reader `crates/canon-engineering/src/vocabulary.rs` has no category for a decision, so this story adds that category to the reader and adds no term to the Rust (Atlas ADR 0077 point 3, draft). `story:rollback-verification-rule` later uses `rollback_result` in `incident.response/1`.

The relation between a change and the change that supersedes it is not modelled here. Design § 45.5 leaves change nesting open, so the decision is a fact on this case only.

Pressure test: no outcome assumes Git, and a declined change carries no repository artifact.

## Scope

- `protocols/software-change/1.yaml`
- `fixtures/software-change/`
- `protocols/vocabulary.yaml`
- `crates/canon-engineering/src/vocabulary.rs` (inferred: the reader's decision category)
- `crates/canon-engineering/tests/software_change_negative_outcomes.rs` (new)

## Shared surface

`protocols/software-change/1.yaml`, `fixtures/software-change/` and `protocols/vocabulary.yaml` are edited by every story on the `software.change/1` chain, and this story is its third link: `story:software-change-protocol` → `story:software-change-profiles` → `story:software-change-negative-outcomes` → `story:ess-conformance-evidence` → `story:security-independence-rules` → `story:stale-evidence-fixtures`. Canon has no composition that would let the outcomes live in their own file. This story depends on `story:software-change-profiles` and runs before `story:ess-conformance-evidence`. `story:rollback-verification-rule` depends on it for `rollback_result`.

## Protocol first

Atlas ADR 0080 (draft). The first commit changes the following, and nothing else:

- `protocols/software-change/1.yaml`, adding the three outcomes, `release.rollback` and `rollback_result`;
- the fixtures `declined`, `superseded` and `rolled-back` in `fixtures/software-change/`, with the expectations in § Acceptance;
- `crates/canon-engineering/tests/software_change_negative_outcomes.rs`.

On that commit `negative_outcomes_are_earned` fails at item 6. The six new terms do not yet resolve in the vocabulary, and the reader has no decision category. The implementation commit adds the terms to `protocols/vocabulary.yaml` and the category to `crates/canon-engineering/src/vocabulary.rs`.

## Canon capability

Outcomes and completion (C-007), and action admissibility with authority (C-006) for `release.rollback`.

## Domain relations

- ELS protocol → Canon protocol model: many-to-one. Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses. Inferable: from `crates/canon-engineering/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/canon-engineering/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`. No ess/1 document declares it, because ELS opts out of ESS for protocol semantics (`AGENTS.md` § ESS).

## Acceptance

The test `negative_outcomes_are_earned` in `crates/canon-engineering/tests/software_change_negative_outcomes.rs` passes under `task check`. It loads `protocols/software-change/1.yaml` through Canon by way of the harness. It expects:

1. Starting from the fixture `declined` in `fixtures/software-change/` (an `intent` artifact, no `implementation` artifact, no decision), evaluation reports no outcome. After an `explicitly_declined` decision is added, it reports the outcome `declined`, and the case still holds no `implementation` artifact.
2. Starting from the fixture `superseded` in `fixtures/software-change/` (implementation revision R1, no decision), evaluation reports no outcome. After an `explicitly_superseded` decision is added, it reports the outcome `superseded`.
3. Starting from the fixture `rolled-back` in `fixtures/software-change/` (a `release` and a `deployment`, no authority decision, no `rollback_result`), evaluation reports `release.rollback` approval-required and no outcome.
4. Starting from the state of 3, with an authority decision approving `release.rollback` added and the rollback recorded as performed but no `rollback_result`, evaluation reports `release.rollback` admissible and `rolled_back` blocked: no outcome.
5. Starting from the state of 4 and adding `rollback_result` evidence with verdict complete, evaluation reports the outcome `rolled_back`.
6. Starting from `protocols/vocabulary.yaml` as this story leaves it, read through `crates/canon-engineering/src/vocabulary.rs`, each of `declined`, `rolled_back`, `superseded`, `rollback_result`, `explicitly_declined` and `explicitly_superseded` resolves to exactly one vocabulary entry.

## Source

TASKBOARD E-006; Atlas ADR 0077 point 3 (draft); `docs/design/engineering-lifecycle-specification-design.md` § 6.7, § 8.8, § 9, § 14, § 15, § 39.8–§ 39.10; round-1 reviews `review-result:els-first-domain-acceptance-r1`, `review-result:els-first-domain-design-r1`.
