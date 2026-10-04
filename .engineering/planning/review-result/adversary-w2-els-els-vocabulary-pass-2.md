---
format: aep.planning-md/3
id: review-result:adversary-w2-els-els-vocabulary-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w2 adversary, els story:els-vocabulary, pass 2
relations:
- reviews: story:els-vocabulary
revision: 1
---
unit: els/els-vocabulary, uncommitted working tree on impl/els-vocabulary (base 943992c)
verdict: NEEDS-CHANGE
cases: executed 6→6, red 0
origin: introduced 1 / pre-existing 2 / undecided 0
wrote-outside-worktree: 22 paths, all under the assigned scratch dir (listed in part 6)
needs-coordinator: whether to rewrite pass-1's exact-table test (finding 1). It sits in an adversary file, so no unit in this wave can change it.

**Summary:** I wrote no failing test, so nothing is red in the tree. The vocabulary code holds up. The problem is in the pass-1 test file: one of its tests will break the moment any later story adds a term, which five planned stories do.

## 1. `git --no-pager diff --stat`
```
 crates/els/src/lib.rs              |  2 ++
 docs/examples/incident-response.md | 12 ++++++------
```
Untracked: `crates/els/src/vocabulary.rs` and `crates/els/tests/` (the implementation and pass 1). I changed no file in the worktree.

## 2. Cases added
None. Everything I found is either a planned future state or a wording question. Writing a test for those would just pin my own opinion as a constant.

## 3. Suite run
I ran `cargo test --workspace --locked` with the brief's `CARGO_TARGET_DIR`. It exited 0: lib 1 passed, adversary_vocabulary 3 passed, vocabulary 2 passed, doctests 0.

## 4. Findings

| # | file:line | finding | verdict / origin | evidence | what reaches it |
|---|---|---|---|---|---|
| 1 | crates/els/tests/adversary_vocabulary.rs:107, :110-113 | Pass 1 requires the vocabulary to be exactly the 35 story terms. The story says five later stories each add terms to `vocabulary.rs` (software-change-profiles, software-change-negative-outcomes, ess-conformance-evidence, security-independence-rules, rollback-verification-rule). None of them has this test file in scope, and an existing case may not be weakened. | NEEDS-CHANGE / introduced | Mutant `later_story_risk` (adds `risk`, the profiles story's first term): unit suite green, pass 1 red, `left: 36 right: 35`. Mutant `later_story_rollback_verified` gives the same result. Pass 1's own `extra_term` mutant used `rollback.verified`, which is a planned term. | The next unit on the chain. Fix: keep the per-term category and marking check, and drop the `len()==35` and "no extra" parts. |
| 2 | crates/els/src/vocabulary.rs:90 | `lookup` takes only an id, so one spelling can't belong to two categories. Design § 10 (lines 900-930) names obligations after evidence kinds (`test_result`, `security_review`, `operational_observation`). The profiles story needs obligation ids and lists none. | CONFIRMED / introduced | Mutant `obligation_named_like_evidence` (adds obligation `test_result`): unit suite red at tests/vocabulary.rs:63 and :126 ("declared twice"). | Only if a later story follows the § 10 names. No story declares that yet, so this doesn't hold the unit. |
| 3 | crates/els/src/vocabulary.rs:209 | The meaning of `release.proven` ("shown to do what was intended") is the meaning of `objective.realized`. Design § 9 defines it as `implementation.verified` plus current `build_provenance`. | CONFIRMED / introduced | Mutant `release_proven_meaning`: both suites stay green, because nothing checks meanings beyond being non-empty. | The protocol stories name terms from this module, and this meaning is the only definition ELS writes down. |
| 4 | .engineering/planning/story/els-vocabulary.md:32 | The story says "authority requirements are keyed by action id", but Canon's C-006 decides authority per capability, which has its own name (`publish_finding` requires `finding.publish`). Software-change-protocol acceptance 3-4 will need a capability string. Using the action id as that string still works. | CONFIRMED / pre-existing | Canon `.engineering/planning/story/action-admissibility.md`, Outcome and Domain relations | story:software-change-protocol and story:incident-response-protocol |
| 5 | canon docs/design/canon-protocol-calculus-design.md:716-739 | Canon § 14's INC-492 example puts the verb first in action ids (`inspect.metrics`, `rollback.recent_release`) and uses dotted obligation ids (`establish.service_health`). ELS puts the subject first and uses single-token obligations. | CONFIRMED / pre-existing | ELS examples used `metrics.inspect` at base | Only readers of the docs |

The respelled incident example matches Canon § 18 (`impact.bounded`, `service.healthy`, `cause.identified`). The ELS design doc has no incident text, so there is nothing there to contradict.

Two smaller points I left out of the findings:
- The `Marking::Core` comment at vocabulary.rs:28 says "meaningful for … alike". At least 6 core terms are incident-only, such as `emergency.leave` and `restore_service`. The story defines core as "not specific to Git, pull requests or code", which is a weaker property.
- Lookups only check claim and action ids, not other ids in the examples. Mutants `doc_obligation_typo` and `doc_artifact_typo` survive, but acceptance item 3 only asks about claim and action ids.

## 5. Attacked and could not break
- **Pass-1 table copied from the implementation?** No. I checked both copies term by term against the story; they match.
- **Pass-1 dotted-id test passing on nothing?** No. It requires a non-empty set per file, and the unit test requires at least one claim and one action per file.
- **`claim_id()` / `action_id()` on the wrong category:** they return `None` and don't panic. A panicking mutant is caught at tests/vocabulary.rs:143.
- **Canon semantics leaking through the API:** none. There is no truth, evidence or obligation logic, only conversion to Canon's id types, and no Canon compile or evaluate is used.
- **Conflicts with story:fixture-harness:** none. Its smoke protocol uses only core terms (`service.healthy`, `operational_observation`), and the `lib.rs` module line can be added to.
- **Surviving mutant:** `marking_display_debug`. `Display` for `Marking` is never checked directly, and pass 1 calls `to_string` on both sides of its comparison. I'm noting it only.

## 6. Paths written outside the worktree
All under `~/.cache/ga-wave-2026-10-04-w2/els-els-vocabulary/scratch/`:
- `adv2-mutate.sh`
- `adv2-pristine/` and `adv2-mut/` (copies of the worktree without `.git`/`.engineering`, 192K each)
- `adv2-target/` (15M, a separate build dir so the copies don't share the unit's build output)
- `adv2-mut-<name>-{unit,adv}.log` for 9 mutants, 18 files

Nothing else was written. The unit's build dir only got the test run in part 3.

## 7. Findings block
```findings
- file: crates/els/tests/adversary_vocabulary.rs
  line: 107
  category: contract-drift
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: pass-1 requires exactly 35 terms, so every later story that adds a vocabulary term, as the story plans, turns it red without having the file in scope
- file: crates/els/src/vocabulary.rs
  line: 90
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: lookup takes only an id, so the obligation names design § 10 uses (test_result, security_review) cannot be declared next to the evidence kinds of the same name
- file: crates/els/src/vocabulary.rs
  line: 209
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: the meaning of release.proven describes objective.realized, not the design § 9 definition (implementation.verified plus current build_provenance)
- file: .engineering/planning/story/els-vocabulary.md
  line: 32
  category: contract-drift
  severity: note
  verdict: CONFIRMED
  origin: pre-existing
  message: the story keys authority by action id, while Canon C-006 decides authority per separately named capability
- file: docs/design/canon-protocol-calculus-design.md
  line: 716
  category: contract-drift
  severity: note
  verdict: CONFIRMED
  origin: pre-existing
  message: Canon's INC-492 example puts the verb first in action ids and uses dotted obligation ids, unlike ELS's subject-first actions and single-token obligations
```
