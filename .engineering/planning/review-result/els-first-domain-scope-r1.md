---
format: aep.planning-md/3
id: review-result:els-first-domain-scope-r1
kind: review-result
status: active
title: els-first-domain decomposition — scope critic, round 1
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

I extracted 11 promises from `epic:els-first-domain` and the Atlas `epic:ga-els-first-domain` it refines, and traced all 11 to exactly one story. I found no gap, no reach beyond the parent, no double claim and no silent narrowing.

**What I read**
- 12 artifacts: the parent, the 9 stories and the Atlas parent epic, plus the TASKBOARD lines for E-001 to E-009.
- Commands: `aep plan artifact show` on `epic:els-first-domain` and each of the 9 stories, `aep plan artifact show epic:ga-els-first-domain` (via the Atlas planning file), and `aep plan artifact graph` in both stores. `aep plan artifact kinds` and `aep plan artifact relations` were not run this pass.

**Promises and the story that claims each**
1. Engineering vocabulary (`.engineering/planning/epic/els-first-domain.md:7`, `:20`): `story:els-vocabulary`.
2. `software.change/1` with revision-bound implementation evidence: `story:software-change-protocol`.
3. Profiles: `story:software-change-profiles`.
4. Outcomes beyond `accepted`: `story:software-change-negative-outcomes`.
5. `incident.response/1`: `story:incident-response-protocol`.
6. Stale-evidence fixtures (E-005, in the Atlas outcome and in the local "Covers E-001 … E-009" at `.engineering/planning/epic/els-first-domain.md:20`): `story:stale-evidence-fixtures`.
7. ESS conformance evidence binding (E-007): `story:ess-conformance-evidence`.
8. Authority and independence rules (E-008): `story:security-independence-rules`. Authority requirements themselves sit in `story:software-change-protocol`, and independence is claimed only by this story.
9. ML protocol shape decided and recorded, not built (E-009): `story:ml-protocol-shape-decision`.
10. Acceptance, `.engineering/planning/epic/els-first-domain.md:24-26`, both protocols compile and evaluate on the same kernel: `story:software-change-protocol` and `story:incident-response-protocol`.
11. Acceptance, the R1-test-evidence, R2-implementation case reports `tests.pass = UNKNOWN`, and the incident fixture leaves emergency mode with `cause_identified = UNKNOWN`: `story:software-change-protocol` and `story:incident-response-protocol`.

Neighbouring stories are fenced against each other, so no two claim the same outcome. `story:software-change-protocol` and `story:stale-evidence-fixtures` split R1/R2 test staleness from review, observation and health staleness. `story:stale-evidence-fixtures` and `story:ess-conformance-evidence` split spec-digest staleness. `story:software-change-protocol`, `story:software-change-profiles`, `story:software-change-negative-outcomes` and `story:security-independence-rules` each state their out-of-story items. `story:ml-protocol-shape-decision` stays a decision with no build, matching the Atlas epic's "decided and recorded (E-009), not built". Nothing else in either store claims part of the parent: the Atlas graph shows only the Atlas epic and its reviews.

**What I could not establish**
- I did not read the build pack `ROADMAP.md` Phase 2, which the Atlas epic cites as its source. I judged the set against the two epic bodies and the TASKBOARD.
- Out of my lane, and not part of my verdict: whether each acceptance is checkable (`plan-critic-acceptance`). Also whether the cross-story dependencies are sound, for example `story:security-independence-rules` adding incident rollback verification that `story:incident-response-protocol` does not declare (`plan-critic-design`).

```findings
[]
```
