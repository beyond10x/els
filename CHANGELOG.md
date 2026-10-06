# Changelog

All notable changes to this component are recorded here. Versions are component-scoped and released
under bare-version tags such as `0.1.0`. The workspace is `publish = false`; releases are source
releases at bare-version tags.

## [Unreleased]

## [0.2.1] - 2026-10-07

Source release using Canon 0.1.0 and ESS 0.55.0. Protocol formats and the generated gate and
provider contracts are unchanged.

### Changed

- The ESS pin moves from 0.53.0 to 0.55.0 in `ess/` and `ess/providers/`, and CI installs 0.55.0.
  The regenerated contracts differ only in their generator stamp; source, contract and projection
  digests are unchanged.
- Builds go into each checkout's own `target/`: the Taskfile no longer sets `CARGO_TARGET_DIR`.

## [0.2.0] - 2026-10-05

Source release using Canon 0.1.0. Existing protocol formats remain unchanged.

### Added

- Typed assertion catalog and `gates validate/run/evaluate/explain`, backed by Canon's pure
  expression core. Collection retains source/context/implementation-bound observations for
  offline replay. Recipes and explicitly admitted Rust providers extend the catalog.
- File/document/schema, process/test, generated-tree, ESS/Codegate report, DNS and TCP providers;
  generated catalog reference and an assertion guide. AEP acceptance and Kubernetes remain planned.

- `support.triage/1`: triage one support ticket with tool-agnostic actions, step order by
  precondition (classify, review, then route) and outcomes `triaged`, `escalated` and `needs_human`;
  three fixtures, a `canon check` test and a concept page.

## [0.1.0] - 2026-10-05

The first release.

### Added

- `software.change/1` and `incident.response/1` on Canon, embedded in the crate and listed by the
  protocol registry (`canon_engineering::registry`).
- The engineering vocabulary, read from `protocols/vocabulary.yaml`.
- The fixture harness that compiles and evaluates each protocol's fixtures.
- The `canon-engineering` command line: list and show the built-in protocols.
- `canon-engineering-docs`: generated protocol pages and the documentation site manifest.

### Changed

- The crate is `b10x-canon-engineering` (library `canon_engineering`, binary `canon-engineering`),
  formerly `b10x-els`; the docs crate is `canon-engineering-docs`, formerly `els-docs`.
