---
format: aep.planning-md/3
id: story:protocol-registry
kind: story
status: draft
title: List and fetch built-in protocols from the crate and the els command line
summary: registry::list/get over embedded protocols/<name>/<major>.yaml, validated by Canon; els protocols list|show.
relations:
- decomposes: epic:els-first-domain
- depends_on: story:software-change-protocol
- depends_on: story:incident-response-protocol
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: inferred
  path: Cargo.lock
- confidence: inferred
  path: crates/els/Cargo.toml
- confidence: inferred
  path: crates/els/build.rs
- confidence: cited
  path: crates/els/src/lib.rs
- confidence: inferred
  path: crates/els/src/main.rs
- confidence: cited
  path: crates/els/src/registry.rs
- confidence: cited
  path: crates/els/tests/protocol_registry.rs
revision: 3
---
## Outcome

ELS ships its built-in protocols as data and lists them in a registry. A built-in is every file at `protocols/<name>/<major>.yaml`, embedded in the `b10x-els` crate when it is built, so adding a protocol file needs no Rust edit. `protocols/vocabulary.yaml` is not a protocol and is not listed. Basis: Atlas ADR 0077 point 3 (draft, operator direction 2026-10-04). Built-in protocols ship with ELS, must be expressible as data alone, and are listed and fetched by name and major version.

- **Library API** (`crates/els/src/registry.rs`):
  - `registry::list()` returns every built-in as (name, major).
  - `registry::get(name, major)` returns the YAML bytes exactly as released, together with Canon's validated `protocol/1` model of them (`b10x_canon::model::parse` and `b10x_canon::validate::validate`).
  - Both refuse an unknown name or major with an error naming it.
  - The registry holds no protocol definition of its own; it reads the data.
- **Command line**: an `els` binary with clap derive (`AGENTS.md` § Rules):
  - `els protocols list` prints each built-in as `<name>@<major>`.
  - `els protocols show <name>@<major>` prints its YAML.

## Scope

- `crates/els/src/registry.rs` (new)
- `crates/els/src/lib.rs` (the `registry` module declaration)
- `crates/els/src/main.rs` (new: the `els` binary; inferred)
- `crates/els/build.rs` (new: embeds `protocols/*/<major>.yaml`; inferred)
- `crates/els/Cargo.toml`, `Cargo.lock` (the `[[bin]]` entry and `clap`; inferred)
- `crates/els/tests/protocol_registry.rs` (new)

## Shared surface

The registry reads `protocols/` and edits nothing in it, so the protocol stories that keep editing `protocols/software-change/1.yaml` and `protocols/incident-response/1.yaml` after it do not touch its files. It does not block them.

It depends on `story:software-change-protocol` and `story:incident-response-protocol`, so that the first built-in protocol documents exist when its acceptance runs. It runs beside `story:software-change-profiles` and with no other story in flight on its files.

It edits `crates/els/Cargo.toml` and `Cargo.lock` after `story:vocabulary-yaml-source`, which comes before it through `story:fixture-harness` and the protocol stories. `story:protocol-docs-render` depends on this story and later edits `crates/els/src/lib.rs` and `crates/els/src/main.rs`.

## Protocol first

Atlas ADR 0080 (draft). The registry adds no protocol data; the documents it serves come from the protocol stories, so the specification change is none. The first commit adds `crates/els/tests/protocol_registry.rs` alone. On that commit `registry_lists_fetches_and_validates_every_builtin` fails because `els::registry` and the `els` binary do not exist, so the test does not build. The implementation commits add the registry, the build-time embedding and the command line.

## Canon capability

Parse and validate `protocol/1` (C-001, canon `story:protocol-source-model`, implemented). The registry does not need Canon's evaluator.

## Acceptance

The test `registry_lists_fetches_and_validates_every_builtin` in `crates/els/tests/protocol_registry.rs` passes under `task check`. It expects:

1. Starting from the repository tree, the test walks `protocols/` itself and collects every `protocols/<name>/<major>.yaml`. `registry::list()` returns exactly those (name, major) pairs, at least `software-change@1` and `incident-response@1`, and does not list `protocols/vocabulary.yaml`.
2. For each listed pair, `registry::get(name, major)` returns bytes identical to the file on disk and a Canon-validated model with no validation error.
3. `registry::get("no-such-protocol", 1)` is refused with a message naming `no-such-protocol`. `registry::get("software-change", 99)` is refused with a message naming `software-change@99`.
4. Run as `CARGO_BIN_EXE_els`:
   - `els protocols list` prints one line `<name>@<major>` per listed pair and exits 0;
   - `els protocols show software-change@1` prints the bytes of `protocols/software-change/1.yaml` and exits 0;
   - `els protocols show no-such-protocol@1` exits non-zero with a message naming `no-such-protocol@1`.

## Source

Atlas ADR 0077 point 3 (draft; operator direction 2026-10-04: built-ins are data, a registry lists and fetches them from the crate and its command line); `AGENTS.md` § Rules (clap derive).
