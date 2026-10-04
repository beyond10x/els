---
format: aep.planning-md/3
id: story:crate-rename
kind: story
status: active
title: The crate is b10x-canon-engineering and the project text says engineering protocols
relations:
- decomposes: epic:engineering-protocols-rename
- serves: vision:O2
scope:
- confidence: inferred
  path: .github/workflows/pages.yml
- confidence: inferred
  path: AGENTS.md
- confidence: inferred
  path: Cargo.lock
- confidence: inferred
  path: Cargo.toml
- confidence: inferred
  path: README.md
- confidence: inferred
  path: Taskfile.yml
- confidence: inferred
  path: crates/els
- confidence: inferred
  path: crates/els-docs
- confidence: inferred
  path: docs
- confidence: inferred
  path: website/docs
revision: 13
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T23:13:07Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-04T23:13:07Z", actor: "human:timo", revision: 3}
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
