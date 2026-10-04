---
format: aep.planning-md/3
id: story:consumer-repin
kind: story
status: draft
title: Governor and intake depend on b10x-canon-engineering from engineering-protocols
relations:
- decomposes: epic:engineering-protocols-rename
- depends_on: story:crate-rename
- depends_on: story:repository-rename
revision: 2
---
## Outcome

Every consumer depends on `b10x-canon-engineering` from `beyond10x/engineering-protocols` by
release tag, and nothing in the chain resolves `b10x-els`.

## Consumers

Today (els `ac7dd03`): `governor/crates/governor/Cargo.toml:16`,
`intake/crates/intake-router/Cargo.toml:14`, `intake/crates/intake-slice/Cargo.toml:15`.

Atlas ADR 0090 (operator, 2026-10-05) moves governor and intake into `beyond10x/loom`
(loom `epic:runtime-consolidation`). This story repins wherever those crates live when it runs:
Loom once loom `story:import-intake` has landed, otherwise governor and intake.

## Acceptance

- Each consumer crate names `b10x-canon-engineering` from
  `https://github.com/beyond10x/engineering-protocols` by the tag `story:crate-rename` released.
- `cargo tree --locked -i b10x-els` fails with "did not match" in every consuming workspace;
  `cargo tree -i b10x-canon-engineering` shows exactly one copy.
- `task check` exits 0 in every consuming repository, and intake's offline slice test still ends
  `ApprovalRequired (repository.merge)`.

## Depends on

`story:crate-rename` (a tag to pin) and `story:repository-rename` (the URL).

## Scope (inferred)

The consuming crates' `Cargo.toml`, their `els::` imports, `Cargo.lock`.
