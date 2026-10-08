# Wave 2026-10-08-w1: incident investigation obligation, Serves section

Skill: `aep:implementing` 0.22.2, wave mode. Release 0.3.0 follows the wave.

| unit | objective | scope |
|---|---|---|
| `story:incident-investigation-obligation` | `vision:O2` | 8 cited |
| `story:agents-serves-section` | `vision:O2` | 1 cited |

`aep plan artifact waves --status active`: `wave 1: story:agents-serves-section,
story:incident-investigation-obligation`, 0 collisions, 0 unassessed. `CHANGELOG.md` was the one
shared path; it left both scopes and is written by the release commit.

Commits this wave makes: the opening store commit, one or more unit commits per unit, their merges
into `wave/2026-10-08-w1`, the closing store commit, then the release commit. The wave branch
reaches `main` through one bot pull request.

## Units

| unit | branch | worktree | build dir | scratch | stage |
|---|---|---|---|---|---|
| `story:incident-investigation-obligation` | `impl/incident-investigation-obligation` | `$HOME/.local/state/worktree/trees/b10x/engineering-protocols/ep-impl-investigation` | `<worktree>/target` | `$HOME/.cache/engineering-protocols-w20261008/investigation` | dispatched |
| `story:agents-serves-section` | `impl/agents-serves-section` | `$HOME/.local/state/worktree/trees/b10x/engineering-protocols/ep-impl-serves` | none (docs only) | `$HOME/.cache/engineering-protocols-w20261008/serves` | dispatched |

Integration checkout: `$HOME/.local/state/worktree/trees/b10x/engineering-protocols/ep-wave-20261008-w1`,
branch `wave/2026-10-08-w1`, base `6e79372`.
