# Wave 2026-10-07-w1: project route inventory

Skill: `aep:implementing` 0.20.1, wave mode. One unit; no other story in the store is about
documentation publication.

| unit | objective | scope |
|---|---|---|
| `story:project-routes` | `vision:O2` | 3 cited, 2 inferred |

`aep plan artifact waves --kind story --status active`: `wave 1: story:project-routes (inferred)`,
0 collisions, 0 unassessed.

Commits this wave makes: the opening store commit, one unit commit, its merge into
`wave/2026-10-07-w1`, the closing store commit. The wave branch reaches `main` through one bot pull
request.

## Units

| unit | branch | head | worktree | build dir | scratch | stage |
|---|---|---|---|---|---|---|
| `story:project-routes` | `impl/project-routes` | `72e42a6` | `$HOME/.local/state/worktree/trees/b10x/engineering-protocols/impl-project-routes` | `<worktree>/target` | `$HOME/.cache/engineering-protocols-w1/project-routes` | merged `fa3ea07`; tree finished, archived |

Integration checkout: `$HOME/.local/state/worktree/trees/b10x/engineering-protocols/wave-2026-10-07-w1`,
branch `wave/2026-10-07-w1`, base `8544a0b`.

## Agents

| role | `subagent_type` | runs | tokens | tool uses | wall |
|---|---|---|---|---|---|
| implementor | `aep:implementor` | 3 (first, 2 corrections) | 78,788 + 122,189 + 168,897 | 42 + 30 + 26 | 284 s + 292 s + 286 s |
| adversary | `aep:adversary` | 2 passes | 108,892 + 153,676 | 40 + 19 | 405 s + 327 s |

## Record

| pass | findings | carried | outcome |
|---|---|---|---|
| adversary 1 | 4 (1 confirmed on the real build, 3 infeasible) | — | 4 fixed |
| adversary 2 | 6 (all infeasible on the real build) | 0 | 6 fixed; the correction was verified by the coordinator, not attacked |

Claim: `VERIFIED` on a real Docusaurus build of the unit: 42 HTML files = 19 pages + 5 refresh
redirects + 17 trailing-slash copies + `404.html`; the inventory lists the 19 routes, and parse5
finds the same anchors on each.

Gate on `fa3ea07`, one exit status per step: fmt 0, clippy 0, `cargo test --workspace --locked
--no-fail-fast` 0 (52 binaries, 189 passed, 0 failed; `--list` 189, every name present), `generate
--check` 0, `aep plan artifact validate` 0.
