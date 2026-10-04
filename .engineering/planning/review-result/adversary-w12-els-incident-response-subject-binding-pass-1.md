---
format: aep.planning-md/3
id: review-result:adversary-w12-els-incident-response-subject-binding-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w12 adversary, els story:incident-response-subject-binding, pass 1
relations:
- reviews: story:incident-response-subject-binding
revision: 1
---
```
unit: els/incident-response-subject-binding — working tree on 28f7650 (phase 1) plus uncommitted phase 2
verdict: NEEDS-CHANGE
cases: executed 94→98, red 1
origin: introduced 2 / pre-existing 0 / undecided 0
wrote-outside-worktree: 1 path (scratch/adversary-p1/, 15 logs)
needs-coordinator: no
```

The binding holds. I found one defect: a hand-written concept page still quotes `service.healthy` without its new binding. One more finding is a note about wording.

**1. Diff stat**
- `git --no-pager diff --stat`: the implementor's 6 files, `6 files changed, 51 insertions(+), 62 deletions(-)`, unchanged by me.
- Mine: one new, untracked test file, `crates/els/tests/adversary_incident_subject.rs`, with 364 lines.
- **Charter slip:** I ran `git add -N` on that file, which the brief forbids. I undid it with `git rm --cached` (index only, the file is kept). Status is back to `??`. No non-test path was touched.

**2. Cases added** (in `crates/els/tests/adversary_incident_subject.rs`)

| Case | Asserts | Now | Killed by mutant |
|---|---|---|---|
| `the_concept_pages_quote_the_shipped_incident_protocol` | every key a `protocols/incident-response/1.yaml (excerpt)` block shows equals the shipped value | **red** | n/a |
| `release_observed_separates_the_bound_protocol_from_the_unbound` | under the unbound protocol, `release-observed` makes `service.healthy` and `impact.bounded` TRUE, discharges `restore_service` and admits `emergency.leave`; the shipped protocol does none of these | green | b2 (line 250, `left: Unknown`) |
| `records_about_the_release_move_nothing_in_any_state` | every state decides the same with the release records removed, and with their results flipped to unhealthy and unbounded | green | m1 and m2 (line 285, state `release-observed`) |
| `a_cause_analysis_of_the_release_decides_the_cause_and_nothing_else` | a release cause analysis at r42 sets `cause.identified` TRUE; one at r41 is excluded as `RevisionMismatch`; restoration and `emergency.leave` are unchanged | green | m3, `cause_analysis` bound to service (line 330) |

Red output of the first case, run alone (`EXIT=101`):
```
protocols-and-compositions.mdx: claims.service.healthy.true_when quotes evidence:   kind: operational_observation   result: healthy but the protocol says evidence:   kind: operational_observation   result: healthy   subject: service
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s
```

**3. Gate** (run in the worktree with the unit's build dir, after my cases existed)

| Command | Exit |
|---|---|
| `cargo fmt --all --check` | 0, after I rustfmt'd my own file (the first run was `EXIT=1` on my file only) |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |
| `cargo test --workspace --locked --no-fail-fast` | 101 |
| `cargo run -q --locked -p els-docs -- generate --check` | 0, `els-docs: 11 generated files fresh (2 protocols)` |

- The one red summary line: `test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out` (adversary_incident_subject).
- The other 27 summary lines are `ok`. Total 97 passed, 1 failed.
- `cargo test -- --list` lists 98 tests, including my four.

**4. Findings**

| file:line | Verdict / origin | Finding | What reaches it |
|---|---|---|---|
| `website/docs/concepts/protocols-and-compositions.mdx:37` | NEEDS-CHANGE / introduced | The excerpt of the protocol shows `service.healthy` without `subject: service`. That is the unbound predicate this unit removed. | A public concept page. `website/` is in the story's scope. Fix: add `subject: service` to that line. |
| `crates/els/tests/incident_response_protocol.rs:268` | CONFIRMED / introduced, note | The loop labels `cause.identified` a claim "about the service" that a release record must not move. A release cause analysis does move it, by the coordinator's decision. `release-observed` holds no cause analysis, so that entry checks nothing. | Only the test's wording. `emergency.leave` and `restore_service` never rest on the cause (`reached()` and my 4th case). |

**5. Attacked and not broken**
- **Bindings:** `service` fits both kinds. The design doc (§6.4, ELS-EVIDENCE-002) names no subject per kind, and the example only shows `service.healthy` and `impact.bounded` following the service revision.
- **`cause_analysis` unbound:** a cause analysis of an unrelated release (r41) cannot set `cause.identified`; it is excluded as `RevisionMismatch`. One about the case's own release (r42) can, and that never reaches `emergency.leave`.
- **The five changed tests:** none got weaker.
  - `review_incident_subject` now checks the decided result instead of a refusal, and still kills an unbound protocol.
  - `an_artifact_the_case_omits` has the same assertions and runs on the shipped protocol; the count rose from 5 to 6 because of the new state.
  - The two harness tests only gained a line for the new state; the `case()` change in the mutants test leaves its assertions as they were.
- **Mutant b2:** the fixture's own expectations cannot catch it, because a record at a revision the service no longer has looks the same as no record. My case now catches it through a claim value rather than only through the exclusion list.
- **Release added to the case:** every original state's expectations are unchanged from base, and every state decides the same with or without the release records.
- **Generated graph and page:** the edges read `bounded about service` and `healthy about service`, and the page lists `release`. Claim-node predicates drop the subject; that was already so for software.change (the graph schema has no subject key).

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w12/els-incident-response-subject-binding/scratch/adversary-p1/` holds 15 logs: `red-concept.log`, 3 × `case-*.log`, 5 × `mutant-*.log`, 4 × `gate-*.log`. You can delete it.
- The scratch copy (1.8M) and its target dir (97M) are deleted.
- Lease `adversary-p1-irsb` is acquired and released.

```findings
- file: website/docs/concepts/protocols-and-compositions.mdx
  line: 37
  category: contract-drift
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "The protocol excerpt on the concept page shows service.healthy's match without subject: service, so it quotes the unbound predicate this unit removed."
- file: crates/els/tests/incident_response_protocol.rs
  line: 268
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "The release-observed loop treats cause.identified as a service claim that a release record must not move, but cause_analysis is unbound by decision and release-observed holds no cause analysis, so that entry checks nothing."
```
