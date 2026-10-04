---
format: aep.planning-md/3
id: story:repository-rename
kind: story
status: draft
title: The repository is beyond10x/engineering-protocols in GitHub, gates-policy and Atlas
relations:
- decomposes: epic:engineering-protocols-rename
revision: 1
---
## Outcome

`https://github.com/beyond10x/engineering-protocols` is the repository, and every registry that
names it uses the new name.

## Acceptance

- `gh repo view beyond10x/engineering-protocols` returns the repository; the rename was made by
  `b10x-bot[bot]` (`PATCH /repos/beyond10x/els` with `{"name":"engineering-protocols"}` through
  `b10x-gates api`).
- gates-policy names `beyond10x/engineering-protocols` and no longer `beyond10x/els`; the org secret
  `B10X_GATES_POLICY` is refreshed by its owner (agents stop and report at that step).
- A bot push and a bot PR to the renamed repository both succeed, and `common / Security and
  privacy` passes on that PR.
- Atlas catalog entry and the workspace guidance rosters (`atlas/src/workspace.rs`, `GUIDANCE`)
  name `engineering-protocols`.
- The local primary checkout is moved to `~/beyond10x/engineering-protocols` with its remote URL
  updated.

## Scope (inferred)

GitHub repository settings; `gates-policy/policy.json`; Atlas `catalog/`, `src/workspace.rs`.
