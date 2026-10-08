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
| `story:incident-investigation-obligation` | `impl/incident-investigation-obligation` | `$HOME/.local/state/worktree/trees/b10x/engineering-protocols/ep-impl-investigation` | `<worktree>/target` (1.5G at the first gate) | `$HOME/.cache/engineering-protocols-w20261008/investigation` | head `f039013`, merged `a713b39` |
| `story:agents-serves-section` | `impl/agents-serves-section` | `$HOME/.local/state/worktree/trees/b10x/engineering-protocols/ep-impl-serves` | none (docs only) | `$HOME/.cache/engineering-protocols-w20261008/serves` | head `1282049`, merged `fdffbae` |

Integration checkout: `$HOME/.local/state/worktree/trees/b10x/engineering-protocols/ep-wave-20261008-w1`,
branch `wave/2026-10-08-w1`, base `6e79372`.

## Agents

| role | `subagent_type` | runs |
|---|---|---|
| implementor, investigation | `aep:implementor` | 4 (first; scope extension to 5 tests that count states and terms; adversary fix; comment fix and gates after a disk stop) |
| implementor, Serves | `aep:implementor` | 1 |
| adversary, investigation | `aep:adversary` | 2 passes |

## Record

| pass | findings | outcome |
|---|---|---|
| adversary 1 | 1 introduced: a cause analysis of the release discharges `investigate_cause` while the text promised a service-only investigation | fixed by wording (option A); `cause.identified` keeps no subject binding, as decided for `story:incident-response-subject-binding` |
| adversary 2 | 2 introduced notes: the acceptance test's comment and the story's Outcome kept the old wording | fixed in `69d9661` and the story body |

The Serves unit's O1 line names the one ungated write action, `software.change/1`'s
`repository.edit`, instead of claiming every write action names a capability.

The first gate attempt on the fix stopped at 24G free on `/`, under the 25G floor; it resumed with
a 2G build budget and a 20G pause floor.

Package gates on `e7ae441` (the release commit), one exit status per step: `cargo fmt --all
--check` 0; `cargo clippy -p b10x-canon-engineering --all-targets --locked -- -D warnings` 0;
`cargo test -p b10x-canon-engineering --locked` 0 (121 passed, 0 failed; `--list` 121);
`cargo clippy -p canon-engineering-docs --all-targets --locked -- -D warnings` 0;
`cargo test -p canon-engineering-docs --locked` 0 (51 passed); `generate --check` 0 (16 files
fresh). The pull request's CI runs the full gate.
