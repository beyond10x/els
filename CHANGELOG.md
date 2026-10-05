# Changelog

All notable changes to this component are recorded here. Versions are component-scoped and released
under bare-version tags such as `0.1.0`. The workspace is `publish = false`; releases are source
releases at bare-version tags.

## [Unreleased]

### Added

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
