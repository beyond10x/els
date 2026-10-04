---
format: aep.planning-md/3
id: story:software-change-profiles
kind: story
status: draft
title: Define trivial, standard, elevated and critical profiles for software.change/1
summary: Risk profiles change derived obligations and authority, not the lifecycle.
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
scope:
- confidence: inferred
  path: crates/els/src/vocabulary.rs
- confidence: cited
  path: crates/els/tests/software_change_profiles.rs
- confidence: cited
  path: fixtures/software-change/
- confidence: cited
  path: protocols/software-change/1.yaml
- confidence: cited
  path: protocols/vocabulary.yaml
revision: 6
---
## Outcome

Four risk profiles (trivial, standard, elevated, critical) change the proof burden of `software.change/1` without creating a second lifecycle. Each profile keeps the same claims and transitions and changes only the derived obligations and authority requirements.

This story edits `protocols/software-change/1.yaml` to add the `risk` case input and the obligation rules derived from it. It adds the terms it introduces to `protocols/vocabulary.yaml`: the case input `risk` and the profiles `trivial`, `standard`, `elevated` and `critical`. The typed reader `crates/els/src/vocabulary.rs` knows only the six categories of `story:els-vocabulary`, none of which is a case input or a profile. So this story adds the categories those terms need to the reader, and adds no term to the Rust (Atlas ADR 0077 point 3, draft: the vocabulary is data).

Design basis: `docs/design/engineering-lifecycle-specification-design.md` § 9 declares risk as a case input, and § 10 derives obligations from it. § 27 creates a distinct lifecycle only when the semantic graph differs. A profile is therefore the value of that case input, not an imported protocol. Protocol composition is an open Canon question (Canon design § 39.5), and canon's `protocol/1` model has no import section (`crates/canon/src/model/mod.rs`). The TASKBOARD says `standard` and design § 9 says `normal`; use `standard`.

Pressure test: the mechanism (case input → derived obligations) names no Git or pull-request concept, so `incident.response/1` could later key severity on it. Adding incident severity is not part of this story.

## Scope

- `protocols/software-change/1.yaml`
- `fixtures/software-change/`
- `protocols/vocabulary.yaml`
- `crates/els/src/vocabulary.rs` (inferred: the reader's term categories)
- `crates/els/tests/software_change_profiles.rs` (new)

## Shared surface

This story is the second link of the `software.change/1` chain: `story:software-change-protocol` → `story:software-change-profiles` → `story:software-change-negative-outcomes` → `story:ess-conformance-evidence` → `story:security-independence-rules` → `story:stale-evidence-fixtures`. Every story on it edits `protocols/software-change/1.yaml` and `fixtures/software-change/`, and Canon has no composition that would let a profile live in its own file. This story depends on `story:software-change-protocol` and runs before `story:software-change-negative-outcomes`.

It is also the first story after `story:vocabulary-yaml-source` to edit `protocols/vocabulary.yaml` and `crates/els/src/vocabulary.rs`, and it depends on that story, which creates the YAML and the reader. It runs beside `story:protocol-registry`, which touches neither file.

## Protocol first

Atlas ADR 0080 (draft). The first commit changes the following, and nothing else:

- `protocols/software-change/1.yaml`, adding the `risk` case input and its derived obligations;
- the fixture `profiles` in `fixtures/software-change/`, with the expectations in § Acceptance;
- `crates/els/tests/software_change_profiles.rs`.

On that commit `profiles_nest_open_obligations` fails at item 4. `risk`, `trivial`, `standard`, `elevated` and `critical` do not yet resolve in the vocabulary, and the reader has no category for them. The implementation commit adds the terms to `protocols/vocabulary.yaml` and the categories to `crates/els/src/vocabulary.rs`, without changing the protocol YAML or the fixture.

## Canon capability

Case inputs in `protocol/1` (C-001) and obligations derived by rule (C-005).

**Gap.** canon's `protocol/1` model at `0d2437d` has no case-input section. `crates/canon/src/model/mod.rs` declares only `artifacts`, `evidence_kinds`, `claims`, `obligations`, `actions` and `outcomes`, with `deny_unknown_fields`, and `Obligation` carries only a description, with no derivation rule. C-001 is implemented in canon (`story:protocol-source-model`) without them. This story, and the later uses of `affects_runtime_behavior` and `affects_security_boundary`, wait on Canon for case inputs and derived obligations.

## Domain relations

- ELS protocol → Canon protocol model: many-to-one. Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses. Inferable: from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`. No ess/1 document declares it, because ELS opts out of ESS for protocol semantics (`AGENTS.md` § ESS).

## Acceptance

The test `profiles_nest_open_obligations` in `crates/els/tests/software_change_profiles.rs` passes under `task check`. Its fixture is `profiles` in `fixtures/software-change/`: implementation revision R1, no evidence, no authority decision, and one fixed evaluation instant. The fixture is evaluated four times, with `risk` set to `trivial`, `standard`, `elevated` and `critical`. It expects:

1. Starting from that fixture, all four evaluations run against one `canon-ir/1` document compiled once from `protocols/software-change/1.yaml`, so no profile has a protocol of its own.
2. Starting from that fixture, the four open-obligation sets the evaluations report are pairwise different.
3. Starting from that fixture, each open-obligation set strictly contains the set of the profile below it: trivial ⊂ standard ⊂ elevated ⊂ critical.
4. Starting from `protocols/vocabulary.yaml` as this story leaves it, read through `crates/els/src/vocabulary.rs`, each of `risk`, `trivial`, `standard`, `elevated` and `critical` resolves to exactly one vocabulary entry.

## Source

TASKBOARD E-003; Atlas ADR 0077 point 3 (draft); `docs/design/engineering-lifecycle-specification-design.md` § 6.6, § 9, § 10, § 27; round-1 review `review-result:els-first-domain-parallel-safety-r1`.
