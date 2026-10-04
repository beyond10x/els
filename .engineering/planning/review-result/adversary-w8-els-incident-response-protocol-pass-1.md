---
format: aep.planning-md/3
id: review-result:adversary-w8-els-incident-response-protocol-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w8 adversary, els story:incident-response-protocol, pass 1
relations:
- reviews: story:incident-response-protocol
revision: 1
---
```
unit: els/incident-response-protocol, working tree at 3bc4e68 plus uncommitted phase 2 (support/mod.rs, incident_response_protocol.rs) plus my 2 untracked test files
verdict: NEEDS-CHANGE
cases: executed 59→63, red 3
origin: introduced 3 / pre-existing 0 / undecided 0
wrote-outside-worktree: 2 paths (part 6)
needs-coordinator: yes. Finding 3's fix is in crates/els-docs, which is outside the story's scope
```

The protocol YAML itself is correct. The acceptance test and fixture are weaker than they claim: two wrong versions of the protocol pass every check. The generated website page also leaves out what discharges `restore_service`.

**1. Diff stat**

`git --no-pager diff --stat` (tracked files, all from phase 2, none mine):
```
 crates/els/tests/incident_response_protocol.rs | 107 ++++++++
 crates/els/tests/support/mod.rs                | 349 +++++++++++++++++++++----
```
My files are untracked, so the diff stat does not show them. `git status --short` lists them as `?? crates/els/tests/adversary_incident_mutants.rs` and `?? crates/els/tests/adversary_incident_website.rs`. I changed no other path. `rustfmt` ran on my two files only; `support/mod.rs` still has its 08:25 modification time.

**2. Cases added**

| file | test | asserts | now |
|---|---|---|---|
| `adversary_incident_mutants.rs` | `acceptance_rejects_a_leave_that_rests_on_cause_analysis` | the test's independence check or inc-492 rejects a protocol whose `emergency.leave` is `any[all[healthy, bounded], evidence cause_analysis=identified]` | red |
| same | `acceptance_rejects_restore_service_discharged_on_unknown_health` | the acceptance rejects a `discharged_when` of `any[healthy is true, healthy is unknown]` | red |
| same | `discriminating_states_separate_the_shipped_protocol_from_both_mutants` | two states that tell the real protocol apart from both mutants; this is the shape of the fix | green |
| `adversary_incident_website.rs` | `protocol_page_and_graph_say_what_discharges_restore_service` | the page's Obligations section names `service.healthy`, and the graph has an edge at `obligation:restore_service` | red |

Each red test first checks that the shipped protocol passes the replayed checks, then shows the mutant really is wrong, then fails. Red output from running my cases alone, before the suite (saved to `scratch/adversary1-cases-red.log`):
```
panicked at crates/els/tests/adversary_incident_mutants.rs:259:5:
a protocol whose emergency.leave is admissible on cause analysis alone passes the acceptance: independence check passes = true, inc-492 differences = []
panicked at crates/els/tests/adversary_incident_mutants.rs:290:5:
a protocol that discharges restore_service while the service's health is unknown passes the acceptance: independence check passes = true, inc-492 differences = []
panicked at crates/els/tests/adversary_incident_website.rs:49:5:
the page's Obligations section does not name `service.healthy`: ... | `restore_service` | urgent; bring the affected service back to health |
the graph has no edge at `obligation:restore_service`
```
After `rustfmt` the line numbers are 261, 292 and 50.

**3. Suite**

`cargo test --workspace --locked --no-fail-fast`: EXIT=101. Every binary is `ok` except `adversary_incident_mutants` (1 passed, 2 failed) and `adversary_incident_website` (0 passed, 1 failed). That is 63 run, 3 red. `--list` on `incident_response_protocol` shows its 3 tests exist in this tree. `cargo clippy --workspace --all-targets --locked -- -D warnings` and `cargo fmt --all --check` both exit 0.

**4. Findings** (they cover the working tree described in the header)

| # | file:line | verdict | origin | what was measured / what reaches it |
|---|---|---|---|---|
| 1 | `crates/els/tests/incident_response_protocol.rs:133` | NEEDS-CHANGE | introduced | The independence check uses `claim_references()`, which cannot see an evidence match in a precondition. inc-492 never adds a `cause_analysis`, so `emergency.leave` admissible on an identified cause while the service is unhealthy passes. What reaches it: any later edit of the protocol (rollback-verification-rule and stale-evidence-fixtures edit this file next). Fix: add a fixture state with an identified cause while the service is unhealthy, expecting `emergency.leave` blocked. |
| 2 | `fixtures/incident-response/inc-492.fixture.yaml:49` | NEEDS-CHANGE | introduced | Option C makes a state reachable between the rollback and the first observation of s2: the old observations are excluded and `service.healthy` is UNKNOWN. inc-492 skips that state, so discharging `restore_service` on unknown health passes. Nothing checks `discharged_when` structurally. Fix: add a state after the s2 revision with no s2 evidence, expecting `restore_service` open and `emergency.leave` blocked. |
| 3 | `website/docs/protocols/incident-response/1.mdx` | NEEDS-CHANGE | introduced | The canon bump added `discharged_when` to the compiled obligation (at 33540b1 the obligation IR has only a description). The generator still renders a description only (`crates/els-docs/src/protocol.rs:244`, `graph.rs:128`), so the page never says what discharges `restore_service`, and its graph node has no edges. The fix is in els-docs, outside this story's scope. |

**5. Attacked and could not break**
- Vocabulary: every name the protocol declares is in `vocabulary.yaml` under its category. Capabilities and results are not a vocabulary category.
- The YAML against the story's Outcome and `docs/examples/incident-response.md`: claims, evidence kinds, the obligation, the 3 read actions, the 2 gated actions with their own capabilities, and `emergency.leave` without `cause.identified` all match.
- Harness: it passes Canon's decision through unchanged. Authority accumulates inside one `from_yaml` call, so nothing leaks between fixtures. Each state keeps its own copy of the case, so `set_revisions` cannot change an earlier state's result.
- Missing `expect.obligations` or `expect.actions`: the smoke protocol declares neither, so nothing is hidden there. Every inc-492 state lists both.
- Canon pin bump: no other ELS test reads obligations, actions or outcomes. The refusal of a top-level `authority:` key in `fixture_harness` still holds, and the suite is green apart from my cases.
- Swapped capabilities, `any` in place of `all` (without an evidence match), and "not false" forms are all caught. The three-valued logic keeps UNKNOWN blocked. I reasoned these through; I ran only the two mutants above.

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w8/els-incident-response-protocol/scratch/adversary1-cases-red.log`
- `~/.cache/ga-wave-2026-10-04-w8/els-incident-response-protocol/scratch/adversary1-suite.log`
- I also wrote build output into the assigned `~/.cache/b10x-target/els-w8-incident-response-protocol`.

**7. Findings block**
```findings
- file: crates/els/tests/incident_response_protocol.rs
  line: 133
  category: mutant
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "The independence check misses evidence matches and inc-492 never adds a cause_analysis, so an emergency.leave admissible on an identified cause while the service is unhealthy passes the acceptance."
- file: fixtures/incident-response/inc-492.fixture.yaml
  line: 49
  category: mutant
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "inc-492 has no state between the s2 revision and its first observation, where service.healthy is UNKNOWN, so a restore_service discharged on unknown health passes the acceptance."
- file: website/docs/protocols/incident-response/1.mdx
  category: contract-drift
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "After the canon bump the compiled obligation carries discharged_when, but the generated page and graph render restore_service as its description only, so they never say that service.healthy discharges it."
```
