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
- depends_on: story:software-change-protocol
- serves: vision:O2
- serves: vision:governed-autonomy
revision: 1
---
## Outcome

`software.change/1` declares terminal outcomes beyond `accepted`:

- `declined` — a decision to make no change, reachable without any implementation artifact (design §6.7, ELS-OUTCOME-001);
- `rolled_back` — earned only with rollback-result evidence; invoking a rollback is not enough (design §15, ELS-RECOVERY-001);
- `superseded` — earned on an explicit supersession decision recorded on this case.

The relation between a change and the change that supersedes it is not modelled here: design §45.5 leaves change nesting open, so the decision is a fact on this case only.

Pressure test: no outcome assumes Git; a declined change carries no repository artifact.

## Canon capability

Outcomes and completion (C-007).

## Domain relations

- ELS protocol → Canon protocol model: many-to-one; Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses — inferable (inferred from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`; no ess/1 document declares it, because ELS opts out of ESS for protocol semantics, `AGENTS.md` § ESS).

## Acceptance

`software.change/1` fixtures conclude one case as `declined` with no implementation artifact and one as `superseded` on an explicit supersession decision, and report `rolled_back` blocked after a rollback action until rollback-result evidence is supplied.

## Source

TASKBOARD E-006; `docs/design/engineering-lifecycle-specification-design.md` §6.7, §8.8, §14, §15, §39.8–§39.10.
