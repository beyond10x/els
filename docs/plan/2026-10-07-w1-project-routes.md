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
| `story:project-routes` | `impl/project-routes` | — | `$HOME/.local/state/worktree/trees/b10x/engineering-protocols/impl-project-routes` | `<worktree>/target` | `$HOME/.cache/engineering-protocols-w1/project-routes` | planned |

Integration checkout: `$HOME/.local/state/worktree/trees/b10x/engineering-protocols/wave-2026-10-07-w1`,
branch `wave/2026-10-07-w1`, base `8544a0b`.

## Agents

| role | `subagent_type` |
|---|---|
| implementor | `aep:implementor` |
| adversary | `aep:adversary` |
