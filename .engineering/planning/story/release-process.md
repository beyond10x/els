---
format: aep.planning-md/3
id: story:release-process
kind: story
status: active
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
revision: 7
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T23:30:28Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-04T23:30:28Z", actor: "human:timo", revision: 3}
---
## Outcome

The repository has a release process: a tag-driven release workflow, a documented checklist in
`AGENTS.md`, and a first tag, so consumers pin `b10x-canon-engineering` by tag instead of by rev.

## Why

Adversary pass 1 on `story:crate-rename` (wave 2026-10-05-w21,
`review-result:adversary-w21-els-crate-rename-pass-1`): "The repository has no release process (no
release workflow, no tags, nothing in README or AGENTS)". `story:crate-rename` acceptance item 5 and
`story:consumer-repin` both assume a tag.

## Acceptance

- `AGENTS.md` names the release steps; a workflow builds and publishes a GitHub Release on a tag.
- The first tag exists after a green `task check` on `main`, with its GitHub Release; checks on the
  tag are green.
- Consumers can pin `tag = "<version>"`.
