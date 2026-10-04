---
format: aep.planning-md/3
id: review-result:els-first-domain-design-r1
kind: review-result
status: active
title: els-first-domain decomposition — design critic, round 1
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

- `story:ess-conformance-evidence` — its `implementation.conforms` claim is, in design §9, a conjunct of `implementation.verified` (the executing to candidate gate) that `story:software-change-protocol` defines, and no body says whether E-007 extends that claim or adds a free-standing one — `docs/design/engineering-lifecycle-specification-design.md:716-735`, `.engineering/planning/story/software-change-protocol.md:30`, `.engineering/planning/story/ess-conformance-evidence.md:22`
- `story:security-independence-rules` — the security-review claim is also a conjunct of `implementation.verified` (design §9) and the body does not say whether it joins that definition or stands apart, so the protocol story and this one each hold half of one claim — `docs/design/engineering-lifecycle-specification-design.md:716-735`, `.engineering/planning/story/security-independence-rules.md:25`
- `story:software-change-negative-outcomes` — its acceptance needs a rollback action and `rollback_result` evidence on `software.change/1` (design §9, §15), but the body does not say it declares them and the protocol story's action list names only `repository.merge`, so neither story owns them — `.engineering/planning/story/software-change-negative-outcomes.md:25,42`, `.engineering/planning/story/software-change-protocol.md:30`
- `story:els-vocabulary` — its acceptance resolves only terms in the two example files, while the protocol stories name `code_review`, `operational_observation`, `release.proven` and `objective.realized` "from the E-001 vocabulary". `system_conformance`, `security_review` and `rollback_result` (E-006, E-007, E-008) are not assigned to the vocabulary or to those stories. The body should state who owns each term — `.engineering/planning/story/els-vocabulary.md:21,37`, `.engineering/planning/story/software-change-protocol.md:30`
- `story:stale-evidence-fixtures` — it is a test-layer slice: the freshness horizons and revision binding are declared in `story:software-change-protocol` and `story:incident-response-protocol`, and their fixtures land only here, against `AGENTS.md`'s rule that every protocol rule lands with its fixture. E-002 already keeps its own R1/R2 stale case. The fix is to fold each stale case into the story that declares the rule, or to say why the rule lands without a fixture — `AGENTS.md:26`, `.engineering/planning/story/stale-evidence-fixtures.md:25`, `.engineering/planning/story/incident-response-protocol.md:34`

**What I read:** 12 artifacts — the epic, 9 stories and 2 visions — through `aep plan artifact list`, `show`, `relations`, `graph` and `validate` (valid). I walked all 33 declared edges and went outside the set to the visions. Canon's store (`aep plan artifact list` and `graph` in the canon repo) was read for its edges and not for cycles; ELS's links to Canon capabilities are `refs`, not edges, and are out of lane. I also read `AGENTS.md` and `docs/examples/`, and design §9-§12 and §15.

**Structure:** there is no cycle. The chain does not serialise the set. `story:els-vocabulary` leads to the two protocol stories, five stories hang off them, and `story:ml-protocol-shape-decision` is free of the others. Every `depends_on` points at a story whose outcome the dependent genuinely needs.

**What I could not establish:**
- Whether C-004 binds two subject revisions at once. `story:ess-conformance-evidence` already flags this as a wait on Canon, so it is outside the set.
- Whether E-003, E-007 and E-008 edit one `software.change/1` source and its case inputs (`risk`, `affects_security_boundary`, `affects_runtime_behavior`). That is the parallel-safety critic's lane, and it does not set my verdict.
- Whether E-005's `deployment.healthy` and incident-health acceptance can be checked is the acceptance critic's lane.

```findings
- file: .engineering/planning/story/ess-conformance-evidence.md
  line: 22
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: its implementation.conforms claim is a conjunct of implementation.verified in design §9 which story:software-change-protocol defines, and no body says whether E-007 extends that claim or adds a free-standing one, so the two stories each hold half of one claim
- file: .engineering/planning/story/security-independence-rules.md
  line: 25
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: the security-review claim is also a conjunct of implementation.verified in design §9 and the body does not say whether it joins that definition or stands apart, so the protocol story and this one each hold half of one claim
- file: .engineering/planning/story/software-change-negative-outcomes.md
  line: 25
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: its acceptance needs a rollback action and rollback_result evidence on software.change/1 but the body does not say it declares them and story:software-change-protocol names only repository.merge among its actions, so neither story owns them
- file: .engineering/planning/story/els-vocabulary.md
  line: 37
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: the acceptance resolves only terms in the two example files while story:software-change-protocol names code_review, operational_observation, release.proven and objective.realized from this vocabulary, and terms E-006, E-007 and E-008 need (system_conformance, security_review, rollback_result) are assigned neither here nor to those stories; the body should state who owns each term
- file: .engineering/planning/story/stale-evidence-fixtures.md
  line: 25
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: it is a test-layer slice whose fixtures exercise freshness and revision-binding rules declared in story:software-change-protocol and story:incident-response-protocol, so those rules land without fixtures against AGENTS.md:26; fold each stale case into the story that declares the rule or state why the rule lands without one
```
