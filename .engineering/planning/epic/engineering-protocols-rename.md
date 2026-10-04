---
format: aep.planning-md/3
id: epic:engineering-protocols-rename
kind: epic
status: draft
title: Rename els to engineering-protocols and its crate to canon-engineering
relations:
- serves: vision:governed-autonomy
revision: 1
---
## Outcome

The repository `beyond10x/els` is named `engineering-protocols`, and its crate `b10x-els` is named
`b10x-canon-engineering`, so a reader can tell from either name that it holds engineering protocols
for Canon.

## Why

Operator request, 2026-10-05: "rename els repo to \"engineering-protocols\" and its inner crate from
els -> canon-engineering". The repository review of 2026-10-04 found the acronym hides what the
repository holds: readers meet `software-change@1` and cannot guess it lives in "ELS".

## What carries the old name today (els `ac7dd03`)

| Where | Name |
|---|---|
| `crates/els/Cargo.toml:2` | package `b10x-els` |
| `crates/els/Cargo.toml:9` | binary `els` |
| `crates/els-docs/Cargo.toml:2` | package `els-docs` |
| `governor/crates/governor/Cargo.toml:16` | `b10x-els`, git `beyond10x/els`, rev `ac7dd03` |
| `intake/crates/intake-router/Cargo.toml:14` | `els = { package = "b10x-els", … }` |
| `intake/crates/intake-slice/Cargo.toml:15` | `b10x-els`, git `beyond10x/els`, rev `ac7dd03` |
| `gates-policy/policy.json` | enrollment of `beyond10x/els` |
| workspace guidance (`atlas/src/workspace.rs`, `GUIDANCE`) | "ELS" in the independent-source and enrolled rosters |

## Stories

- `story:crate-rename`: in-repository rename of packages, library, binary and wording.
- `story:repository-rename`: GitHub rename and every registry that names the repository.
- `story:consumer-repin`: governor and intake depend on the renamed crate from the renamed
  repository.

## Not in scope

Changing any protocol, its identifier (`software-change@1`) or its content.
