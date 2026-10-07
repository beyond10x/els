---
format: aep.planning-md/3
id: blocker:canon-case-inputs
kind: blocker
status: open
title: Canon protocol/1 has no case inputs and no derived obligations
relations:
- blocks: story:software-change-profiles
revision: 1
---
## What is missing

Canon's `protocol/1` has no case-input section and no obligation derived by rule. The `Protocol` struct declares `artifacts`, `evidence_kinds`, `claims`, `obligations`, `actions`, `outcomes` and `invalidation` with `deny_unknown_fields`, and `Obligation` carries only a description and `discharged_when` (canon `crates/canon/src/model/mod.rs:55-75` and `:122-125`, canon checkout `main` at `761239f`, read 2026-10-07).

No artifact in the canon store mentions case inputs, and canon's one open issue, https://github.com/beyond10x/canon/issues/5, covers imports, the floor construct, `canon diff` and child case outcomes, not case inputs (read 2026-10-07).

## What it stops

`story:software-change-profiles`: a profile is the value of the case input `risk` or `change_kind`, and the risk profiles derive obligations from it. `story:ess-conformance-evidence` and `story:security-independence-rules` use case inputs too and depend on that story.

## Clears when

A Canon release declares case inputs and obligations derived from them in `protocol/1`, and this repository's Canon pin moves to it.
