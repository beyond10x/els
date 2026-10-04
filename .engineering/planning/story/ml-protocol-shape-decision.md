---
format: aep.planning-md/3
id: story:ml-protocol-shape-decision
kind: story
status: implemented
title: Decide the shape of the ML research protocol
summary: 'Decision record, not a build: own protocol or profile, its artifacts, claims and outcomes mapped to Canon.'
refs:
- provider: canon
  reference: taskboard:C-001
- provider: canon
  reference: taskboard:C-007
- provider: taskboard
  reference: E-009
relations:
- decomposes: epic:els-first-domain
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: cited
  path: .engineering/planning/architecture-decision-record/ml-protocol-shape.md
revision: 7
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T02:18:12Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"review_outcome":1}}}
- {from: "proposed", to: "active", at: "2026-10-04T02:18:12Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"review_outcome":1}}}
- {from: "active", to: "implemented", at: "2026-10-04T02:41:25Z", actor: "human:timo", revision: 7, decided_on: {"recorded":{"test_result":1,"review_outcome":1,"verification":1}}}
---
## Outcome

A recorded decision on the shape of an ML research protocol. This is a decision, not a build: no protocol YAML, no Rust and no fixtures.

The decision states:

- whether ML research is its own protocol (`ml.experiment/1`, which Atlas ADR 0068 lists as likely) or a profile of another, such as `engineering.investigation/1`;
- its artifacts: research question, hypothesis, experiment design, dataset snapshot, baseline, candidate, run configuration, results, analysis and conclusion;
- its claims: `experiment.valid`, `result.reproducible`, `candidate.beats_baseline`, `hypothesis.supported`, …;
- its terminal outcomes: `supported`, `refuted`, `inconclusive`, `invalid_experiment` and `superseded`, starting from Canon design § 19;
- for each of these, the Canon capability it needs, naming any gap. Protocol composition (Canon design § 39.5) is a likely one.

**The result is a protocol as data.** Under Atlas ADR 0077 point 3 (draft, operator decision 2026-10-04), an ELS protocol is a Canon `protocol/1` YAML document at `protocols/<name>/<major>.yaml` in this repository, released as versioned data and loaded and validated by Canon. The decision therefore names the YAML document its protocol becomes, for example `protocols/ml-experiment/1.yaml`. If ML research is a profile of another protocol, the decision names that protocol's existing YAML document and the case input that selects the profile. Canon's `protocol/1` has no import or composition today (canon `crates/canon/src/model/mod.rs`), so a profile cannot live in a file of its own.

## Scope

- `.engineering/planning/architecture-decision-record/ml-protocol-shape.md` (new, written through `aep plan artifact new`)

## Shared surface

None in source. This story writes only `.engineering/planning/architecture-decision-record/ml-protocol-shape.md`, which no other story touches, and it has no `depends_on` edge.

## Protocol first

Atlas ADR 0080 (draft): exempt, because there is no behaviour change. The story writes a decision record and no protocol YAML, fixture or code, so no test can go red. The protocol the decision names gets its own story, and that story carries its own § Protocol first.

## Canon capability

Mapped, not used: `protocol/1` (C-001), outcomes (C-007), and whatever else the decision names.

## Acceptance

This story builds nothing, so its acceptance is a check on the planning store, read with `aep plan artifact show architecture-decision-record:ml-protocol-shape`, rather than a test in `crates/els/tests/`. Starting from a store that holds no `architecture-decision-record` related `decides` to this story, it expects:

1. The store holds one `architecture-decision-record:ml-protocol-shape` related `decides: story:ml-protocol-shape-decision`.
2. That record is at status `accepted`, reached through `aep plan artifact move`. Its lifecycle runs `proposed → accepted`, and a record at `proposed` does not meet this.
3. Its body states whether ML research is its own protocol or a profile of another, naming that other protocol if so.
4. Its body lists the protocol's artifacts, claims and terminal outcomes, and maps each to a Canon TASKBOARD capability (C-001…C-011) or names it as a Canon gap.
5. Its body names the `protocols/<name>/<major>.yaml` document the protocol becomes. For a profile, it names the existing protocol document it extends and the case input that selects it.

## Source

TASKBOARD E-009; Atlas ADR 0068; Atlas ADR 0077 point 3 (draft); Canon `docs/design/canon-protocol-calculus-design.md` § 19, § 39.5, § 40; round-1 review `review-result:els-first-domain-acceptance-r1`.
