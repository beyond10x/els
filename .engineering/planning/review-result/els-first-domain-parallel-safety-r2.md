---
format: aep.planning-md/3
id: review-result:els-first-domain-parallel-safety-r2
kind: review-result
status: active
title: els-first-domain decomposition — parallel-safety critic, round 2
relations:
- reviews: epic:els-first-domain
- reviews: story:els-vocabulary
- reviews: story:ess-conformance-evidence
- reviews: story:incident-response-protocol
- reviews: story:ml-protocol-shape-decision
- reviews: story:security-independence-rules
- reviews: story:software-change-negative-outcomes
- reviews: story:software-change-profiles
- reviews: story:software-change-protocol
- reviews: story:stale-evidence-fixtures
revision: 1
---
approve

**What I read:** 9 stories, the epic and the 4 round-1 results, via `aep plan artifact list`, `graph`, `waves --kind story` and `show` on every story. Also `review-result:els-first-domain-parallel-safety-r1`, `crates/els/Cargo.toml`, `crates/els/src/lib.rs`, `Cargo.toml` and `Taskfile.yml`. Surfaces: 9 cited, 0 inferred, 0 unplaceable. `waves` reports "7 wave(s), 55 collision(s), 0 unassessed". Wave 1 (`story:els-vocabulary`, `story:ml-protocol-shape-decision`) and wave 2 (`story:incident-response-protocol`, `story:software-change-protocol`) each share no scoped path. All 55 collisions are between stories joined by `depends_on` edges, directly or through a chain. Every body's "Shared surface" section names the chain, and the chains in the graph match the bodies.

All seven round-1 findings have landed:
- The blocker, no path for the protocol source, is fixed by typed scopes on every story.
- `crates/els/src/lib.rs` and `crates/els/src/protocols/mod.rs` are now owned by `story:els-vocabulary`.
- `crates/els/tests/support/mod.rs`, the fixture harness, is also owned by `story:els-vocabulary`.
- `crates/els/src/vocabulary.rs` is owned by `story:els-vocabulary`, with the later additions listed per story.
- The software.change source and fixtures are now one explicit chain.
- The incident source and fixtures are chained through `story:security-independence-rules` to `story:stale-evidence-fixtures`.

**What I could not establish:**
- `crates/els/Cargo.toml` and the workspace `Cargo.lock` are in no story's scope. The harness has to parse fixtures and `story:ess-conformance-evidence` has to read `ess-conformance-report/2`. Whether either needs a new dependency depends on what `b10x-canon` (a git dependency) exports, and I did not inspect that. If one does, the two wave-2 stories would collide on those files unnoticed. I could not establish a need, so this is not a finding.
- Out of my lane: whether the Canon capabilities C-001 to C-011 exist as the bodies claim, and whether the split is right (scope and design critics).

```findings
[]
```
