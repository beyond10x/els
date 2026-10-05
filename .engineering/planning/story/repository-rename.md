---
format: aep.planning-md/3
id: story:repository-rename
kind: story
status: active
title: The repository is beyond10x/engineering-protocols in GitHub, gates-policy and Atlas
relations:
- decomposes: epic:engineering-protocols-rename
- serves: vision:O2
scope:
- confidence: inferred
  path: .github/workflows/b10x-docs-site.yml
- confidence: inferred
  path: AGENTS.md
- confidence: inferred
  path: Cargo.toml
- confidence: inferred
  path: crates/canon-engineering-docs/src/manifest.rs
- confidence: inferred
  path: website/docusaurus.config.ts
revision: 8
transitions:
- {from: "draft", to: "proposed", at: "2026-10-05T08:31:00Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-05T08:31:00Z", actor: "human:timo", revision: 3}
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
