---
format: aep.planning-md/3
id: review-result:adversary-w1-project-routes-pass-1
kind: review-result
status: active
title: Wave 2026-10-07-w1 adversary, story:project-routes, pass 1
relations:
- reviews: story:project-routes
revision: 1
---
```
unit: story:project-routes
verdict: red
cases: executed 36→40, red 4
origin: introduced 4, pre-existing 0, undecided 0
wrote-outside-worktree: $HOME/.cache/engineering-protocols-w1/project-routes/adversary/{check_ids.py,probe_symlink.sh,alone.log,suite.log,build-copy/}
needs-coordinator: no
```

The suite is red with 4 new cases. One of them is reached by the real build: the landing route lists 2 anchors that are not rendered elements. The other 3 fail only on fixtures I built, so their verdict is `INFEASIBLE`. Findings cover the uncommitted worktree on branch `impl/project-routes`, base `5d28957`.

**1. Diff stat**

`git --no-pager diff --stat` shows only the implementor's tracked changes: `.github/workflows/pages.yml`, `AGENTS.md`, `src/main.rs` and `src/manifest.rs` (4 files, +16 −9). I added one untracked file, `crates/canon-engineering-docs/tests/adversary_project_routes.rs`, which `git status` lists as `??`. I changed no non-test path.

**2. Cases added** (`$HOME/.local/state/worktree/trees/b10x/engineering-protocols/impl-project-routes/crates/canon-engineering-docs/tests/adversary_project_routes.rs`)

I wrote them first and ran them alone with `cargo test -p canon-engineering-docs --locked --test adversary_project_routes`, which exited 101 with 0 passed and 4 failed. All 4 are still red.

| Case | Asserts | Red output, verbatim |
|---|---|---|
| `adversary_landing_anchors_exclude_ids_inside_a_script` (:82) | Uses the banner `&lt;script&gt;` copied verbatim from `website/build/index.html`. Expects landing anchors to be only the rendered ids. | `left: ["__docusaurus", "__docusaurus-base-url-issue-banner", "__docusaurus-base-url-issue-banner-suggestion-container", "__docusaurus_skipToContent_fallback", "b10x-hero-title"]` |
| `adversary_an_id_shown_in_inline_code_is_no_anchor` (:111) | `&lt;code&gt;&amp;lt;h2 id="shown"&amp;gt;&lt;/code&gt;` should give no anchor | `left: ["real", "shown"]  right: ["real"]` |
| `adversary_a_page_that_mentions_a_refresh_redirect_is_still_a_route` (:142) | A real page that shows `http-equiv="refresh"` inside `&lt;code&gt;` should still be a route | `left: None  right: Some(["redirects"])` |
| `adversary_a_failed_inventory_write_leaves_no_site_manifest` (:164) | If writing `b10x-routes.json` fails, `b10x-site.json` should not be left behind | `a site-manifest that wrote no inventory leaves no site manifest either` |

**3. Gate, run after the cases existed**

`cargo test -p canon-engineering-docs --locked --no-fail-fast` exited 101. Per test file: 24, 3, 1, 4, 3 and 1 passed; `adversary_project_routes` had 0 passed and 4 failed. The output ends with `error: 1 target failed: -p canon-engineering-docs --test adversary_project_routes`. `-- --list` shows 40 tests. The 36 before is that same run without my file.

**4. Findings**

| # | Location | What was measured | What reaches it | Verdict / origin |
|---|---|---|---|---|
| F1 | `src/routes.rs:33` | `ids()` looks for the text ` id=` anywhere in the file, including inside a JavaScript string. | **The real build.** I re-ran the current binary on a copy of `website/build`; the output is byte-identical to the existing `b10x-routes.json`. Route `/engineering-protocols/` lists `__docusaurus-base-url-issue-banner` and `…-suggestion-container`. An HTML parser compared on all 19 routes finds only these 2 extra. Acceptance 2 ("rendered `id` attribute values") does not hold. Website would accept links to these two anchors, which do not exist on the live page. | CONFIRMED / introduced, warning |
| F2 | `src/routes.rs:33` | Same scan. An id shown inside `&lt;code&gt;` becomes an anchor, because the minified build writes `"` unescaped in text (seen in `docs/guides/assertions.html`). | Nothing found. No page in the real build has ` id=` in its text (parser comparison). | INFEASIBLE / introduced, note |
| F3 | `src/routes.rs:24` | `is_redirect` also matches `http-equiv="refresh"` in body text. A real page that mentions it drops out of the inventory with exit 0 and no error. | Nothing found. Only the 5 real redirect pages contain `http-equiv`. | INFEASIBLE / introduced, note |
| F4 | `src/manifest.rs:43-46` | `b10x-site.json` is written before `b10x-routes.json`. If the second write fails, the site is left declared with no inventory beside it, or with one carrying an older commit. | Nothing found. The CI step then fails, so the upload step never runs on that build. | INFEASIBLE / introduced, note |

Fixes, which I did not apply:
- **F1 and F2:** read `id` only inside start tags, and skip the contents of `&lt;script&gt;` and `&lt;style&gt;`.
- **F3:** match only a `&lt;meta http-equiv=refresh&gt;` tag in `&lt;head&gt;`.
- **F4:** write `b10x-routes.json` first and `b10x-site.json` last.

The model producer, `secrets-docs` `routes.rs`, behaves the same way for F1–F3.

**5. Attacked and not broken**
- **Redirects and copies on the real build:** of 42 `.html` files, 23 are excluded (404 plus 17 marked copies plus 5 refresh pages) and 19 are routes. Every one of the 17 `x.html` pages has its `x/index.html` copy.
- **Consumer rules:** the real inventory passes Website's `validate` rules, re-implemented in Python (no bad paths or anchors, landing present, sorted). Atlas's identity fields match.
- **Refused sites:** nothing is written before the inventory is computed, which the implementor's suite already tests.
- **Determinism:** the output is the same across runs.
- **Mutants on the implementor's suite:** dropping any of the three `is_redirect` branches, the landing check, the `index.html` rule or unquoted id parsing all turn its suite red.
- **Symlinks:** a symlinked directory is silently skipped (probe). The real build has no symlinks.
- **Non-UTF-8 names:** a non-UTF-8 asset name, not just a page, makes the run fail with exit 1 (probe). The real build has none.
- **HTML entities in ids:** not decoded (read from the code). No id in the real build contains one.
- **`pages.yml`:** dropping the `if:` makes the inventory check run on PRs too. It is beyond the story's "no change expected" but breaks nothing.

**6. Paths written outside the worktree**

All under `$HOME/.cache/engineering-protocols-w1/project-routes/adversary/`: `check_ids.py`, `probe_symlink.sh`, `alone.log`, `suite.log`, and `build-copy/` (3.1M, a copy of `website/build`). Inside the worktree, my failing tests left 8 fixture directories `target/tmp/adversary-project-routes-*`; I removed them. I took no worktree lease, because the brief bans worktree commands.

```findings
[
  {"file": "crates/canon-engineering-docs/src/routes.rs", "line": 33, "category": "acceptance", "severity": "warning", "verdict": "CONFIRMED", "origin": "introduced", "message": "ids() lists two id strings from the landing page's base-URL banner &lt;script&gt; as anchors of /engineering-protocols/ in the real build, though no such element is rendered."},
  {"file": "crates/canon-engineering-docs/src/routes.rs", "line": 33, "category": "boundary", "severity": "note", "verdict": "INFEASIBLE", "origin": "introduced", "message": "An id shown in inline code text (&lt;code&gt;&amp;lt;h2 id=\"x\"&amp;gt;&lt;/code&gt;) becomes an anchor; no page in the real build has one."},
  {"file": "crates/canon-engineering-docs/src/routes.rs", "line": 24, "category": "boundary", "severity": "note", "verdict": "INFEASIBLE", "origin": "introduced", "message": "is_redirect matches http-equiv=\"refresh\" in body text, so a real page that mentions it is silently left out of the inventory; no such page in the real build."},
  {"file": "crates/canon-engineering-docs/src/manifest.rs", "line": 43, "category": "concurrency", "severity": "note", "verdict": "INFEASIBLE", "origin": "introduced", "message": "b10x-site.json is written before b10x-routes.json, so a failed inventory write leaves a declared site with no or stale inventory; CI stops before the upload."}
]
```
