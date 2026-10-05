---
format: aep.planning-md/3
id: story:engineering-assertions
kind: story
status: implemented
title: Collect and replay typed engineering assertions
relations:
- serves: vision:O2
scope:
- confidence: cited
  path: .github/workflows/check.yml
- confidence: cited
  path: AGENTS.md
- confidence: cited
  path: Cargo.lock
- confidence: cited
  path: README.md
- confidence: cited
  path: catalog
- confidence: cited
  path: crates/assertion-providers
- confidence: cited
  path: crates/canon-engineering-assertions
- confidence: cited
  path: crates/canon-engineering-docs
- confidence: cited
  path: crates/canon-engineering/src/main.rs
- confidence: cited
  path: crates/canon-engineering/tests/adversary2_protocol_registry.rs
- confidence: cited
  path: crates/canon-engineering/tests/assertion_replay.rs
- confidence: cited
  path: ess
- confidence: cited
  path: fixtures/assertions
- confidence: cited
  path: website/docs
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-10-05T20:13:30Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-05T20:13:30Z", actor: "human:timo", revision: 4}
- {from: "active", to: "implemented", at: "2026-10-05T20:50:57Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"test_result":1,"review_outcome":2}}}
---
## Intent

Implement the operator-approved assertion catalog using Canon's generic expression core.
A repository gate is readable YAML with a text purpose and typed assertions. Every assertion
must evaluate TRUE. Missing or stale evidence is UNKNOWN, never a successful check.

## Acceptance

- `validates_assertions_without_collecting_evidence`: validate source, types and recipes before
  any provider runs; bad signatures or unknown fields fail with source diagnostics.
- Collection plans deduplicate requests and retain observations bound to source, catalog,
  provider implementation and context. Offline replay is pure and byte deterministic.
- Catalog covers files/text/documents/schema, explicit process bindings, normalized test
  reports and inventory, ESS reports, generated output and Codegate dependency reports.
- DNS and TCP examples and adapters distinguish negative observations from unavailable ones.
- Explicitly registered process providers extend namespaces without an engine rebuild;
  recipes extend the catalog without parser changes. No shell evaluation.
- Missing/extra generated output, zero-run tests, malformed reports, timeout, source drift,
  recipe cycles and invalid evidence cannot make gates green.
- CLI provides assertions list/describe and gates validate/run/evaluate/explain.
- README, agent instructions, public guide and generated catalog reference describe the
  implemented contract; examples execute in tests. Kubernetes and AEP acceptance integration
  are explicitly deferred, as are Codegate scores until a producer exists.

## Protocol first

Protocol semantics are unchanged. New runner data is specified first in `ess/domains/gates.yaml`,
validated with ESS 0.53.0, and provider envelopes in the provider subunit's standalone ESS root.
First commit consists only of the specification and the CLI fixture/test. Named red test:
`crates/canon-engineering/tests/assertion_cli.rs::validates_assertions_without_collecting_evidence`.
Language semantics are supplied by the corresponding Canon unit, not copied here.

## Scope

Cited: new `crates/canon-engineering-assertions`, `crates/assertion-providers`, catalog/fixtures,
existing CLI entry point and docs generator, manifests/lock, ESS and documentation.
Existing engineering protocol YAML is unchanged. Provider subunit and CLI subunit have
separate managed trees. One coherent story; no multi-child decomposition requiring critic panel.

## Approval

The operator approved the design and said “Implement the plan” on 2026-10-05.
See `docs/plan/assertions-wave.md` for execution and review evidence.
