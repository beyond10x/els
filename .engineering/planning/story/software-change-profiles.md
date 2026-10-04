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
revision: 1
---
## Outcome

Four risk profiles — trivial, standard, elevated, critical — change the proof burden of `software.change/1` without creating a second lifecycle: the same claims and transitions, different derived obligations and authority requirements.

Design basis: `docs/design/engineering-lifecycle-specification-design.md` §9 declares risk as a case input and §10 derives obligations from it; §27 creates a distinct lifecycle only when the semantic graph differs. A profile is therefore the value of that case input, not an imported protocol: protocol composition is an open Canon question (Canon design §39.5) and no Canon TASKBOARD item C-001…C-011 delivers it. The TASKBOARD says `standard`; design §9 says `normal`; use `standard`.

Pressure test: the mechanism (case input → derived obligations) names no Git or pull-request concept, so `incident.response/1` could later key severity on it. Adding incident severity is not part of this story.

## Canon capability

Case inputs in `protocol/1` (C-001); obligations derived by rule (C-005).

## Domain relations

- ELS protocol → Canon protocol model: many-to-one; Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses — inferable (inferred from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`; no ess/1 document declares it, because ELS opts out of ESS for protocol semantics, `AGENTS.md` § ESS).

## Acceptance

Evaluating one `software.change/1` fixture case under each of the trivial, standard, elevated and critical profiles yields four different open-obligation sets, each containing the set of the profile below it.

## Source

TASKBOARD E-003; `docs/design/engineering-lifecycle-specification-design.md` §6.6, §9, §10, §27.
