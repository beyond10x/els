---
format: aep.planning-md/3
id: epic:els-first-domain
kind: epic
status: active
title: 'ELS first domain: software change and incident response'
summary: Engineering vocabulary, software.change/1 with profiles and outcomes, incident.response/1, ESS evidence binding, ML protocol shape decided.
refs:
- provider: atlas
  reference: epic:ga-els-first-domain
relations:
- serves: vision:governed-autonomy
- serves: vision:O2
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T00:01:14Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-04T01:17:40Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"review_outcome":1}}}
---
## Outcome

Engineering protocols on Canon. Covers TASKBOARD E-001 … E-009.

## Acceptance

`task check` passes with the stories under this epic implemented, and the two protocol fixtures, run through the fixture harness `crates/els/tests/support/mod.rs` against one Canon crate and evaluator, report:

1. Fixture `chg-1842` in `fixtures/software-change/` (the test `chg_1842_merge_waits_for_current_revision_tests_and_authority`): `software.change/1` compiles through Canon to a `canon-ir/1` document with no error, and starting from implementation revision R2 with one passing `test_result` bound to R1, evaluation reports `tests.pass = UNKNOWN`, not `FALSE`.
2. Fixture `inc-492` in `fixtures/incident-response/` (the test `inc_492_leaves_emergency_while_cause_unknown`): `incident.response/1` compiles through the same Canon crate as item 1 to a `canon-ir/1` document with no error, and starting from an `impact_assessment` reporting impact bounded, an `operational_observation` reporting the service healthy after one reporting it unhealthy, and no `cause_analysis`, evaluation reports `cause.identified = UNKNOWN`, `restore_service` satisfied and `emergency.leave` admissible.

## Source

Atlas `epic:ga-els-first-domain`; Atlas ADR 0068; `docs/design/engineering-lifecycle-specification-design.md`; round-2 review `review-result:els-first-domain-acceptance-r2`.
