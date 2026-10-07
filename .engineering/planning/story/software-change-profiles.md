---
format: aep.planning-md/3
id: story:software-change-profiles
kind: story
status: draft
title: Define risk profiles and bugfix/refactor change-kind profiles for software.change/1
summary: Risk profiles change derived obligations and authority; the bugfix profile adds the red-first rule, refactor omits it; repository.edit splits into tests.write and implementation.edit.
refs:
- provider: canon
  reference: taskboard:C-001
- provider: canon
  reference: taskboard:C-005
- provider: taskboard
  reference: E-003
relations:
- decomposes: epic:els-first-domain
- depends_on: story:software-change-protocol
- serves: vision:O2
- serves: vision:governed-autonomy
- depends_on: story:vocabulary-yaml-source
- depends_on: story:software-change-negative-outcomes
scope:
- confidence: inferred
  path: crates/canon-engineering/src/vocabulary.rs
- confidence: cited
  path: crates/canon-engineering/tests/adversary_vocabulary.rs
- confidence: cited
  path: crates/canon-engineering/tests/software_change_profiles.rs
- confidence: cited
  path: crates/canon-engineering/tests/vocabulary.rs
- confidence: cited
  path: crates/canon-engineering/tests/vocabulary_yaml.rs
- confidence: cited
  path: docs/examples/software-change.md
- confidence: cited
  path: fixtures/software-change/
- confidence: cited
  path: protocols/software-change/1.yaml
- confidence: cited
  path: protocols/vocabulary.yaml
revision: 23
---
## Outcome

Four risk profiles (trivial, standard, elevated, critical) change the proof burden of `software.change/1` without creating a second lifecycle. Each profile keeps the same claims and transitions and changes only the derived obligations and authority requirements.

This story edits `protocols/software-change/1.yaml` to add the `risk` case input and the obligation rules derived from it. It adds the terms it introduces to `protocols/vocabulary.yaml`: the case input `risk` and the profiles `trivial`, `standard`, `elevated` and `critical`. The typed reader `crates/canon-engineering/src/vocabulary.rs` knows only the six categories of `story:els-vocabulary`, none of which is a case input or a profile. So this story adds the categories those terms need to the reader, and adds no term to the Rust (Atlas ADR 0077 point 3, draft: the vocabulary is data).

Design basis: `docs/design/engineering-lifecycle-specification-design.md` § 9 declares risk as a case input, and § 10 derives obligations from it. § 27 creates a distinct lifecycle only when the semantic graph differs. A profile is therefore the value of that case input, not an imported protocol. Protocol composition is an open Canon question (Canon design § 39.5), and canon's `protocol/1` model has no import section (`crates/canon/src/model/mod.rs`). The TASKBOARD says `standard` and design § 9 says `normal`; use `standard`.

Pressure test: the mechanism (case input → derived obligations) names no Git or pull-request concept, so `incident.response/1` could later key severity on it. Adding incident severity is not part of this story.

## ADR 0084 addition: bugfix and refactor profiles

Atlas ADR 0084 (accepted 2026-10-04, option C; on Atlas branch `plan/ga-adrs-0084-0085` at `38266148`, not yet on `main`) § Also needed: AEP's `applies_when: task.kind any_of [feature, bugfix]` (aep `principles/development/test-driven.yaml:18-20`) becomes a `software.change/1` profile. Same mechanism as the risk profiles: a second case input, `change_kind`, whose value selects the rule; no second lifecycle.

- **`repository.edit` splits** into `tests.write` (no precondition) and `implementation.edit`, in `protocols/vocabulary.yaml` and in `protocols/software-change/1.yaml` (ADR 0084 § Decision 1). `repository.edit` leaves the vocabulary.
- **`bugfix`** adds the red-first rule: `implementation.edit` has the precondition claim `regression.reproduced`, TRUE on a `test_result` whose result is `failed`. Commission then does not invoke `implementation.edit` before red exists (ADR 0082). Once Canon has the evidence-order predicate (canon `story:evidence-order-predicate`), the rule also requires the first `test_result` to precede the first implementation change; that tightening is a later change to this protocol, not part of this story.
- **`refactor`** omits the rule: `implementation.edit` has no red-first precondition.
- Not added: `feature`. AEP's principle covers it, ADR 0084 names only `bugfix` and `refactor`.
- Telling a test edit from an implementation edit needs a path scope on capabilities (ADR 0084 § Decision 1, ADR 0083 § Open; canon `story:capability-scope`). Until then the split is by action id only.

**Gap.** A precondition that holds only under one `change_kind` needs case inputs in `protocol/1`, the same gap § Canon capability names for `risk`.

## Scope

- `protocols/software-change/1.yaml`
- `fixtures/software-change/`
- `protocols/vocabulary.yaml`
- `crates/canon-engineering/src/vocabulary.rs` (inferred: the reader's term categories)
- `crates/canon-engineering/tests/software_change_profiles.rs` (new)
- `crates/canon-engineering/tests/vocabulary.rs`, `crates/canon-engineering/tests/vocabulary_yaml.rs`, `crates/canon-engineering/tests/adversary_vocabulary.rs` (each lists `repository.edit` as an expected term; the split replaces it)
- `docs/examples/software-change.md` (names `repository.edit` in its example output)

## Shared surface

This story is the third link of the `software.change/1` chain: `story:software-change-protocol` → `story:software-change-negative-outcomes` → `story:software-change-profiles` → `story:ess-conformance-evidence` → `story:security-independence-rules` → `story:stale-evidence-fixtures`. Every story on it edits `protocols/software-change/1.yaml` and `fixtures/software-change/`, and Canon has no composition that would let a profile live in its own file. This story depends on `story:software-change-negative-outcomes` and runs before `story:ess-conformance-evidence`, which needs the case-input category this story adds to the reader.

It also depends on `story:vocabulary-yaml-source`, which creates `protocols/vocabulary.yaml` and the reader `crates/canon-engineering/src/vocabulary.rs`. It edits both after `story:software-change-negative-outcomes`, which adds the reader's decision category. It runs beside `story:protocol-registry`, which touches neither file.

## Protocol first

Atlas ADR 0080 (draft). The first commit changes the following, and nothing else:

- `protocols/software-change/1.yaml`, adding the `risk` and `change_kind` case inputs, the obligations derived from `risk`, the split of `repository.edit` into `tests.write` and `implementation.edit`, the claim `regression.reproduced` and the `bugfix` precondition on `implementation.edit`;
- the fixtures `profiles` and `change-kinds` in `fixtures/software-change/`, with the expectations in § Acceptance;
- `crates/canon-engineering/tests/software_change_profiles.rs`.

On that commit `profiles_nest_open_obligations` fails at items 4 and 8. `risk`, `trivial`, `standard`, `elevated`, `critical`, `change_kind`, `bugfix`, `refactor`, `tests.write`, `implementation.edit` and `regression.reproduced` do not yet resolve in the vocabulary, `repository.edit` still does, and the reader has no category for case inputs or profiles. The implementation commit adds and removes the terms in `protocols/vocabulary.yaml`, the categories in `crates/canon-engineering/src/vocabulary.rs`, the expected-term lists in the three vocabulary test files and the action names in `docs/examples/software-change.md`, without changing the protocol YAML or the fixtures.

## Canon capability

Case inputs in `protocol/1` (C-001) and obligations derived by rule (C-005).

**Gap.** canon's `protocol/1` model at `0d2437d` has no case-input section. `crates/canon/src/model/mod.rs` declares only `artifacts`, `evidence_kinds`, `claims`, `obligations`, `actions` and `outcomes`, with `deny_unknown_fields`, and `Obligation` carries only a description, with no derivation rule. C-001 is implemented in canon (`story:protocol-source-model`) without them. This story, and the later uses of `affects_runtime_behavior` and `affects_security_boundary`, wait on Canon for case inputs and derived obligations.

## Domain relations

- ELS protocol → Canon protocol model: many-to-one. Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses. Inferable: from `crates/canon-engineering/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/canon-engineering/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`. No ess/1 document declares it, because ELS opts out of ESS for protocol semantics (`AGENTS.md` § ESS).

## Acceptance

The test `profiles_nest_open_obligations` in `crates/canon-engineering/tests/software_change_profiles.rs` passes under `task check`. Its fixture is `profiles` in `fixtures/software-change/`: implementation revision R1, no evidence, no authority decision, and one fixed evaluation instant. The fixture is evaluated four times, with `risk` set to `trivial`, `standard`, `elevated` and `critical`. It expects:

1. Starting from that fixture, all four evaluations run against one `canon-ir/1` document compiled once from `protocols/software-change/1.yaml`, so no profile has a protocol of its own.
2. Starting from that fixture, the four open-obligation sets the evaluations report are pairwise different.
3. Starting from that fixture, each open-obligation set strictly contains the set of the profile below it: trivial ⊂ standard ⊂ elevated ⊂ critical.
4. Starting from `protocols/vocabulary.yaml` as this story leaves it, read through `crates/canon-engineering/src/vocabulary.rs`, each of `risk`, `trivial`, `standard`, `elevated` and `critical` resolves to exactly one vocabulary entry.
5. Starting from the fixture `change-kinds` (implementation revision R1, no authority decision, one fixed evaluation instant) with `change_kind` `bugfix` and no evidence, `tests.write` is admissible and `implementation.edit` is blocked naming `regression.reproduced`.
6. Starting from `change-kinds` with `change_kind` `bugfix` and one `test_result` whose result is `failed`, `implementation.edit` is admissible.
7. Starting from `change-kinds` with `change_kind` `refactor` and no evidence, `implementation.edit` is admissible.
8. Starting from `protocols/vocabulary.yaml` as this story leaves it, each of `change_kind`, `bugfix`, `refactor`, `tests.write`, `implementation.edit` and `regression.reproduced` resolves to exactly one vocabulary entry, and `repository.edit` resolves to none.

## Source

TASKBOARD E-003; Atlas ADR 0084 § Decision 1 and § Also needed; aep `principles/development/test-driven.yaml:18-20,26`; Atlas ADR 0077 point 3 (draft); `docs/design/engineering-lifecycle-specification-design.md` § 6.6, § 9, § 10, § 27; round-1 review `review-result:els-first-domain-parallel-safety-r1`.

## Order

Third of the three stories https://github.com/beyond10x/engineering-protocols/issues/7 asks to order: `story:software-change-negative-outcomes`, then `story:ml-experiment-protocol`, then this story. It is last because Canon has no case inputs and no derived obligations, and nothing in Canon plans them yet: `blocker:canon-case-inputs` blocks this story until a Canon release ships both.
