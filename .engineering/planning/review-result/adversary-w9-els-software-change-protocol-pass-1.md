---
format: aep.planning-md/3
id: review-result:adversary-w9-els-software-change-protocol-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w9 adversary, els story:software-change-protocol, pass 1
relations:
- reviews: story:software-change-protocol
revision: 1
---
unit: els/software-change-protocol, commit debd808 plus my untracked test file in ~/.local/state/worktree/trees/b10x/els/els-w9-software-change-protocol
verdict: NEEDS-CHANGE (6 red cases: 2 are wrong behaviour in the shipped protocol, 4 are wrong protocols that the acceptance still passes)
cases: executed 76→84, red 6
origin: introduced 6 / pre-existing 0 / undecided 0
wrote-outside-worktree: 3 paths
needs-coordinator: yes. Finding 1 can't be fixed in this protocol and needs a Canon story. The coordinator's subject-binding attack is answered by finding 1.

**1. Diff**
```
?? crates/els/tests/adversary_software_change_protocol.rs
```
`git diff --stat` is empty and the only untracked file is a test file. Nothing under `protocols/`, `fixtures/` or the harness was touched, and the mutants are compiled from in-memory copies of the protocol text.

**2. Cases added** (all in `crates/els/tests/adversary_software_change_protocol.rs`)

| Test | Asserts | Now |
|---|---|---|
| `a_test_result_about_another_artifact_does_not_verify_the_implementation` :265 | a passing `test_result` about intent, system_specification, plan, release or deployment (each at its current revision), plus the merge grant, leaves `tests.pass` and `implementation.verified` UNKNOWN and merge blocked | red |
| `accepted_is_blocked_while_the_implementation_is_unverified` :306 | with only the R1 tests, a healthy d0 and a satisfied objective, `accepted` is blocked | red |
| `acceptance_rejects_accepted_on_any_instead_of_all` :346 | the replayed acceptance rejects `accepted` changed from `all` to `any` | red |
| `acceptance_rejects_tests_pass_on_a_failing_test_result` :397 | the replayed acceptance rejects `tests.pass` without `result: pass` | red |
| `acceptance_rejects_claims_that_ignore_their_evidence_result` :432 | the same for `implementation.reviewed`, `deployment.healthy` and `objective.realized` | red |
| `acceptance_rejects_merge_declared_as_a_read` :469 | the replayed acceptance rejects `repository.merge effect: read` | red |
| `the_replayed_acceptance_passes_the_shipped_protocol` | the replay accepts the shipped protocol, so the replay is faithful | green |
| `merge_needs_current_tests_and_its_own_grant` | probes the merge gate (part 5) | green |

Each mutant test first checks that the mutant really decides differently from the shipped protocol, then fails because the acceptance does not notice.

Red output, from the first run of this file alone (line numbers are from before rustfmt; the latest run fails at the same assertions, at :290, :328, :372, :423, :454 and :481):
```
---- a_test_result_about_another_artifact_does_not_verify_the_implementation stdout ----
assertion `left == right` failed: implementation R2 was never tested; a passing test_result about each of these artifacts made tests.pass and implementation.verified TRUE and repository.merge admissible
  left: ["intent@i1: (True, True, \"admissible\")", "system_specification@s1: (True, True, \"admissible\")", "plan@p1: (True, True, \"admissible\")", "release@v0: (True, True, \"admissible\")", "deployment@d0: (True, True, \"admissible\")"]
---- accepted_is_blocked_while_the_implementation_is_unverified stdout ----
assertion `left == right` failed: the change is accepted while implementation R2 is unverified and unmerged
  left: "legitimate"
 right: "blocked"
---- acceptance_rejects_accepted_on_any_instead_of_all stdout ----
a protocol whose `accepted` needs only one of objective.realized and deployment.healthy passes the acceptance
---- acceptance_rejects_tests_pass_on_a_failing_test_result stdout ----
a protocol whose tests.pass is TRUE on a failing test result passes the acceptance
---- acceptance_rejects_claims_that_ignore_their_evidence_result stdout ----
  left: ["code_review", "operational_observation", "objective_observation"]
---- acceptance_rejects_merge_declared_as_a_read stdout ----
a protocol that declares repository.merge a read passes the acceptance
test result: FAILED. 2 passed; 6 failed
```

**3. Suite** (run after the cases existed; CARGO_TARGET_DIR=~/.cache/b10x-target/els-w9-software-change-protocol)

| Command | Exit |
|---|---|
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |
| `cargo test --workspace --locked --no-fail-fast` | 101: 78 passed, 6 failed (exactly the 6 above) |

`-- --list` shows all 8 adversary tests and `chg_1842_merge_waits_for_current_revision_tests_and_authority` in this tree. The failure lines point into this file at this tree's line numbers.

**4. Findings** (all cover debd808)

| # | file:line | Finding | Verdict / origin | What reaches it |
|---|---|---|---|---|
| 1 | protocols/software-change/1.yaml:26 | A `test_result` about any artifact at its current revision makes `tests.pass` and `implementation.verified` TRUE, and with the grant, merge becomes admissible while R2 was never tested. This contradicts the story's "bound to the implementation revision". Canon 8fc260a can't express it: an evidence kind has only `description` and `max_age` (canon `model/mod.rs:81-88`), and a match has only `kind` and `result` (`predicate.rs:70-74`). This is the same gap the coordinator saw in incident-response. | INFEASIBLE / introduced | Whatever produces the record sets its subject. Nothing in ELS or Canon refuses a different subject. I built the case myself and found no workflow that produces one. |
| 2 | protocols/software-change/1.yaml:104 | `accepted` requires only `objective.realized` and `deployment.healthy`. Design § 9 reaches `accepted` only after `candidate` (implementation verified) and `released` (release proven), and Canon has no states to keep that path. So a change that was never verified or merged is accepted. Fix: add `- claim: release.proven` to `accepted.requires`. | INFEASIBLE / introduced | Constructed: ordinary evidence (healthy d0, objective satisfied) with only stale tests. No fixture or workflow builds this today. |
| 3 | crates/els/tests/software_change_protocol.rs:252 | `accepted` is only ever asserted `blocked`, and both of its claims are UNKNOWN in every state. So a protocol where either claim alone is enough passes. | CONFIRMED / introduced | Mutant, killed by a state with only `deployment.healthy` TRUE |
| 4 | crates/els/tests/software_change_protocol.rs:193 | Only `implementation.verified`'s `result: pass` is pinned. `tests.pass` without a result is TRUE on a failing R2 test, and the acceptance passes. | CONFIRMED / introduced | Mutant, killed by a failing R2 `test_result` |
| 5 | fixtures/software-change/chg-1842.fixture.yaml:18 | No state gives `code_review`, `operational_observation` or `objective_observation` evidence, so dropping `result:` from any of those three claims passes. | CONFIRMED / introduced | Mutant |
| 6 | crates/els/tests/software_change_protocol.rs:203 | No action's `effect` is asserted (incident-response does this at `incident_response_protocol.rs:159-171`). `repository.merge effect: read` passes. | CONFIRMED / introduced | Mutant. The effect class is shown on the website page. |

**5. Attacked and not broken**
- With the grant and only R1 tests, merge is blocked. With R2 tests and a denial, merge is blocked. A grant for another capability leaves it approval-required. Passing and failing R2 results together make verified UNKNOWN, so merge is blocked.
- `set_revisions` moving implementation R2→R3 after the grant excludes the R2 result and blocks merge again.
- Each of these protocol mutants fails the fixture: merge without its precondition, merge without its capability or with the wrong one, `any` in place of `all` in `implementation.verified`, `release.proven` without either conjunct.
- Every name is in the vocabulary in its category, and the test checks that.
- The generated `1.mdx`, the graph JSON and the `index.md` row match the YAML.
- Not tested: an R2 grant still admits merging R3 once R3's tests pass. Canon authority is per capability, not per revision, so I didn't raise it.

**6. Paths written outside the worktree**
- ~/.cache/ga-wave-2026-10-04-w9/els-software-change-protocol/scratch/adversary1-cases.log
- ~/.cache/ga-wave-2026-10-04-w9/els-software-change-protocol/scratch/adversary1-suite.log
- ~/.cache/b10x-target/els-w9-software-change-protocol (the assigned build dir; I built into it)

My session lease `adversary-w9-scp-p1` has been released.

**7. Findings**
```findings
- file: protocols/software-change/1.yaml
  line: 26
  category: acceptance
  severity: warning
  verdict: INFEASIBLE
  origin: introduced
  message: "A passing test_result about any artifact at its current revision (intent, specification, plan, release, deployment) makes tests.pass and implementation.verified TRUE and merge admissible with the grant, because Canon 8fc260a cannot bind an evidence kind or match to a subject."
- file: protocols/software-change/1.yaml
  line: 104
  category: acceptance
  severity: warning
  verdict: INFEASIBLE
  origin: introduced
  message: "The accepted outcome requires only objective.realized and deployment.healthy, so a change never verified, merged or released is accepted, although design section 9 reaches accepted only through implementation.verified and release.proven."
- file: crates/els/tests/software_change_protocol.rs
  line: 252
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "accepted is asserted only as blocked with both of its claims UNKNOWN, so a protocol whose accepted requires any instead of all passes the acceptance."
- file: crates/els/tests/software_change_protocol.rs
  line: 193
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "tests.pass without result: pass is TRUE on a failing R2 test result and still passes the acceptance, which pins the result only on implementation.verified."
- file: fixtures/software-change/chg-1842.fixture.yaml
  line: 18
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "No state supplies code_review, operational_observation or objective_observation evidence, so dropping result: from implementation.reviewed, deployment.healthy or objective.realized passes the acceptance."
- file: crates/els/tests/software_change_protocol.rs
  line: 203
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "No action effect class is asserted, so repository.merge declared effect: read passes the acceptance."
```
