---
format: aep.planning-md/3
id: story:els-vocabulary
kind: story
status: draft
title: Define the engineering vocabulary package
summary: Typed engineering terms shared by software.change/1 and incident.response/1; no generic claim or evidence semantics.
refs:
- provider: canon
  reference: taskboard:C-001
- provider: taskboard
  reference: E-001
relations:
- decomposes: epic:els-first-domain
- serves: vision:O2
- serves: vision:governed-autonomy
revision: 1
---
## Outcome

ELS exposes one typed engineering vocabulary — case kinds, artifact kinds, evidence kinds, claim ids, action ids and authority capabilities — from which `software.change/1` and `incident.response/1` name their terms. It holds engineering names and what they mean, nothing more: it defines no claim, evidence, obligation or truth semantics, which belong to Canon (`AGENTS.md` § Boundary).

Terms come from both domains: delivery (implementation, system specification, release, deployment, test result, code review, `repository.merge`) and operations (incident, service health, customer impact, cause, remediation, `metrics.inspect`, `release.rollback`, `traffic.shift`). A term that only makes sense for Git, pull requests or code is marked as belonging to `software.change`, not to the shared core.

Where sources disagree the vocabulary settles one spelling: `docs/examples/incident-response.md` writes `service_healthy` and `cause_identified`, Canon `docs/design/canon-protocol-calculus-design.md` §18 writes `service.healthy` and `cause.identified`.

## Canon capability

Identifier forms of Canon `protocol/1` (Canon TASKBOARD C-001). The bootstrap Canon crate already carries `ClaimId`, `ActionId` and `ProtocolId`.

## Domain relations

- ELS protocol → Canon protocol model: many-to-one; Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses — inferable (inferred from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`; no ess/1 document declares it, because ELS opts out of ESS for protocol semantics, `AGENTS.md` § ESS).

## Acceptance

An `els` test resolves every engineering term named in `docs/examples/software-change.md` and `docs/examples/incident-response.md` to exactly one typed vocabulary entry and shows a term absent from the vocabulary refused with its name.

## Source

TASKBOARD E-001 (Atlas `docs/design/governed-autonomy/TASKBOARD.md` § ELS); Atlas ADR 0068; Canon `docs/design/canon-protocol-calculus-design.md` §16.2; `docs/examples/`.
