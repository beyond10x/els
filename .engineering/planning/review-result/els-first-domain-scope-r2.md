---
format: aep.planning-md/3
id: review-result:els-first-domain-scope-r2
kind: review-result
status: active
title: els-first-domain decomposition — scope critic, round 2
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

I traced 11 promises from `epic:els-first-domain` and the Atlas epic it refines, and found a story for all 11. I found no gap, no reach beyond the parent, no double claim and no silent narrowing. None of the revisions since round 1 changed that.

**Promises traced.** The parent's own promises are the Outcome "Covers TASKBOARD E-001 … E-009" and the three-clause Acceptance. The Atlas epic adds a longer outcome list (`.engineering/planning/epic/ga-els-first-domain.md`, Outcome). Each of the 11 is claimed by exactly one story, except the shared acceptance clauses, which are split by protocol:

| # | Promise | Claimed by |
|---|---|---|
| 1 | engineering vocabulary (E-001) | `story:els-vocabulary` |
| 2 | `software.change/1` with revision-bound implementation evidence (E-002) | `story:software-change-protocol` |
| 3 | trivial/standard/elevated/critical profiles (E-003) | `story:software-change-profiles` |
| 4 | "several terminal outcomes" (E-006) | `story:software-change-negative-outcomes` |
| 5 | `incident.response/1` (E-004) | `story:incident-response-protocol` |
| 6 | stale-evidence fixtures (E-005) | `story:stale-evidence-fixtures` |
| 7 | ESS conformance evidence binding (E-007) | `story:ess-conformance-evidence` |
| 8 | "authority and independence rules" (E-008) | authority on `repository.merge` in `story:software-change-protocol`; independence in `story:security-independence-rules` |
| 9 | ML protocol shape "decided and recorded, not built" (E-009) | `story:ml-protocol-shape-decision` |
| 10 | Acceptance: both protocols compile and evaluate on the same Canon kernel | `story:software-change-protocol`, `story:incident-response-protocol` |
| 11 | Acceptance: `tests.pass = UNKNOWN` for R1 evidence against an R2 implementation, and the incident fixture leaves emergency mode with the cause `UNKNOWN` | `story:software-change-protocol` (acceptance 2), `story:incident-response-protocol` (acceptance 4) |

**Claims that touch.** None overlap:
- The overlapping claims are fenced in the story bodies. `implementation.verified` is defined in `story:software-change-protocol` and extended by `story:ess-conformance-evidence` and `story:security-independence-rules`, each adding one named conjunct.
- `story:software-change-protocol` keeps R1/R2 test staleness. `story:stale-evidence-fixtures` takes review, observation and health staleness. Spec-digest staleness stays in `story:ess-conformance-evidence`.
- `release.rollback` and `rollback_result` appear in the two protocols but are declared once per protocol, in different stories.

**Reach.** Every story traces to a parent sentence. The harness and layout that `story:els-vocabulary` creates serve the parent's "fixture reports" acceptance. The 30-minute incident horizon in `story:stale-evidence-fixtures` is stated there as taken from design § 11.3.

**What I read**
- 13 artifacts: the parent, the 9 stories, `review-result:els-first-domain-scope-r1`, the Atlas epic file `.engineering/planning/epic/ga-els-first-domain.md` and the TASKBOARD (E-001 to E-009).
- Commands, all run in the `els-plan-w2` worktree: `aep plan artifact show` on the parent and all 9 stories (whole body), `aep plan artifact graph`, and `aep plan artifact show review-result:els-first-domain-scope-r1`.

**What I could not establish**
- I did not run `aep plan artifact kinds` or `relations`. The edges used are `decomposes` for "drafted from" and `depends_on`.
- Build pack `ROADMAP.md` Phase 2, which the Atlas epic cites, was not read. I judged against the two epic bodies and the TASKBOARD.
- Out of my lane, and not part of my verdict:
  - `story:software-change-profiles` says profiles change "authority requirements", but its acceptance tests only the open-obligation sets (`plan-critic-acceptance`).
  - The parent's acceptance writes `cause_identified`, while the stories spell it `cause.identified` under the rename `story:els-vocabulary` documents. It is the same claim, so it is not a scope defect.
  - `story:els-vocabulary` creates the fixture harness `crates/els/tests/support/mod.rs`, and its acceptance does not exercise it (`plan-critic-acceptance`).

```findings
[]
```
