---
format: aep.planning-md/3
id: review-result:adversary-w1-project-routes-pass-2
kind: review-result
status: active
title: Wave 2026-10-07-w1 adversary, story:project-routes, pass 2
relations:
- reviews: story:project-routes
revision: 1
---
```
unit: story:project-routes
verdict: red
cases: executed 42→48, red 6
origin: introduced 6, pre-existing 0, undecided 0
wrote-outside-worktree: $HOME/.cache/engineering-protocols-w1/project-routes/adversary/pass-2/{alone.log,suite.log,whatwg_ids.py,parse5_ids.mjs,parse5_inventory.mjs,probe_removal.sh,site-before.json,build-copy/}
needs-coordinator: no
```

Pass 2 is red with 6 new cases, but the real build reaches none of them, so all 6 are `INFEASIBLE` notes. The correction holds on the real build: parse5 finds the same ids as the inventory on all 19 routes, and the two banner ids from pass 1 are gone. Findings cover the uncommitted worktree on `impl/project-routes`, base `5d28957`.

**1. Diff stat**

`git --no-pager diff --stat` still shows only the implementor's tracked files (4 files, +24 −10). My only addition is the untracked `crates/canon-engineering-docs/tests/adversary_project_routes_pass2.rs`. I changed no non-test path.

**2. Cases added** (`$HOME/.local/state/worktree/trees/b10x/engineering-protocols/impl-project-routes/crates/canon-engineering-docs/tests/adversary_project_routes_pass2.rs`)

I wrote them first and ran them alone with `--test adversary_project_routes_pass2`: exit 101, 0 passed, 6 failed. All 6 are still red. For the 4 scanner cases, parse5 7.3.0 (from the worktree's `website/node_modules`) gives the expected anchors. html5lib agrees on 3; its etree builder does not separate template content.

| Case | Markup | Red output, verbatim |
|---|---|---|
| `adversary_p2_script_does_not_end_at_a_longer_end_tag_name` | `&lt;script&gt;var a="&lt;/scripty&gt;";var b='&lt;b id="leak"&gt;';&lt;/script&gt;` | `left: ["leak", "real"]  right: ["real"]` |
| `adversary_p2_an_abruptly_closed_empty_comment_hides_nothing` | `&lt;!--&gt;&lt;h2 id=real&gt;…&lt;!-- later --&gt;` | `left: []  right: ["real"]` |
| `adversary_p2_markup_inside_a_bogus_comment_is_no_element` | `&lt;![CDATA[&lt;a id="cdata"&gt;]]&gt;` | `left: ["cdata", "real"]  right: ["real"]` |
| `adversary_p2_ids_inside_a_template_are_not_rendered` | `&lt;template&gt;&lt;h2 id="inert"&gt;` | `left: ["inert", "real"]  right: ["real"]` |
| `adversary_p2_no_anchor_carries_a_control_character` | `id="tab\there"` | `anchor "tab\there" of "/engineering-protocols/" is refused by Website` |
| `adversary_p2_removing_an_earlier_manifest_deletes_nothing_outside_the_site` | `.well-known` is a symlink to an outside directory; the site has no landing route | `left: None  right: Some("not this site's\n")` |

**3. Gate, run after the cases existed**

`cargo test -p canon-engineering-docs --locked --no-fail-fast` exited 101. Per test file: 24, 3, 1, 4 (pass 1, now green), 4, 5 and 1 passed; `adversary_project_routes_pass2` had 0 passed and 6 failed. `-- --list` shows 48 tests.

**4. Findings**

| # | Location | What was measured | What reaches it | Verdict / origin |
|---|---|---|---|---|
| P1 | `src/routes.rs:151` | The script skip stops at the first `&lt;/script`, including `&lt;/scripty`. Browsers end a script only at `&lt;/script` followed by whitespace, `/` or `&gt;`. | Nothing found. No script body in the build contains `&lt;/scr`, `&lt;!--` or `&lt;script`. | INFEASIBLE / introduced, note |
| P2 | `src/routes.rs:127` | `&lt;!--&gt;` is a complete empty comment in HTML. The scanner keeps reading to the next `--&gt;` and hides the elements in between. | Nothing found. The build has no `&lt;!--&gt;`; its 48 comments are 31 empty React separators (`&lt;!-- --&gt;`) and 17 copy markers. | INFEASIBLE / introduced, note |
| P3 | `src/routes.rs:135` | After `&lt;!` or `&lt;?` (bogus comment or CDATA) it steps one byte and reads tags inside the comment. | Nothing found. 0 files contain `CDATA`. | INFEASIBLE / introduced, note |
| P4 | `src/routes.rs:149` | Ids inside `&lt;template&gt;` are listed, though template content is not part of the page. | Nothing found. 0 files contain `&lt;template` or `&lt;noscript`. | INFEASIBLE / introduced, note |
| P5 | `src/routes.rs:139` | An anchor holding a control character is published. Website rejects the whole site for it (`routes.rs:168-172`). | Nothing found. No anchor in the real inventory holds one. | INFEASIBLE / introduced, note |
| P6 | `src/manifest.rs:42` | The removal follows a symlinked `.well-known` and deletes `b10x-site.json` outside the site when the run is refused. | Nothing found. The build has no symlinks and `website/static` has no `.well-known`. | INFEASIBLE / introduced, note |

Fixes, which I did not apply:
- **P1:** end the skip only at `&lt;/script` followed by whitespace, `/` or `&gt;`.
- **P2:** treat `&lt;!--&gt;` and `&lt;!---&gt;` as a complete comment.
- **P3:** skip `&lt;!…&gt;` and `&lt;?…&gt;` up to the first `&gt;`.
- **P4:** skip `&lt;template&gt;` content.
- **P5:** refuse or drop control-character anchors.
- **P6:** refuse a `.well-known` that is not a real directory, checked with `symlink_metadata`, before removing anything.

**5. Attacked and not broken**
- **Real build:** re-running the current binary on a fresh copy gives byte-identical `b10x-routes.json` and `b10x-site.json`. Routes are the same 19 as in pass 1; only the 2 banner ids are gone.
- **Removal:** it deletes only the two fixed names. A symlinked `b10x-site.json` file loses its link and the target is kept (probe). A directory at either name is left in place, and the later write fails with no manifest (pass-1 case, now green).
- **Read, not run:**
  - `&lt;scripts&gt;` is correctly not skipped like `&lt;script&gt;`.
  - Tag names, attribute names and `&lt;/SCRIPT` are matched in any case.
  - `&gt;` inside quoted values is handled.
  - Attributes without values and an empty `id` are handled.
  - `data-id` and `xid` are not taken for `id`.
  - A BOM changes nothing.
  - An unterminated comment or script runs to end of file, as HTML does.
  - Slicing stays on UTF-8 character boundaries.
  - An id in an unterminated tag at end of file is kept, where HTML drops the tag.
  - Entities in ids are not decoded; the build has 0 `&amp;amp;`.
  - A site with no `index.html` is refused before the removal, so an earlier manifest stays. CI always builds fresh.
- **Not raised:** a refresh inside `&lt;noscript&gt;` makes a page count as a redirect. Acceptance 3 can be read either way.

**6. Paths written outside the worktree**

All under `$HOME/.cache/engineering-protocols-w1/project-routes/adversary/pass-2/`: `alone.log`, `suite.log`, `whatwg_ids.py`, `parse5_ids.mjs`, `parse5_inventory.mjs`, `probe_removal.sh`, `site-before.json`, and `build-copy/` (a copy of `website/build`). Inside the worktree, my failing cases left 4 fixture directories `target/tmp/adversary-p2-*`; I removed them. No worktree lease was taken, because the brief bans worktree commands.

```findings
[
  {"file": "crates/canon-engineering-docs/src/routes.rs", "line": 151, "category": "boundary", "severity": "note", "verdict": "INFEASIBLE", "origin": "introduced", "message": "The script skip ends at the first &lt;/script prefix such as &lt;/scripty, where HTML continues the script, so markup in the rest of the script string is read as anchors."},
  {"file": "crates/canon-engineering-docs/src/routes.rs", "line": 127, "category": "boundary", "severity": "note", "verdict": "INFEASIBLE", "origin": "introduced", "message": "&lt;!--&gt; is a complete empty comment in HTML, but scan searches on to the next --&gt; and drops the rendered elements in between."},
  {"file": "crates/canon-engineering-docs/src/routes.rs", "line": 135, "category": "boundary", "severity": "note", "verdict": "INFEASIBLE", "origin": "introduced", "message": "Bogus comments (&lt;![CDATA[, &lt;!x, &lt;?x) are stepped over one byte at a time, so start tags inside them, which create no element, are read as anchors."},
  {"file": "crates/canon-engineering-docs/src/routes.rs", "line": 149, "category": "acceptance", "severity": "note", "verdict": "INFEASIBLE", "origin": "introduced", "message": "Ids inside &lt;template&gt; content, which is not part of the page, are listed as anchors."},
  {"file": "crates/canon-engineering-docs/src/routes.rs", "line": 139, "category": "contract-drift", "severity": "note", "verdict": "INFEASIBLE", "origin": "introduced", "message": "An id holding a control character is published as an anchor, and Website's IndependentSites::validate rejects the whole site for it."},
  {"file": "crates/canon-engineering-docs/src/manifest.rs", "line": 42, "category": "boundary", "severity": "note", "verdict": "INFEASIBLE", "origin": "introduced", "message": "The earlier-manifest removal follows a symlinked .well-known and deletes b10x-site.json outside the site when the inventory refuses the site."}
]
```
