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
revision: 1
---
## Outcome

ELS fixtures show evidence going stale in every way the two protocols declare, beyond the R1/R2 test case that E-002 already owns: a review approval bound to an earlier implementation revision, a software operational observation past its freshness horizon, and an incident health observation past its horizon. Each stale case evaluates `UNKNOWN`, never `FALSE` (`AGENTS.md` § Rules). Staleness of conformance evidence after a specification revision belongs to E-007.

The fixtures exercise Canon behaviour through ELS protocols; ELS does not implement applicability or freshness. The evaluation instant is passed explicitly.

## Canon capability

Evidence applicability and revision binding (C-004); invalidation and freshness (C-008).

## Domain relations

- ELS protocol → Canon protocol model: many-to-one; Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses — inferable (inferred from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`; no ess/1 document declares it, because ELS opts out of ESS for protocol semantics, `AGENTS.md` § ESS).

## Acceptance

The ELS fixture suite passes cases in which an R1 code-review approval leaves the review claim `UNKNOWN` on R2, a deployment-health observation older than its freshness horizon leaves `deployment.healthy` `UNKNOWN`, and an incident health observation older than its horizon leaves `service_healthy` `UNKNOWN`, with none of the three evaluating `FALSE`.

## Source

TASKBOARD E-005; `docs/design/engineering-lifecycle-specification-design.md` §11, §39.7; `AGENTS.md` § Rules.
