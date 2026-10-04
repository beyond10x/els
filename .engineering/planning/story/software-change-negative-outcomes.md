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
- confidence: cited
  path: crates/els/src/protocols/software_change.rs
- confidence: cited
  path: crates/els/src/vocabulary.rs
- confidence: cited
  path: crates/els/tests/software_change_negative_outcomes.rs
- confidence: cited
  path: fixtures/software-change/
revision: 4
---
## Outcome

`software.change/1` declares terminal outcomes beyond `accepted`:

- `declined` — earned on an `explicitly_declined` decision, reachable without any implementation artifact (design § 6.7, § 9 `completion.declined`, ELS-OUTCOME-001);
- `rolled_back` — earned only with `rollback_result` evidence whose verdict is complete; invoking a rollback is not enough (design § 9 `completion.rolled_back`, § 15, ELS-RECOVERY-001);
- `superseded` — earned on an `explicitly_superseded` decision recorded on this case.

To make `rolled_back` reachable this story also declares, in `crates/els/src/protocols/software_change.rs`, the action `release.rollback` on `software.change/1` with an authority requirement (the term is already in the vocabulary from `story:els-vocabulary`), and the evidence kind `rollback_result` with subject `release`. It adds the terms it introduces to `crates/els/src/vocabulary.rs`: the outcomes `declined`, `rolled_back` and `superseded`, the evidence kind `rollback_result`, and the decisions `explicitly_declined` and `explicitly_superseded`. `story:rollback-verification-rule` later uses `rollback_result` in `incident.response/1`.

The relation between a change and the change that supersedes it is not modelled here: design § 45.5 leaves change nesting open, so the decision is a fact on this case only.

Pressure test: no outcome assumes Git; a declined change carries no repository artifact.

## Shared surface

`crates/els/src/protocols/software_change.rs`, `fixtures/software-change/` and `crates/els/src/vocabulary.rs` are edited by every story on the `software.change/1` chain, so this story is its third link: `story:software-change-protocol` → `story:software-change-profiles` → `story:software-change-negative-outcomes` → `story:ess-conformance-evidence` → `story:security-independence-rules` → `story:stale-evidence-fixtures`. It depends on `story:software-change-profiles` and runs before `story:ess-conformance-evidence`.

## Canon capability

Outcomes and completion (C-007); action admissibility with authority (C-006) for `release.rollback`.

## Domain relations

- ELS protocol → Canon protocol model: many-to-one; Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses — inferable (inferred from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`; no ess/1 document declares it, because ELS opts out of ESS for protocol semantics, `AGENTS.md` § ESS).

## Acceptance

The test `negative_outcomes_are_earned` in `crates/els/tests/software_change_negative_outcomes.rs` passes under `task check`; it expects:

1. Starting from the fixture `declined` in `fixtures/software-change/` (an `intent` artifact, no `implementation` artifact, no decision), evaluation reports no outcome; after adding an `explicitly_declined` decision, it reports the outcome `declined`, and the case still holds no `implementation` artifact.
2. Starting from the fixture `superseded` in `fixtures/software-change/` (implementation revision R1, no decision), evaluation reports no outcome; after adding an `explicitly_superseded` decision, it reports the outcome `superseded`.
3. Starting from the fixture `rolled-back` in `fixtures/software-change/` (a `release` and a `deployment`, no authority decision, no `rollback_result`), evaluation reports `release.rollback` approval-required and no outcome.
4. Starting from the state of 3, adding an authority decision approving `release.rollback` and recording the rollback as performed, with no `rollback_result`, evaluation reports `release.rollback` admissible and `rolled_back` blocked: no outcome.
5. Starting from the state of 4 and adding `rollback_result` evidence with verdict complete, evaluation reports the outcome `rolled_back`.
6. Starting from `crates/els/src/vocabulary.rs` as this story leaves it, `declined`, `rolled_back`, `superseded`, `rollback_result`, `explicitly_declined` and `explicitly_superseded` each resolve to exactly one vocabulary entry.

## Source

TASKBOARD E-006; `docs/design/engineering-lifecycle-specification-design.md` § 6.7, § 8.8, § 9, § 14, § 15, § 39.8–§ 39.10; round-1 reviews `review-result:els-first-domain-acceptance-r1`, `review-result:els-first-domain-design-r1`.
