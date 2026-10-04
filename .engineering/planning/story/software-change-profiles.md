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
scope:
- confidence: cited
  path: crates/els/src/protocols/software_change.rs
- confidence: cited
  path: crates/els/src/vocabulary.rs
- confidence: cited
  path: crates/els/tests/software_change_profiles.rs
- confidence: cited
  path: fixtures/software-change/
revision: 3
---
## Outcome

Four risk profiles — trivial, standard, elevated, critical — change the proof burden of `software.change/1` without creating a second lifecycle: the same claims and transitions, different derived obligations and authority requirements. This story edits `crates/els/src/protocols/software_change.rs` to add the `risk` case input and the obligation rules derived from it, and adds the terms it introduces to `crates/els/src/vocabulary.rs`: the case input `risk` and the profiles `trivial`, `standard`, `elevated` and `critical`.

Design basis: `docs/design/engineering-lifecycle-specification-design.md` § 9 declares risk as a case input and § 10 derives obligations from it; § 27 creates a distinct lifecycle only when the semantic graph differs. A profile is therefore the value of that case input, not an imported protocol: protocol composition is an open Canon question (Canon design § 39.5) and no Canon TASKBOARD item C-001…C-011 delivers it. The TASKBOARD says `standard`; design § 9 says `normal`; use `standard`.

Pressure test: the mechanism (case input → derived obligations) names no Git or pull-request concept, so `incident.response/1` could later key severity on it. Adding incident severity is not part of this story.

## Shared surface

`crates/els/src/protocols/software_change.rs`, `fixtures/software-change/` and `crates/els/src/vocabulary.rs` are edited by every story on the `software.change/1` chain, so this story is its second link: `story:software-change-protocol` → `story:software-change-profiles` → `story:software-change-negative-outcomes` → `story:ess-conformance-evidence` → `story:security-independence-rules` → `story:stale-evidence-fixtures`. It depends on `story:software-change-protocol` and runs before `story:software-change-negative-outcomes`.

## Canon capability

Case inputs in `protocol/1` (C-001); obligations derived by rule (C-005).

## Domain relations

- ELS protocol → Canon protocol model: many-to-one; Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses — inferable (inferred from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`; no ess/1 document declares it, because ELS opts out of ESS for protocol semantics, `AGENTS.md` § ESS).

## Acceptance

The test `profiles_nest_open_obligations` in `crates/els/tests/software_change_profiles.rs` passes under `task check`. Its fixture is `profiles` in `fixtures/software-change/` (implementation revision R1, no evidence, no authority decision, one fixed evaluation instant), evaluated four times, with `risk` set to `trivial`, `standard`, `elevated` and `critical`; it expects:

1. Starting from that fixture, all four evaluations run against one `canon-ir/1` document compiled once from `crates/els/src/protocols/software_change.rs`: no profile has a protocol of its own.
2. Starting from that fixture, the four open-obligation sets the evaluations report are pairwise different.
3. Starting from that fixture, each open-obligation set strictly contains the set of the profile below it: trivial ⊂ standard ⊂ elevated ⊂ critical.
4. Starting from `crates/els/src/vocabulary.rs` as this story leaves it, `risk`, `trivial`, `standard`, `elevated` and `critical` each resolve to exactly one vocabulary entry.

## Source

TASKBOARD E-003; `docs/design/engineering-lifecycle-specification-design.md` § 6.6, § 9, § 10, § 27; round-1 review `review-result:els-first-domain-parallel-safety-r1`.
