---
format: aep.planning-md/3
id: story:release-process
kind: story
status: implemented
title: els has a tag-driven release process and a first tag
relations:
- decomposes: epic:engineering-protocols-rename
- serves: vision:O2
scope:
- confidence: inferred
  path: AGENTS.md
- confidence: inferred
  path: CHANGELOG.md
- confidence: inferred
  path: Cargo.lock
- confidence: inferred
  path: Cargo.toml
revision: 9
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T23:30:28Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-04T23:30:28Z", actor: "human:timo", revision: 3}
- {from: "active", to: "implemented", at: "2026-10-04T23:32:04Z", actor: "human:timo", revision: 9, decided_on: {"recorded":{"test_result":1,"verification":1}}}
---
## Outcome

The repository releases the way llm does: a release commit sets the workspace version and adds a
`CHANGELOG.md` entry, `b10x-bot[bot]` tags the merged commit with a bare version (`0.1.0`), and the
bot publishes the GitHub Release. Consumers then pin `b10x-canon-engineering` by `tag`.

## Why

Adversary pass 1 on `story:crate-rename` (wave 2026-10-05-w21,
`review-result:adversary-w21-els-crate-rename-pass-1`): "The repository has no release process (no
release workflow, no tags, nothing in README or AGENTS)". `story:crate-rename` acceptance item 5 and
`story:consumer-repin` both assume a tag. llm's process (llm `CHANGELOG.md`, release commit
`8fa8a15a`, tag `0.1.6`) has no release workflow; this story copies it.

## Acceptance

- `AGENTS.md` has a release section naming the steps and the completion rule (tag, required
  checks and GitHub Release verified).
- `CHANGELOG.md` exists with a `0.1.0` entry; the workspace version is `0.1.0`.
- Tag `0.1.0` (annotated, by `b10x-bot[bot]`) points at a `main` commit whose checks are green, and
  the GitHub Release `0.1.0` exists, authored by the bot.
