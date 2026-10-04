---
format: aep.planning-md/3
id: review-result:els-first-domain-acceptance-r2
kind: review-result
status: active
title: els-first-domain decomposition — acceptance critic, round 2
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

epic:els-first-domain — the acceptance names `cause_identified = UNKNOWN`, a spelling story:els-vocabulary now settles as `cause.identified` and refuses (its acceptance item 4 refuses the old underscore form), so the incident clause cannot be checked as written, and it also joins three independent outcomes with semicolons — .engineering/planning/epic/els-first-domain.md:26

Round-1 findings, one line each:
- Fixed: `story:els-vocabulary` (35 terms, 24 core and 11 marked, which match the table's counts), `story:incident-response-protocol` (items 2 to 4 give start states and a blocked/admissible pair), `story:ml-protocol-shape-decision` (item 2 requires `accepted`; the lifecycle runs `proposed -> accepted, rejected`), `story:stale-evidence-fixtures` (each stale case has a fresh counterpart).
- Fixed: `story:security-independence-rules` has start states and `TRUE` cases for both halves in numbered items. The two protocols share one test, as the already-approved multi-item stories do.
- Fixed: `story:software-change-negative-outcomes` has a separate item per outcome, and item 5 reaches `rolled_back`.
- Still approved: `story:software-change-protocol`, `story:software-change-profiles`, `story:ess-conformance-evidence`.

Read: 10 artifacts (the epic and 9 stories). I ran `aep plan artifact list`, `show` on the round-1 acceptance result, the epic and each story, and `lifecycle architecture-decision-record`. I also read `docs/examples/*.md`, and `git diff --stat` showed that only the 9 stories changed since round 1.

Could not establish:
- Whether the `story:els-vocabulary` item 3 test can extract claim and action ids from the example `text` blocks unambiguously. Lines such as `implementation = R2` and `traffic.shift [approval]` need a rule that the story does not state. This is not a finding, because the rule belongs to the test's author.
- Whether the 30-minute horizon, `different_principal` and `ess-conformance-report/2` match the design document. That is outside my lane.

```findings
- file: .engineering/planning/epic/els-first-domain.md
  line: 26
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: the acceptance names `cause_identified = UNKNOWN`, a spelling story:els-vocabulary now settles as `cause.identified` and refuses (its acceptance item 4 refuses the old underscore form), so the incident clause cannot be checked as written, and it also joins three independent outcomes with semicolons
```
