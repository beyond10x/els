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
revision: 1
---
## Outcome

A recorded decision on the shape of an ML research protocol. This is a decision, not a build: no protocol source, no Rust, no fixtures.

The decision states whether ML research is its own protocol (`ml.experiment/1`, which Atlas ADR 0068 lists as likely) or a profile of another such as `engineering.investigation/1`; its artifacts (research question, hypothesis, experiment design, dataset snapshot, baseline, candidate, run configuration, results, analysis, conclusion), claims (`experiment.valid`, `result.reproducible`, `candidate.beats_baseline`, `hypothesis.supported`, …) and terminal outcomes (`supported`, `refuted`, `inconclusive`, `invalid_experiment`, `superseded`), starting from Canon design §19; and, for each, the Canon capability it needs, naming any gap (protocol composition, Canon design §39.5, is a likely one).

## Canon capability

Mapped, not used: `protocol/1` (C-001), outcomes (C-007), and whatever else the decision names.

## Acceptance

The ELS store holds an `architecture-decision-record` related `decides` to this story that states whether ML research is its own protocol or a profile of another and lists its artifacts, claims and terminal outcomes, each mapped to a Canon TASKBOARD capability or named as a Canon gap.

## Source

TASKBOARD E-009; Atlas ADR 0068; Canon `docs/design/canon-protocol-calculus-design.md` §19, §39.5, §40.
