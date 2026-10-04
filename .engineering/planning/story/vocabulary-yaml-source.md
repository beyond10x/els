---
format: aep.planning-md/3
id: story:vocabulary-yaml-source
kind: story
status: draft
title: Move the engineering vocabulary to protocols/vocabulary.yaml
summary: The 35 vocabulary terms become released YAML data; vocabulary.rs becomes its typed reader.
refs:
- provider: taskboard
  reference: E-001
relations:
- decomposes: epic:els-first-domain
- depends_on: story:els-vocabulary
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: inferred
  path: Cargo.lock
- confidence: inferred
  path: crates/els/Cargo.toml
- confidence: cited
  path: crates/els/src/vocabulary.rs
- confidence: cited
  path: crates/els/tests/vocabulary_yaml.rs
- confidence: cited
  path: protocols/vocabulary.yaml
revision: 3
---
## Outcome

The engineering vocabulary is data. `protocols/vocabulary.yaml` holds the 35 terms `story:els-vocabulary` declared in `crates/els/src/vocabulary.rs` (at `13d180f`), each with the same id, category, marking and meaning, and `crates/els/src/vocabulary.rs` becomes the typed reader of that file. The Rust API it already offers (`terms`, `lookup`, `Term`, `Category`, `Marking`, `UnknownTerm`, `Term::claim_id`, `Term::action_id`) reads the YAML and holds no term of its own. What ELS releases is the YAML; the reader embeds the released file when the crate is built, and also reads a vocabulary document given as text, so a term added only to the YAML appears through the API without a Rust edit.

Basis: Atlas ADR 0077 point 3 (draft, operator decision 2026-10-04). A protocol or the vocabulary may be written as YAML data or produced in Rust through a typed API that yields the same document; ELS releases the data, and the Rust API reads or produces it and defines nothing itself.

`protocols/vocabulary.yaml` sits beside the protocol directories but is not a protocol. It is not named `protocols/<name>/<major>.yaml`, so `story:protocol-registry` does not list it.

The story adds the YAML parsing dependencies to `crates/els/Cargo.toml` (inferred: `serde` with `derive` and `serde_yaml_ng` 0.10, the pair Canon uses in canon `crates/canon/Cargo.toml`). `story:fixture-harness` reads its fixture files with them.

## Scope

- `protocols/vocabulary.yaml` (new)
- `crates/els/src/vocabulary.rs` (rewritten as the reader)
- `crates/els/tests/vocabulary_yaml.rs` (new)
- `crates/els/Cargo.toml`, `Cargo.lock` (inferred: the YAML dependencies)

## Shared surface

`protocols/vocabulary.yaml` is created here. Later stories add their terms to it in this order: `story:software-change-profiles` → `story:software-change-negative-outcomes` → `story:ess-conformance-evidence` → `story:security-independence-rules` → `story:rollback-verification-rule`. `crates/els/src/vocabulary.rs` is rewritten here, and later `story:software-change-profiles`, `story:software-change-negative-outcomes` and `story:security-independence-rules` add to it the term categories their terms need. `crates/els/Cargo.toml` and `Cargo.lock` are edited here and later by `story:protocol-registry`, which comes after it through `story:fixture-harness` and the protocol stories. `story:fixture-harness` depends on this story. This story does not edit `crates/els/tests/vocabulary.rs` or `crates/els/tests/adversary_vocabulary.rs`.

## Protocol first

Atlas ADR 0080 (draft). The first commit adds `protocols/vocabulary.yaml` with the 35 terms, and `crates/els/tests/vocabulary_yaml.rs`. On that commit `vocabulary_yaml_is_the_source` fails at item 3: `crates/els/src/vocabulary.rs` still holds its own term list and has no reader, so a term added only to the YAML does not appear through the API. The implementation commit turns `vocabulary.rs` into the reader and does not touch the YAML.

## Domain relations

- ELS vocabulary → ELS protocol: one-to-many; every term a protocol names is declared in the vocabulary once. Inferred from `crates/els/src/vocabulary.rs:1-8` (module doc) and `crates/els/tests/vocabulary.rs`. No ess/1 document declares it, because ELS opts out of ESS for protocol semantics (`AGENTS.md` § ESS).

## Acceptance

The test `vocabulary_yaml_is_the_source` in `crates/els/tests/vocabulary_yaml.rs` passes under `task check`. It expects:

1. Starting from `protocols/vocabulary.yaml`, the file holds exactly 35 entries, and their (id, category, marking, meaning) equal the 35 terms of `crates/els/src/vocabulary.rs` at `13d180f`. The test carries that list as its expected value.
2. Starting from the same file, `vocabulary::terms()` returns exactly those 35 entries in file order, and `vocabulary::lookup` finds each of them.
3. Starting from the text of `protocols/vocabulary.yaml` with one term appended (`example_term`, an evidence kind, marked core), reading that text through the same typed reader returns 36 entries and `lookup("example_term")` finds the new one. No Rust source changes.
4. Starting from this story's tree, `crates/els/tests/vocabulary.rs` and `crates/els/tests/adversary_vocabulary.rs` pass unedited.

## Source

Operator decision 2026-10-04, Atlas ADR 0077 point 3 (draft); `story:els-vocabulary`; `crates/els/src/vocabulary.rs` at `13d180f`.
