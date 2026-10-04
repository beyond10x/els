---
format: aep.planning-md/3
id: story:ml-protocol-shape-decision
kind: story
status: draft
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
revision: 3
---
## Outcome

A recorded decision on the shape of an ML research protocol. This is a decision, not a build: no protocol source, no Rust, no fixtures.

The decision states whether ML research is its own protocol (`ml.experiment/1`, which Atlas ADR 0068 lists as likely) or a profile of another such as `engineering.investigation/1`; its artifacts (research question, hypothesis, experiment design, dataset snapshot, baseline, candidate, run configuration, results, analysis, conclusion), claims (`experiment.valid`, `result.reproducible`, `candidate.beats_baseline`, `hypothesis.supported`, …) and terminal outcomes (`supported`, `refuted`, `inconclusive`, `invalid_experiment`, `superseded`), starting from Canon design § 19; and, for each, the Canon capability it needs, naming any gap (protocol composition, Canon design § 39.5, is a likely one).

## Shared surface

None in source: this story writes only `.engineering/planning/architecture-decision-record/ml-protocol-shape.md`, which no other story touches, and it has no `depends_on` edge.

## Canon capability

Mapped, not used: `protocol/1` (C-001), outcomes (C-007), and whatever else the decision names.

## Acceptance

This story builds no Rust, so its acceptance is a check on the planning store, read with `aep plan artifact show architecture-decision-record:ml-protocol-shape`, not a test in `crates/els/tests/`. Starting from a store that holds no `architecture-decision-record` related `decides` to this story, it expects:

1. The store holds one `architecture-decision-record:ml-protocol-shape` related `decides: story:ml-protocol-shape-decision`.
2. That record is at status `accepted` (its lifecycle runs `proposed → accepted`; a record at `proposed` does not meet this), reached through `aep plan artifact move`.
3. Its body states whether ML research is its own protocol or a profile of another, naming that other protocol if so.
4. Its body lists the protocol's artifacts, claims and terminal outcomes, and maps each to a Canon TASKBOARD capability (C-001…C-011) or names it as a Canon gap.

## Source

TASKBOARD E-009; Atlas ADR 0068; Canon `docs/design/canon-protocol-calculus-design.md` § 19, § 39.5, § 40; round-1 review `review-result:els-first-domain-acceptance-r1`.
