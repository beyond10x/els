---
format: aep.planning-md/3
id: story:project-routes
kind: story
status: active
title: The Pages site publishes its route inventory at .well-known/b10x-routes.json
summary: 'canon-engineering-docs site-manifest also writes b10x-project-routes/v1 into the built site: every canonical page route under /engineering-protocols/ with its element IDs, stamped with the commit b10x-site.json carries.'
relations:
- serves: vision:O2
scope:
- confidence: inferred
  path: .github/workflows/pages.yml
- confidence: cited
  path: AGENTS.md
- confidence: inferred
  path: crates/canon-engineering-docs/src/main.rs
- confidence: cited
  path: crates/canon-engineering-docs/src/manifest.rs
- confidence: cited
  path: crates/canon-engineering-docs/src/routes.rs
revision: 8
transitions:
- {from: "draft", to: "proposed", at: "2026-10-07T00:04:49Z", actor: "human:timo", revision: 7}
- {from: "proposed", to: "active", at: "2026-10-07T00:04:49Z", actor: "human:timo", revision: 8}
---
## Outcome

The Pages site publishes its route inventory, `/engineering-protocols/.well-known/b10x-routes.json`
(`b10x-project-routes/v1`), beside `.well-known/b10x-site.json`. Website reads it to list the
project and check links into it (`beyond10x/website` `tools/website/src/routes.rs`,
`IndependentSites::load` and `validate`), and Atlas's documentation publish checks its identity
(`beyond10x/atlas` `src/docs_portal.rs:1107-1112`). Today `AGENTS.md:98` says the site publishes no
route inventory.

The document is written by `canon-engineering-docs site-manifest`, which `pages.yml` already runs on
`website/build` before uploading it (`.github/workflows/pages.yml`, step "Declare what was built").

## Acceptance

1. `canon-engineering-docs site-manifest --site <built site> --commit <sha>` writes
   `.well-known/b10x-routes.json` beside `.well-known/b10x-site.json`. Its top-level keys are exactly
   `schema` = `b10x-project-routes/v1`, `repository` = `engineering-protocols`, `baseUrl` =
   `/engineering-protocols/`, `commit` = the commit written into `b10x-site.json`, and `routes`.
2. `routes` holds one entry `{"path", "anchors"}` per page Docusaurus built, sorted by `path`. Every
   `path` starts with `/engineering-protocols/` and ends with `/`; no `path` appears twice; the
   landing route `/engineering-protocols/` is present. `anchors` are the page's rendered `id`
   attribute values, sorted and unique.
3. These produce no route: `404.html`; a client redirect page (a `http-equiv="refresh"` page, which
   `@docusaurus/plugin-client-redirects` writes for `/protocols/…` and `/vocabulary`); and a
   trailing-slash copy at `x/index.html` marked `<!-- b10x-trailing-slash-copy -->`, which
   `@beyond10x/docs-system` writes for every `x.html` (`src/product-site.ts`,
   `writeTrailingSlashRedirects`, at the pinned commit `9d35e63`). A page `x.html` is listed as
   `/engineering-protocols/x/`.
4. `site-manifest` refuses a built site whose inventory has no landing route, and two built files
   that answer one route.
5. Tests in `crates/canon-engineering-docs` check 1–4 on a fixture tree holding a landing page, a
   page `x.html` with its marked copy `x/index.html`, a directory index, a redirect page and
   `404.html`.
6. After the merge to `main`, `https://beyond10x.github.io/engineering-protocols/.well-known/b10x-routes.json`
   answers 200, and its `commit` equals the `commit` of the deployed `.well-known/b10x-site.json`.

## Scope

- `crates/canon-engineering-docs/src/routes.rs` (new): the inventory. Cited: the request names
  `canon-engineering-docs site-manifest` as the place.
- `crates/canon-engineering-docs/src/manifest.rs`: `write` writes the inventory with the manifest,
  from the same `commit`. Cited (`manifest.rs:33-45`).
- `crates/canon-engineering-docs/src/main.rs`: the `SiteManifest` doc comment and `mod routes`.
  Inferred.
- `AGENTS.md`: line 98 states the site publishes no `.well-known/b10x-routes.json`. Cited.
- `.github/workflows/pages.yml`: no change expected; the step already runs `site-manifest` and
  uploads with `include-hidden-files: true`. Inferred.

## Producer

Two producers of this document exist. `beyond10x/metaharness` `crates/metaharness-docs/src/validate.rs`
(`inventory`) reads a hand-rendered site where every page is `index.html`. `beyond10x/secrets`
`crates/secrets-docs/src/routes.rs` reads a Docusaurus build with `trailingSlash: false` and the same
`@beyond10x/docs-system` pin as this site (`website/package.json:18`, `9d35e63`), and skips redirect
pages and trailing-slash copies. This site is that case (`website/docusaurus.config.ts`:
`trailingSlash: false`, `@docusaurus/plugin-client-redirects`), so the route rules follow
`secrets-docs`; the document shape is the same in both.

## Spec first

Not modelled in ESS. `b10x-project-routes/v1` is Website's format (`tools/website/src/routes.rs`,
`IndependentPage`), and this repository's specification under `ess/` models assertion collection
only (`ess/ess-inputs.yaml`: `system.yaml`, `domains/gates.yaml`, `domains/expr.yaml`). The other
publication outputs of `canon-engineering-docs` (`b10x-site.json`, `status.json`, the protocol
graphs) are not modelled there either. Declaring Website's type here would be a second, hand-made
copy of a format another repository owns.

## Protocol first

Exempt: no protocol, vocabulary or fixture changes behaviour. The change is documentation
publication.
