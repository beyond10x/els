---
format: aep.planning-md/3
id: story:ess-conformance-evidence
kind: story
status: draft
title: Admit ESS conformance reports as system-conformance evidence
summary: ess-conformance-report/2 bound to implementation and specification revisions through a closed evidence contract.
refs:
- provider: canon
  reference: taskboard:C-004
- provider: taskboard
  reference: E-007
relations:
- decomposes: epic:els-first-domain
- depends_on: story:software-change-protocol
- serves: vision:O2
- serves: vision:governed-autonomy
revision: 1
---
## Outcome

`software.change/1` admits an ESS conformance report as `system_conformance` evidence bound to two revisions — implementation and system specification. The binding is a closed evidence contract: ELS reads `ess-conformance-report/2` (`spec_digest`, `implementation`, `conformance_status`) into Canon evidence with its own types and takes no dependency on ESS crates or the ESS domain model (design §19.1). `passed` establishes, `failed` contradicts, `inconclusive` leaves the claim undecided, and a report for a previous specification digest does not establish the current claim (design §11.2). `ess-conformance-report/1` is not admitted: ESS never upconverts it to a qualifying report (ESS `docs/design/review-conformance-coverage.md` § New standalone report).

This is a use of ESS output, not an ESS specification of ELS (`AGENTS.md` § ESS).

## Canon capability

Evidence applicability and revision binding (C-004), with evidence bound to two subject revisions at once. If C-004 binds a single subject revision only, this story waits on Canon.

## Domain relations

- ELS protocol → Canon protocol model: many-to-one; Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses — inferable (inferred from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`; no ess/1 document declares it, because ELS opts out of ESS for protocol semantics, `AGENTS.md` § ESS).

## Acceptance

A `software.change/1` fixture evaluates `implementation.conforms` `TRUE` only for an `ess-conformance-report/2` whose `conformance_status` is `passed` and whose `spec_digest` and implementation both match the case’s current revisions, `FALSE` for `failed`, and `UNKNOWN` for `inconclusive` or for a `passed` report carrying the previous specification digest.

## Source

TASKBOARD E-007; Atlas ADR 0068; `docs/design/engineering-lifecycle-specification-design.md` §11.2, §19; Canon `docs/design/canon-protocol-calculus-design.md` §22.
