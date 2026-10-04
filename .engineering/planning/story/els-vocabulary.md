---
format: aep.planning-md/3
id: story:els-vocabulary
kind: story
status: implemented
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
scope:
- confidence: cited
  path: crates/els/src/lib.rs
- confidence: cited
  path: crates/els/src/vocabulary.rs
- confidence: cited
  path: crates/els/tests/vocabulary.rs
- confidence: cited
  path: docs/examples/incident-response.md
- confidence: cited
  path: docs/examples/software-change.md
revision: 10
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T01:17:40Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"review_outcome":4}}}
- {from: "proposed", to: "active", at: "2026-10-04T01:17:41Z", actor: "human:timo", revision: 7, decided_on: {"recorded":{"review_outcome":4}}}
- {from: "active", to: "implemented", at: "2026-10-04T01:33:49Z", actor: "human:timo", revision: 10, decided_on: {"recorded":{"test_result":1,"review_outcome":8,"verification":1}}}
---
## Outcome

ELS exposes one typed engineering vocabulary in `crates/els/src/vocabulary.rs` — artifact kinds, evidence kinds, claim ids, action ids, obligation ids and outcome ids — from which `software.change/1` and `incident.response/1` name their terms. It holds engineering names and what they mean, nothing more: it defines no claim, evidence, obligation or truth semantics, which belong to Canon (`AGENTS.md` § Boundary). Authority requirements are keyed by action id, so the vocabulary declares no separate authority-capability names.

A term that only makes sense for Git, pull requests or code is marked as belonging to `software.change`; every other term is marked core (`AGENTS.md` § Boundary, pressure test).

This story delivers the term module only. The compile-and-evaluate fixture harness and the protocol module layout are `story:fixture-harness`'s, so this story needs no Canon compile or evaluate capability and can start on its own.

### Spelling

Claim and action ids are dotted, `<subject>.<predicate>`, as in Canon `docs/design/canon-protocol-calculus-design.md` § 18 (`service.healthy`, `cause.identified`, `impact.bounded`). Artifact, evidence, obligation, outcome and case-input ids are single snake_case tokens, as in `docs/design/engineering-lifecycle-specification-design.md` § 9–§ 10. This story updates `docs/examples/incident-response.md` to that spelling (`customer_impact_bounded` → `impact.bounded`, `service_healthy` → `service.healthy`, `cause_identified` → `cause.identified`); `docs/examples/software-change.md` already uses dotted ids and changes only where one of its terms does not resolve.

### Term set declared here

| category | core | marked `software.change` |
|---|---|---|
| artifact kinds | `intent`, `system_specification`, `plan`, `release`, `deployment`, `service` | `implementation` |
| evidence kinds | `operational_observation`, `objective_observation`, `impact_assessment`, `cause_analysis` | `test_result`, `code_review`, `build_provenance` |
| claim ids | `release.proven`, `deployment.healthy`, `objective.realized`, `impact.bounded`, `service.healthy`, `cause.identified` | `tests.pass`, `implementation.reviewed`, `implementation.verified` |
| action ids | `metrics.inspect`, `logs.search`, `release.inspect`, `release.rollback`, `traffic.shift`, `emergency.leave` | `repository.inspect`, `repository.edit`, `repository.merge`, `tests.run` |
| obligation ids | `restore_service` | — |
| outcome ids | `accepted` | — |

These are every term `story:software-change-protocol` and `story:incident-response-protocol` use, so neither of them edits `crates/els/src/vocabulary.rs`.

### Terms later stories add

Each later story adds the terms it introduces to `crates/els/src/vocabulary.rs`, adds them to the exact term table in `crates/els/tests/adversary_vocabulary.rs` (which pins the vocabulary to the declared set), and says so in its own body:

| story | terms it adds |
|---|---|
| `story:software-change-profiles` | case input `risk`; profiles `trivial`, `standard`, `elevated`, `critical` |
| `story:software-change-negative-outcomes` | outcomes `declined`, `rolled_back`, `superseded`; evidence `rollback_result`; decisions `explicitly_declined`, `explicitly_superseded` |
| `story:ess-conformance-evidence` | evidence `system_conformance`; claim `implementation.conforms`; case input `affects_runtime_behavior` |
| `story:security-independence-rules` | evidence `security_review`; claim `security.reviewed`; case input `affects_security_boundary`; independence dimensions `different_principal`, `different_agent_run` |
| `story:rollback-verification-rule` | claim `rollback.verified` |

### What this story creates

- `crates/els/src/vocabulary.rs` — the engineering terms.
- The `vocabulary` module declaration in `crates/els/src/lib.rs`. The existing protocol-id functions there stay where they are.

## Shared surface

`crates/els/src/vocabulary.rs` is edited after this story by `story:software-change-profiles`, `story:software-change-negative-outcomes`, `story:ess-conformance-evidence`, `story:security-independence-rules` and `story:rollback-verification-rule`, which form one `depends_on` chain (the first four all edit `crates/els/src/protocols/software_change.rs`, and `story:rollback-verification-rule` depends on `story:security-independence-rules`), so no two of them edit it at once. `crates/els/src/lib.rs` is edited after this story by `story:fixture-harness`, which depends on it.

## Canon capability

Identifier forms of Canon `protocol/1` (Canon TASKBOARD C-001). The bootstrap Canon crate already carries `ClaimId`, `ActionId` and `ProtocolId`.

## Domain relations

- ELS protocol → Canon protocol model: many-to-one; Canon owns the language and an ELS protocol is a document in it; an ELS protocol cannot exist before the Canon capability it uses — inferable (inferred from `crates/els/Cargo.toml:9`, the `b10x-canon` dependency, and `crates/els/src/lib.rs:5`, which types protocol ids with `b10x_canon::ProtocolId`; no ess/1 document declares it, because ELS opts out of ESS for protocol semantics, `AGENTS.md` § ESS).

## Acceptance

The test `vocabulary_declares_first_domain_terms` in `crates/els/tests/vocabulary.rs` passes under `task check`. Its fixture is the vocabulary in `crates/els/src/vocabulary.rs` and the term table under *Term set declared here*, copied into the test; it expects:

1. Starting from that table, each of its 35 terms, looked up by its string, resolves to exactly one vocabulary entry, in the category the table gives it.
2. Starting from the same table, the 11 terms in the `software.change` column carry the `software.change` marking and the other 24 carry the core marking.
3. Starting from `docs/examples/software-change.md` and `docs/examples/incident-response.md` as this story leaves them, every claim id and action id the test reads out of their `text` blocks resolves to exactly one vocabulary entry.
4. Starting from the same vocabulary, looking up `service_healthy` (the spelling the incident example used before this story) and `repository.force_push` (a term nobody declares) each returns a refusal whose message names the term looked up.

## Source

TASKBOARD E-001 (Atlas `docs/design/governed-autonomy/TASKBOARD.md` § ELS); Atlas ADR 0068; Canon `docs/design/canon-protocol-calculus-design.md` § 16.2, § 18; `docs/design/engineering-lifecycle-specification-design.md` § 9–§ 10; `docs/examples/`; round-1 reviews `review-result:els-first-domain-acceptance-r1`, `review-result:els-first-domain-design-r1`, `review-result:els-first-domain-parallel-safety-r1`; round-2 review `review-result:els-first-domain-design-r2` (harness and protocol layout moved to `story:fixture-harness`).


