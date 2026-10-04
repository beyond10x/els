---
format: aep.planning-md/3
id: epic:els-first-domain
kind: epic
status: proposed
title: 'ELS first domain: software change and incident response'
summary: Engineering vocabulary, software.change/1 with profiles and outcomes, incident.response/1, ESS evidence binding, ML protocol shape decided.
refs:
- provider: atlas
  reference: epic:ga-els-first-domain
relations:
- serves: vision:governed-autonomy
- serves: vision:O2
revision: 2
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:01:14Z", actor: "human:timo", revision: 2}
---
## Outcome

Engineering protocols on Canon. Covers TASKBOARD E-001 … E-009.

## Acceptance

`software.change/1` and `incident.response/1` both compile and evaluate on the same Canon kernel; the
software-change fixture reports `tests.pass = UNKNOWN` when test evidence exists only for R1 and the
implementation is R2; the incident fixture can leave emergency mode with `cause_identified = UNKNOWN`.

## Source

Atlas `epic:ga-els-first-domain`; Atlas ADR 0068; `docs/design/engineering-lifecycle-specification-design.md`.
