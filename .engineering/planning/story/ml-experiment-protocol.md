---
format: aep.planning-md/3
id: story:ml-experiment-protocol
kind: story
status: draft
title: Ship the ml.experiment/1 protocol as data
relations:
- decomposes: epic:els-first-domain
- depends_on: story:ml-protocol-shape-decision
revision: 1
---
## Outcome

ELS ships `ml.experiment/1` as Canon `protocol/1` data at `protocols/ml-experiment/1.yaml`, with
the artifacts, claims and terminal outcomes recorded in
`architecture-decision-record:ml-protocol-shape`, and fixtures under `fixtures/ml-experiment/`. Its
terms go into `protocols/vocabulary.yaml`.

## Not yet scoped

Draft. Scoped when its Canon capabilities exist: evaluation needs C-003, C-004, C-007 and C-008
(see the decision record). The record names the Canon gaps this protocol meets (independence for
`result.reproducible`, the superseding case of `superseded`, case inputs, composition).

## Protocol first

Per Atlas ADR 0080 for data-only changes: the fixture expectations first (red), then the YAML.

## Source

`architecture-decision-record:ml-protocol-shape` (wave 2026-10-04-w3); TASKBOARD E-009.
