---
format: aep.planning-md/3
id: review-result:els-first-domain-acceptance-r1
kind: review-result
status: active
title: els-first-domain decomposition — acceptance critic, round 1
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

els-vocabulary — the acceptance requires every term in `docs/examples/` to resolve, but the outcome settles `service.healthy` over the examples' `service_healthy`, so the test either refuses an example term or never checks the settled spelling, and "engineering term" is not delimited — .engineering/planning/story/els-vocabulary.md:37
incident-response-protocol — the acceptance names only end states (`service_healthy = TRUE`, `restore_service` satisfied, "leave-emergency step admissible") and no start state for them, so a protocol whose leave-emergency step is always admissible passes — .engineering/planning/story/incident-response-protocol.md:50
security-independence-rules — the acceptance joins a `software.change/1` outcome and an `incident.response/1` outcome with "and", so one protocol can pass while the other fails — .engineering/planning/story/security-independence-rules.md:39
security-independence-rules — the acceptance names no claim id ("the security-review claim", "rollback verification") and gives the incident half no `TRUE` case, so an incident protocol that never verifies rollback passes — .engineering/planning/story/security-independence-rules.md:39
software-change-negative-outcomes — the acceptance joins three independent outcomes (`declined`, `superseded`, `rolled_back` blocked) in one sentence, so any one can pass while the others fail — .engineering/planning/story/software-change-negative-outcomes.md:42
software-change-negative-outcomes — the `rolled_back` clause names only "blocked … until rollback-result evidence is supplied" and not the outcome being reached once it is, so a protocol that can never conclude `rolled_back` passes — .engineering/planning/story/software-change-negative-outcomes.md:42
stale-evidence-fixtures — each stale case names only the `UNKNOWN` end state and no fresh or matching-revision counterpart, so a review claim or `deployment.healthy` that is always `UNKNOWN` passes — .engineering/planning/story/stale-evidence-fixtures.md:39
ml-protocol-shape-decision — the acceptance asks only that an `architecture-decision-record` be "held", and an ADR starts at `proposed`, so an undecided record meets it; it names no `accepted` status — .engineering/planning/story/ml-protocol-shape-decision.md:33

Read: 9 stories (`els-vocabulary`, `ess-conformance-evidence`, `incident-response-protocol`, `ml-protocol-shape-decision`, `security-independence-rules`, `software-change-negative-outcomes`, `software-change-profiles`, `software-change-protocol`, `stale-evidence-fixtures`) plus the parent `epic:els-first-domain`. I ran `aep plan artifact list/show/lifecycle/relations/validate`, and read `docs/examples/*.md`, `AGENTS.md` and `crates/els/src/lib.rs`. The store validates clean.

Approved on acceptance: `software-change-protocol` (CHG-1842 start, change and end states, matches the example), `software-change-profiles` (strictly nested obligation sets) and `ess-conformance-evidence` (a complete truth table with a `TRUE` case).

Could not establish:
- Whether Canon C-001…C-011 can express any of these fixtures, which is out of my lane.
- The incident leave-emergency step and the security-review and rollback-verification claim ids appear in no source I read.

Out of my lane, not in the verdict:
- The `implementation.conforms` claim is not in `software-change-protocol`'s claim list and `ess-conformance-evidence` does not say it declares it (scope or design).
- The epic's own acceptance joins three statements (`.engineering/planning/epic/els-first-domain.md`, Acceptance).

```findings
- file: .engineering/planning/story/els-vocabulary.md
  line: 37
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: the acceptance requires every term in docs/examples to resolve, but the outcome settles service.healthy over the examples' service_healthy, so the test either refuses an example term or never checks the settled spelling, and "engineering term" is not delimited
- file: .engineering/planning/story/incident-response-protocol.md
  line: 50
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: the acceptance names only end states (service_healthy = TRUE, restore_service satisfied, "leave-emergency step admissible") and no start state for them, so a protocol whose leave-emergency step is always admissible passes
- file: .engineering/planning/story/security-independence-rules.md
  line: 39
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: the acceptance joins a software.change/1 outcome and an incident.response/1 outcome with "and", so one protocol can pass while the other fails
- file: .engineering/planning/story/security-independence-rules.md
  line: 39
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: the acceptance names no claim id ("the security-review claim", "rollback verification") and gives the incident half no TRUE case, so an incident protocol that never verifies rollback passes
- file: .engineering/planning/story/software-change-negative-outcomes.md
  line: 42
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: the acceptance joins three independent outcomes (declined, superseded, rolled_back blocked) in one sentence, so any one can pass while the others fail
- file: .engineering/planning/story/software-change-negative-outcomes.md
  line: 42
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: the rolled_back clause names only "blocked ... until rollback-result evidence is supplied" and not the outcome being reached once it is, so a protocol that can never conclude rolled_back passes
- file: .engineering/planning/story/stale-evidence-fixtures.md
  line: 39
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: each stale case names only the UNKNOWN end state and no fresh or matching-revision counterpart, so a review claim or deployment.healthy that is always UNKNOWN passes
- file: .engineering/planning/story/ml-protocol-shape-decision.md
  line: 33
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: the acceptance asks only that an architecture-decision-record be "held", and an ADR starts at proposed, so an undecided record meets it; it names no accepted status
```
