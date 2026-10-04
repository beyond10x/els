---
format: aep.planning-md/3
id: review-result:els-first-domain-design-r2
kind: review-result
status: active
title: els-first-domain decomposition — design critic, round 2
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
needs-revision

story:security-independence-rules — it holds two things, a security-review rule in `software.change/1` and a rollback-verification rule in `incident.response/1`. Its `depends_on` edges on `story:ess-conformance-evidence` and `story:incident-response-protocol` join the two chains, so the incident rule can't start until the four-story software chain has finished. Either split it, with the incident half depending on `story:incident-response-protocol` and `story:software-change-negative-outcomes` (which adds `rollback_result`) and one `depends_on` edge between the halves because both edit `crates/els/src/vocabulary.rs`, or keep it as one story and state why the two rules must land together — `.engineering/planning/story/security-independence-rules.md:38,41`; `aep plan artifact graph` (edges `security-independence-rules → incident-response-protocol` and `→ ess-conformance-evidence`)

story:els-vocabulary — it delivers two things, the term vocabulary and the compile-and-evaluate fixture harness. The harness needs Canon compile and evaluation capability, but the Canon-capability section lists only C-001 identifier forms. No acceptance step runs the harness, so the head of the graph depends on an unexercised harness and every later story finds its defects. Either move the harness into its own story that both protocol stories `depends_on`, or add an acceptance that compiles and evaluates a smoke fixture through it — `.engineering/planning/story/els-vocabulary.md:79,87`

**What I read:** 12 artifacts via `aep plan artifact list`, `show` (epic, 9 stories, design-r1), `relations`, `graph` and `validate`. Validate reported the set valid. The shell was fish, so one chained `show` loop errored mid-way and I re-ran the remaining stories separately.

**Graph:** I walked the 10 `depends_on` edges and the decomposes, serves and reviews edges, including those to the visions. There is no cycle. The six-link chain on `software_change.rs` has its shared-file reason written in each body, so it is not a finding by itself. Only `story:incident-response-protocol` and `story:ml-protocol-shape-decision` sit outside it, so the finding on `story:security-independence-rules` is the trade-off between splitting and keeping the join.

**Round-1 findings:** all five landed.
- The two `implementation.verified` conjuncts are now stated to extend the definition in `story:software-change-protocol`.
- `story:software-change-negative-outcomes` now owns `release.rollback` and `rollback_result`.
- `story:els-vocabulary` now has an ownership table of terms.
- `story:stale-evidence-fixtures` now declares each staleness rule together with its fixture.

**What I could not establish:**
- Whether Canon will deliver a compile-and-evaluate API. The Canon stories are all `proposed` and `crates/els` has no `tests/` directory. That dependency is outside the set and I did not count it as a finding.
- Whether `story:software-change-protocol` should say what establishes `implementation.reviewed` and `deployment.healthy`. `story:stale-evidence-fixtures` declares the staleness rule while the protocol story only names the claims (`.engineering/planning/story/software-change-protocol.md:41`). This is an unease, not a nameable fix.
- Out of my lane, for the acceptance and scope critics: `different_agent_run` is added to the vocabulary and resolved in acceptance step 8, but no rule or fixture uses it (`.engineering/planning/story/security-independence-rules.md:45,70`).
- Out of my lane: `story:software-change-protocol` declares `release.proven`, `objective.realized` and `implementation.reviewed`, and its acceptance exercises none of them.

```findings
- file: .engineering/planning/story/security-independence-rules.md
  line: 38
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: it holds two rules, security review in software.change/1 and rollback verification in incident.response/1, and its depends_on edges on story:ess-conformance-evidence and story:incident-response-protocol join the two chains so the incident rule waits on the whole software chain; split it with the incident half depending on story:incident-response-protocol and story:software-change-negative-outcomes and one depends_on edge between the halves for crates/els/src/vocabulary.rs, or state why the two rules must land together
- file: .engineering/planning/story/els-vocabulary.md
  line: 79
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: it delivers the vocabulary and the compile-and-evaluate fixture harness as one item, the harness needs Canon compile and evaluation capability that its Canon-capability section (C-001 only) does not list, and no acceptance step runs the harness, so the head of the graph carries an unexercised half; move the harness to its own story that both protocol stories depends_on, or add an acceptance that compiles and evaluates a smoke fixture through it
```
