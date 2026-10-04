---
format: aep.planning-md/3
id: story:protocol-docs-render
kind: story
status: draft
title: Render each built-in protocol as a workflow page under docs/
summary: els protocols render turns canon-ir/1 into a Markdown page with a Mermaid workflow; a drift test holds committed pages to a fresh render.
relations:
- decomposes: epic:els-first-domain
- depends_on: story:protocol-registry
- depends_on: story:stale-evidence-fixtures
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: inferred
  path: README.md
- confidence: cited
  path: crates/els/src/lib.rs
- confidence: cited
  path: crates/els/src/main.rs
- confidence: cited
  path: crates/els/src/render.rs
- confidence: cited
  path: crates/els/tests/protocol_docs_render.rs
- confidence: cited
  path: docs/protocols/
revision: 3
---
## Outcome

Each built-in protocol is rendered as a workflow page in ELS's public documentation. A generator, `els protocols render` (a subcommand of the `els` binary that `story:protocol-registry` creates, clap derive), takes every built-in from the registry and compiles it through Canon to `canon-ir/1` (`b10x_canon::ir::compile`). It writes one page per built-in to `docs/protocols/<name>/<major>.md`. Each page holds:

- the protocol id, revision and description;
- a Mermaid workflow diagram generated from the IR, with each action linked to the evidence kinds it `may_produce`, each evidence kind linked to the claims whose `true_when` names it, and each claim linked to the outcomes whose `requires` names it;
- tables of the actions (with required capabilities), evidence kinds, claims, obligations and outcomes.

The page is generated from Canon's compiled form only, never from the YAML text, so it shows what Canon understood. A drift test fails when a committed page differs from a fresh render. Basis: Atlas ADR 0077 point 3 (draft, operator direction 2026-10-04).

### Publishing path

ELS has no unified-site publication. It carries no `b10x.docs.yaml`, and its `.github/workflows/` holds only `check.yml` and `shared-gates.yml`, so it has no Pages workflow. In Atlas `src/docs.rs`, `INDEPENDENT_DOCUMENTATION_REPOSITORIES` at Atlas `origin/main` `4f3cbd6e` is `["uilab"]`. The unmerged Atlas commit `93850c49` adds `els` to that list ("documented in their own repositories until a site is decided", ADR 0066), which makes a repository's README and `docs/` its documentation. The rendered pages are therefore published as committed Markdown under `docs/protocols/` in this repository, linked from `README.md`, where GitHub renders the Mermaid diagram. No site publication is part of this story.

## Scope

- `crates/els/src/render.rs` (new)
- `crates/els/src/lib.rs` (the `render` module declaration)
- `crates/els/src/main.rs` (the `protocols render` subcommand)
- `crates/els/tests/protocol_docs_render.rs` (new)
- `docs/protocols/` (new: the generated pages)
- `README.md` (a link to `docs/protocols/`; inferred)

## Shared surface

This story edits `crates/els/src/lib.rs` and `crates/els/src/main.rs` after `story:protocol-registry`, and depends on it for the registry and the `els` binary.

It also depends on `story:stale-evidence-fixtures`, the last story that changes either built-in protocol document. A page rendered earlier would drift with every later protocol story, and each of those stories would have to re-render and own the page. Rendering after the chains keeps `docs/protocols/` this story's alone. Any later change to a protocol document re-renders its page, because the drift test fails otherwise.

## Protocol first

Atlas ADR 0080 (draft). The renderer adds no protocol data, so the specification change is none. The first commit adds `crates/els/tests/protocol_docs_render.rs` alone. On that commit `protocol_pages_are_fresh_renders_of_canon_ir` fails because `els::render` and the `protocols render` subcommand do not exist and no page is committed, so the test does not build. The implementation commits add the renderer, the subcommand and the generated pages.

## Canon capability

Compile to `canon-ir/1` (C-002, canon `story:canon-ir`, implemented). The renderer does not need Canon's evaluator.

## Acceptance

The test `protocol_pages_are_fresh_renders_of_canon_ir` in `crates/els/tests/protocol_docs_render.rs` passes under `task check`. It expects:

1. For every (name, major) in `registry::list()`, rendering it gives bytes identical to the committed `docs/protocols/<name>/<major>.md`.
2. For each page, its Mermaid block names every action, evidence kind, claim and outcome in that protocol's `canon-ir/1`, and draws an edge from each action to each evidence kind in its `may_produce`.
3. Starting from the text of `protocols/software-change/1.yaml` with one action added, the rendered page differs from the committed one, so drift in a built-in is detected.
4. Run as `CARGO_BIN_EXE_els`, `els protocols render` into a scratch directory writes one page per built-in, and each page is identical to the committed one.

## Source

Atlas ADR 0077 point 3 (draft; operator direction 2026-10-04: each built-in is rendered as a workflow in ELS's public documentation, from Canon's compiled form, with a drift test); Atlas `src/docs.rs` `INDEPENDENT_DOCUMENTATION_REPOSITORIES` (`origin/main` `4f3cbd6e`; unmerged commit `93850c49`); `story:protocol-registry`.
