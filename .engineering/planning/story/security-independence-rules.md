---
format: aep.planning-md/3
id: story:security-independence-rules
kind: story
status: draft
title: Add security and independence rules
summary: Independent security review for security-boundary changes and independent rollback verification for incidents.
refs:
- provider: canon
  reference: taskboard:C-004
- provider: canon
  reference: taskboard:C-010
- provider: taskboard
  reference: E-008
relations:
- decomposes: epic:els-first-domain
- depends_on: story:software-change-protocol
- depends_on: story:incident-response-protocol
- serves: vision:O2
- serves: vision:governed-autonomy
revision: 1
---
## Outcome

`software.change/1` derives an independent security-review obligation when a change affects a security boundary (design §10, §12), and `incident.response/1` requires rollback verification independent of the actor who performed the rollback (design §39.9) — the same rule form in both protocols.

ELS declares which independence dimensions a requirement needs (different principal, different agent run, and so on). It does not resolve identities: the fixture supplies the independence decision as trusted input (design §12, §45.3; Canon design §39.4). Canon owns the requirement form and the rule that evidence failing it does not satisfy (CANON-INDEPENDENCE-001).

## Canon capability

Evidence applicability (C-004) and an independence requirement form. No Canon TASKBOARD item C-001…C-011 names independence: CANON-INDEPENDENCE-001 appears only in the conformance list (C-010). If neither C-004 nor C-010 delivers the form, this story waits on Canon.

## Domain relations

- ELS protocol → Canon protocol model: many-to-one; Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses — inferable (inferred from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`; no ess/1 document declares it, because ELS opts out of ESS for protocol semantics, `AGENTS.md` § ESS).

## Acceptance

A `software.change/1` fixture with `affects_security_boundary = true` leaves the security-review claim `UNKNOWN` when the approving review comes from the implementing principal and `TRUE` only for an approving review from a producer the supplied independence decision marks independent, and an `incident.response/1` fixture leaves rollback verification `UNKNOWN` when the verifier is the actor who performed the rollback.

## Source

TASKBOARD E-008; `docs/design/engineering-lifecycle-specification-design.md` §10, §12, §39.5, §39.9.
