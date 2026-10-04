---
format: aep.planning-md/3
id: story:protocol-docs-render
kind: story
status: draft
title: Render each built-in protocol as a workflow page in the Docusaurus site
summary: els protocols render turns canon-ir/1 into Markdown pages with Mermaid workflows in website/docs/protocols/, a Docusaurus site pinned to @beyond10x/docs-system; a drift test holds committed pages to a fresh render.
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
  path: crates/canon-engineering/src/lib.rs
- confidence: cited
  path: crates/canon-engineering/src/main.rs
- confidence: cited
  path: crates/canon-engineering/src/render.rs
- confidence: cited
  path: crates/canon-engineering/tests/protocol_docs_render.rs
- confidence: cited
  path: website/docs/protocols/
- confidence: cited
  path: website/docusaurus.config.ts
- confidence: cited
  path: website/package.json
- confidence: cited
  path: website/sidebars.ts
revision: 16
---
## Outcome

Each built-in protocol is rendered as a workflow page in ELS's public documentation site. A generator, `els protocols render` (a subcommand of the `els` binary that `story:protocol-registry` creates, Rust, clap derive), takes every built-in from the registry and compiles it through Canon to `canon-ir/1` (`b10x_canon::ir::compile`). It writes one Markdown page per built-in into the Docusaurus docs tree, at `website/docs/protocols/<name>/<major>.md`. Each page holds:

- Docusaurus front matter (`id`, `title`, `description`) and the protocol id, revision and description;
- a Mermaid workflow diagram, in a fenced `mermaid` block, generated from the IR, with each action linked to the evidence kinds it `may_produce`, each evidence kind linked to the claims whose `true_when` names it, and each claim linked to the outcomes whose `requires` names it;
- tables of the actions (with required capabilities), evidence kinds, claims, obligations and outcomes.

The page is generated from Canon's compiled form only, never from the YAML text, so it shows what Canon understood. A drift test fails when a committed page differs from a fresh render. Basis: Atlas ADR 0077 point 3 (draft, operator direction 2026-10-04).

### Publishing path: Docusaurus A

Operator decision "Docusaurus A" (relayed in the planning brief of 2026-10-04): ELS's public documentation site is Docusaurus pinned to `@beyond10x/docs-system`, the way AEP's is (aep `website/package.json:22` pins `@beyond10x/docs-system` to an exact Git commit; `website/docusaurus.config.ts:106,109` enable Mermaid through `@docusaurus/theme-mermaid`). The protocol pages are generated into that site's docs tree, not into a free-standing `docs/protocols/`. This supersedes the earlier path of this story (committed Markdown under `docs/protocols/`, read on GitHub, with Atlas `src/docs.rs` `INDEPENDENT_DOCUMENTATION_REPOSITORIES` as the reason).

ELS has no site today: no `website/`, no `b10x.docs.yaml`, and `.github/workflows/` holds only `check.yml` and `shared-gates.yml`. This story adds the minimal site the pages need: `website/package.json` pinning `@beyond10x/docs-system` to an exact commit and `@docusaurus/theme-mermaid`, `website/docusaurus.config.ts` with Mermaid on, and `website/sidebars.ts` listing the protocol pages. The generator is the only Rust; the site is configuration.

## Scope

- `crates/canon-engineering/src/render.rs` (new)
- `crates/canon-engineering/src/lib.rs` (the `render` module declaration)
- `crates/canon-engineering/src/main.rs` (the `protocols render` subcommand)
- `crates/canon-engineering/tests/protocol_docs_render.rs` (new)
- `website/docs/protocols/` (new: the generated pages)
- `website/package.json`, `website/docusaurus.config.ts`, `website/sidebars.ts` (new: the site scaffold)
- `README.md` (a link to the site's protocol pages; inferred)
- Out: the site's publication workflow and its registration with the unified site; hand-written site pages beyond the protocol pages; the `docs/design/` and `docs/examples/` Markdown, which stay where they are.

## Shared surface

This story edits `crates/canon-engineering/src/lib.rs` and `crates/canon-engineering/src/main.rs` after `story:protocol-registry`, and depends on it for the registry and the `els` binary.

It also depends on `story:stale-evidence-fixtures`, the last story that changes either built-in protocol document. A page rendered earlier would drift with every later protocol story, and each of those stories would have to re-render and own the page. Rendering after the chains keeps `website/docs/protocols/` this story's alone. Any later change to a protocol document re-renders its page, because the drift test fails otherwise.

## Protocol first

Atlas ADR 0080 (draft). The renderer adds no protocol data, so the specification change is none. The first commit adds `crates/canon-engineering/tests/protocol_docs_render.rs` alone. On that commit `protocol_pages_are_fresh_renders_of_canon_ir` fails because `els::render` and the `protocols render` subcommand do not exist and no page is committed, so the test does not build. The implementation commits add the renderer, the subcommand, the site scaffold and the generated pages.

## Canon capability

Compile to `canon-ir/1` (C-002, canon `story:canon-ir`, implemented). The renderer does not need Canon's evaluator.

## Acceptance

The test `protocol_pages_are_fresh_renders_of_canon_ir` in `crates/canon-engineering/tests/protocol_docs_render.rs` passes under `task check`. It expects:

1. For every (name, major) in `registry::list()`, rendering it gives bytes identical to the committed `website/docs/protocols/<name>/<major>.md`.
2. For each page, its front matter carries `id`, `title` and `description`, and its fenced `mermaid` block names every action, evidence kind, claim and outcome in that protocol's `canon-ir/1`, and draws an edge from each action to each evidence kind in its `may_produce`.
3. Starting from the text of `protocols/software-change/1.yaml` with one action added, the rendered page differs from the committed one, so drift in a built-in is detected.
4. Run as `CARGO_BIN_EXE_els`, `els protocols render --out <scratch>/docs/protocols` writes one page per built-in, each identical to the committed one, and writes nothing outside that directory.
5. `website/package.json` pins `@beyond10x/docs-system` to a 40-hex Git commit, not a branch or a range.

## Source

Atlas ADR 0077 point 3 (draft; operator direction 2026-10-04: each built-in is rendered as a workflow in ELS's public documentation, from Canon's compiled form, with a drift test); operator decision "Docusaurus A" (planning brief 2026-10-04); aep `website/package.json:22`, `website/docusaurus.config.ts:106,109`; `story:protocol-registry`.
