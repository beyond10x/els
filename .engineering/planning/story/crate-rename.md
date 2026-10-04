---
format: aep.planning-md/3
id: story:crate-rename
kind: story
status: draft
title: The crate is b10x-canon-engineering and the project text says engineering protocols
relations:
- decomposes: epic:engineering-protocols-rename
revision: 1
---
## Outcome

Inside the repository, the crate is `b10x-canon-engineering` (library `canon_engineering`), the
docs crate is `canon-engineering-docs`, the binary is `canon-engineering`, and human-facing text
calls the project "engineering protocols" rather than "ELS".

## Acceptance

- `cargo metadata --no-deps` lists `b10x-canon-engineering` and `canon-engineering-docs`, and no
  package named `b10x-els` or `els-docs`.
- `cargo build` produces a binary `canon-engineering`; no binary `els`.
- `grep -rniw 'els'` over `README.md`, `AGENTS.md`, `docs/`, `website/` and `crates/` returns no
  hit naming the project (each remaining hit listed and justified in the PR).
- `task check` exits 0.
- A release is cut through the repository's own release process, so consumers can pin a tag.

## Scope (inferred)

`Cargo.toml`, `Cargo.lock`, `crates/els/` → `crates/canon-engineering/`, `crates/els-docs/` →
`crates/canon-engineering-docs/`, `Taskfile.yml`, `README.md`, `AGENTS.md`, `docs/`, `website/`.
