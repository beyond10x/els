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
revision: 1
---
## Outcome

Governor and intake depend on `b10x-canon-engineering` from `beyond10x/engineering-protocols` by
release tag, and nothing in the chain resolves `b10x-els`.

## Acceptance

- `governor/crates/governor/Cargo.toml`, `intake/crates/intake-router/Cargo.toml` and
  `intake/crates/intake-slice/Cargo.toml` name `b10x-canon-engineering` from
  `https://github.com/beyond10x/engineering-protocols` by the tag `story:crate-rename` released.
- `cargo tree --locked -i b10x-els` fails with "package ID specification … did not match" in
  governor and intake; `cargo tree -i b10x-canon-engineering` shows exactly one copy.
- `task check` exits 0 in both repositories, and intake's offline slice test still ends
  `ApprovalRequired (repository.merge)`.

## Depends on

`story:crate-rename` (a tag to pin) and `story:repository-rename` (the URL).

## Scope (inferred)

governor: `crates/governor/Cargo.toml`, `src` imports of `els::`, `Cargo.lock`. intake:
`crates/intake-router`, `crates/intake-slice`, `Cargo.lock`.
