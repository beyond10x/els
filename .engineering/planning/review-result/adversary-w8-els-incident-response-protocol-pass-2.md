---
format: aep.planning-md/3
id: review-result:adversary-w8-els-incident-response-protocol-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w8 adversary, els story:incident-response-protocol, pass 2
relations:
- reviews: story:incident-response-protocol
revision: 1
---
unit: els/incident-response-protocol, working tree at 3bc4e68 plus the uncommitted phase 2 and pass-1 fixes
verdict: CONFIRMED (no blocker; 2 warnings are INFEASIBLE, 5 notes)
cases: executed 65→75, red 5
origin: introduced 7 / pre-existing 0 / undecided 0
wrote-outside-worktree: 6 (scratch logs); 4 tmp dirs under the build dir, removed
needs-coordinator: none

**1. `git --no-pager diff --stat`**

The tracked stat lists `graph.rs`, `protocol.rs`, `incident_response_protocol.rs`, `support/mod.rs`, `inc-492.fixture.yaml`, `incident-response-1.json` and `1.mdx`. All of these are the implementor's uncommitted phase 2. I did not touch any of them.

My changes are three new untracked test files, and nothing else:
- `~/.local/state/worktree/trees/b10x/els/els-w8-incident-response-protocol/crates/els/tests/adversary2_incident_harness.rs`
- `~/.local/state/worktree/trees/b10x/els/els-w8-incident-response-protocol/crates/els/tests/adversary2_incident_effects.rs`
- `~/.local/state/worktree/trees/b10x/els/els-w8-incident-response-protocol/crates/els-docs/tests/adversary2_obligation_render.rs`

**2. Cases added.** `--list` confirmed all of them in this tree. Each red case was run alone before the suite.

| Case | Asserts | Now |
|---|---|---|
| `a_record_observed_after_the_instant_is_refused` | a record whose own `observed_at` (2026-10-05) is after `at` is refused when loading | red |
| `the_harness_passes_no_observation_time_to_canon` | no evidence reaches Canon carrying `observed_at` | red |
| `the_acceptance_tells_a_write_metrics_inspect_from_the_read_one` | the acceptance can tell `metrics.inspect` with `effect: write` from the shipped protocol | red |
| `a_negated_discharge_is_not_drawn_as_its_opposite` | `not {claim: a}` and `not {claim: c, is: unknown}` are not drawn as `requires a` and `requires c=unknown` | red |
| `the_index_counts_the_obligations_a_protocol_declares` | the index row for incident.response mentions its obligation | red |
| 5 probes (4 harness, 1 render) | grant then denial, entry order, an artifact missing from the case, inert timestamps, nested obligations rendered faithfully and deterministically | green |

Red output, verbatim:
```
a fixture whose record health-2 is observed at 2026-10-05T00:00:00Z, after its instant 2026-10-04T12:00:00Z, loads; the records carry ["health-2 observed_at 2026-10-05T00:00:00Z"] to Canon
the evidence the harness passes to Canon carries observation times: ["health-2 observed_at 2026-10-04T11:40:00Z"]
with `metrics.inspect` declared `effect: write`, every name, the precondition of `emergency.leave` and Canon's decision for every inc-492 state are unchanged: ...
`refute` (discharged when not `a`) is drawn as requiring `a`: ["{\"from\":\"claim:a\",\"kind\":\"requires\",\"to\":\"obligation:refute\"}"]
`decide` (discharged when `c` is not UNKNOWN) is drawn as requiring `c` UNKNOWN: [...,"qualifier":"unknown",...]
| [`incident.response/1`](./incident-response/1.mdx) | 1 | ... | 3 claims, 6 actions, 0 outcomes |
```

**3. Suite run.** Command: `cargo test --workspace --locked --no-fail-fast`, with the brief's environment. Exit 101.
- 75 tests ran: 70 passed, 5 failed, which are exactly the five red cases above.
- Every test file that existed before passed.
- The full log is in `adv2-suite.log` in the scratch directory.

**4. Findings**

| # | file:line | What was measured | What reaches it | Verdict / origin |
|---|---|---|---|---|
| F1 | `crates/els/tests/support/mod.rs:87`, `:528` | Since the Canon bump to 8fc260a, a record may carry its own `observed_at`. At 33540b1 such a record was refused, because the record type rejects unknown keys. The harness passes that time to Canon, and it compares only the outer `observed_at` with `at`. So two of its doc promises are false: it does pass observation times, and it does load evidence observed after the instant. | Nothing in the tree. `story:stale-evidence-fixtures` is the first story that passes `at`. | INFEASIBLE / introduced (warning) |
| F2 | `crates/els-docs/src/graph.rs:183-194` | A claim test under `not` loses its polarity on the graph edge. `not {claim: c, is: unknown}` is drawn with qualifier `unknown`, which is the one value that keeps the obligation open. The outcome and gate edges have the same defect, and that part is pre-existing. | No ELS protocol uses `not` today. Canon's own obligations test uses this exact shape (`a.decided`). | INFEASIBLE / introduced (warning) |
| F3 | `protocols/incident-response/1.yaml:54` | Changing `effect: read` to `write` leaves every name, `emergency.leave` and every inc-492 decision unchanged. Canon never reads `effect`, and the acceptance test never asserts it, even though the story's Outcome calls these three the read actions. | The acceptance test itself | CONFIRMED / introduced (note) |
| F4 | `crates/els-docs/src/generate.rs:217` | The index's "Declares" column counts claims, actions and outcomes, but not obligations: "0 outcomes", with no mention of `restore_service`. The code is unchanged since the base; this unit is the first to show a row. | The shipped site index | CONFIRMED / introduced (note) |
| F5 | `crates/els/tests/support/mod.rs:502-509` | A grant followed by a denial of the same capability in a later state is not a revocation. Canon refuses the accumulated list as `duplicate-identifier` (`` `--authority` decides capability `release.rollback` more than once ``), for that state and every later one. This shows up only from `check`; the fixture still loads. The module docs do not say so (green probe). | A fixture that revokes a grant; none today | CONFIRMED / introduced (note) |
| F6 | `crates/els-docs/src/protocol.rs:156` | The "Dependency graph" intro says only "which outcomes rest on those claims". This protocol has 0 outcomes, and the graph now carries obligation edges. | The shipped page | CONFIRMED / introduced (note) |
| F7 | `crates/els-docs/src/protocol.rs:247` | "UNKNOWN does not discharge it" reads wrongly for a `{claim: c, is: unknown}` discharge, which Canon makes TRUE when c is UNKNOWN. | No ELS protocol uses it | INFEASIBLE / introduced (note) |

Fixes I would suggest (not applied):
- **F1:** refuse an `observed_at` inside the record, or require it to equal the outer one and check it against `at`; then correct the doc lines 86-88.
- **F2:** carry polarity through `not` (for example qualifier `false`), or emit no edge under `not`.
- **F3:** assert each action's `effect` from the IR.
- **F4:** add the obligation count.
- **F5:** document it, or let a later decision replace an earlier one per capability.
- **F6:** name obligations in the graph intro.
- **F7:** say "its predicate being UNKNOWN".

**5. Attacked and not broken**
- Nested all/any/not discharges: the page text matches Canon's compiled order.
- Edges for several obligations: no duplicates, grouped by obligation id, and two runs give identical bytes.
- The order of `add_authority` entries does not change any decision.
- An artifact the protocol declares but the case omits: Canon refuses it as `missing-artifact` at `check`, and `set_revisions` on it is refused at load with an accurate message.
- inc-492's timestamps 11:00 to 11:45 are inert. Canon gets no `--at` and the protocol declares no `max_age`. Moving health-2 before health-1, or every observation to `at`, still passes.
- Protocol YAML against the vocabulary: every name is a term in its category. The ELS design has no incident-response section; § 27 only lists `incident.remediation/1` as a possible future lifecycle.
- The index link and both category files are consistent with the autogenerated sidebar.
- The other doc claims in `support/mod.rs` match the behaviour: states are cumulative, `None` and `[]` authority behave the same, and the refusals it lists happen.

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w8/els-incident-response-protocol/scratch/adv2-a_record_observed_after_the_instant_is_refused.log`
- `~/.cache/ga-wave-2026-10-04-w8/els-incident-response-protocol/scratch/adv2-the_harness_passes_no_observation_time_to_canon.log`
- `~/.cache/ga-wave-2026-10-04-w8/els-incident-response-protocol/scratch/adv2-effects.log`
- `~/.cache/ga-wave-2026-10-04-w8/els-incident-response-protocol/scratch/adv2-a_negated_discharge_is_not_drawn_as_its_opposite.log`
- `~/.cache/ga-wave-2026-10-04-w8/els-incident-response-protocol/scratch/adv2-the_index_counts_the_obligations_a_protocol_declares.log`
- `~/.cache/ga-wave-2026-10-04-w8/els-incident-response-protocol/scratch/adv2-suite.log`
- The tests wrote `~/.cache/b10x-target/els-w8-incident-response-protocol/tmp/adversary2-obligation-render-*` (4 dirs). I removed them; a test run recreates them.
- No session lease was taken: `worktree` has no lease verb, and no host hook ran for one.

**7. Findings block**

```findings
- file: crates/els/tests/support/mod.rs
  line: 528
  category: contract-drift
  severity: warning
  verdict: INFEASIBLE
  origin: introduced
  message: "Since the Canon bump to 8fc260a a record's own observed_at reaches Canon unchecked, so the module docs' promises that the harness passes no observation time and refuses evidence observed after the instant are false for such a record."
- file: crates/els-docs/src/graph.rs
  line: 190
  category: contract-drift
  severity: warning
  verdict: INFEASIBLE
  origin: introduced
  message: "A claim test under not loses its polarity on the claim-to-obligation requires edge, so not {claim: c, is: unknown} is drawn as requiring c UNKNOWN, the one value that keeps the obligation open."
- file: protocols/incident-response/1.yaml
  line: 54
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "Declaring metrics.inspect effect: write changes nothing the acceptance reads, so the story's read actions are not pinned as read."
- file: crates/els-docs/src/generate.rs
  line: 217
  category: contract-drift
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "The protocols index Declares column counts claims, actions and outcomes but not obligations, so incident.response/1 shows 0 outcomes and never mentions restore_service."
- file: crates/els/tests/support/mod.rs
  line: 502
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "A later denial of a capability granted in an earlier state is refused by Canon as duplicate-identifier at check time only, so a fixture cannot model revoking a grant and the module docs do not say so."
- file: crates/els-docs/src/protocol.rs
  line: 156
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "The Dependency graph intro names only outcomes, while incident.response/1 has no outcome and its graph now carries obligation edges."
- file: crates/els-docs/src/protocol.rs
  line: 247
  category: contract-drift
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "The Obligations intro says UNKNOWN does not discharge an obligation, which is false for a discharge written {claim: c, is: unknown}, which Canon makes TRUE when c is UNKNOWN."
```
