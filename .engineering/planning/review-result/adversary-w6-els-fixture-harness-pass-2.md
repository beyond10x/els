---
format: aep.planning-md/3
id: review-result:adversary-w6-els-fixture-harness-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w6 adversary, els story:fixture-harness, pass 2
relations:
- reviews: story:fixture-harness
revision: 1
---
unit: els/fixture-harness, the working tree of els-w6-fixture-harness (d4c8970 plus the uncommitted phase 2 and pass-1 fixes)
verdict: CONFIRMED — 3 red cases: one contradicts the module doc, two are notes
cases: executed 35→44, red 3
origin: introduced 6 / pre-existing 0 / undecided 0
wrote-outside-worktree: 3 logs, plus build output in the brief's build dir
needs-coordinator: whether a fixture's `id` must match its file name, and whether `add_evidence:` with no value must be refused (both are about how far the harness enforces its own format)

**1. Diff**

`git --no-pager diff --stat` shows only `crates/els/tests/fixture_harness.rs | 8 ++++++--`. That is the implementor's earlier fmt reflow, not my change. My two files are untracked: `crates/els/tests/adversary2_harness_inputs.rs` and `crates/els/tests/adversary2_harness_mutants.rs`. I touched no implementation or fixture path. The tests left an empty, gitignored `target/` in the worktree.

**2. Cases added** (each file was run alone before the suite)

| Test | Asserts | Now |
|---|---|---|
| inputs: `adversary2_harness_refuses_a_claim_expected_twice` | a fixture that expects `service.healthy` twice is refused | red |
| inputs: `adversary2_harness_refuses_add_evidence_written_without_a_value` | `add_evidence:` with no value is refused | red |
| inputs: `adversary2_harness_refuses_a_fixture_whose_id_is_not_its_file_name` | `other.fixture.yaml` declaring `id: smoke` is refused | red |
| inputs: `adversary2_harness_aliases_and_merge_keys_reach_canon_as_written` | an aliased record is expanded and refused by Canon as `duplicate-identifier`; a `<<` merge key is refused as `malformed-input`, unknown field `<<` | green |
| mutants: 5 tests | leap years (1900, 2000, 2100, 2400); `check` reports a claim found TRUE where UNKNOWN or FALSE is expected; case and evidence refusals carry Canon's code and message unchanged; `CompileError::Invalid` passes Canon's problems through | green |

Red output, verbatim:
```
`add_evidence:` with no value loads as 0 records for state `initial`
`target/adversary2-harness-ids-2490467/other.fixture.yaml` loads as fixture `smoke`, the id `fixtures/smoke/smoke.fixture.yaml` also declares (`smoke`)
a fixture expecting `service.healthy` twice (false, then unknown) loads, and check gives Ok(()): the `false` expectation was dropped
test result: FAILED. 1 passed; 3 failed; ...
```

**3. Suite run**

`cargo test --workspace --locked --offline --no-fail-fast`, exit 101. Only `--test adversary2_harness_inputs` failed (1 passed, 3 failed); the other 12 result lines are ok. The total is 44 cases. 35 is the same run without my two binaries (9 cases). `-- --list` shows all 9 adversary2 tests exist in this tree. Clippy `-D warnings` exits 0 and `cargo fmt --all --check` exits 0, after I ran rustfmt on my two files only.

**4. Findings**

All cover the working tree above.

| file:line | Verdict | What was measured / what reaches it |
|---|---|---|
| `support/mod.rs:199` | CONFIRMED, introduced, warning | `claims: BTreeMap<String, Truth>` keeps the last of two equal keys without a word. Lines 51-53 of the module doc promise a fixture "cannot carry an input the harness would drop". All other repeated keys are refused (struct fields by serde, the case and records by `Mapping`). No fixture writes a claim twice today; every later protocol fixture's claim list could. Suggested fix: read `claims` through `serde_yaml_ng::Mapping`, or refuse duplicates while deserializing. |
| `support/mod.rs:219` | INFEASIBLE, introduced, note | `add_evidence:` with no value reads as `[]`. Canon refuses a key written with no value (`model/present.rs`). No fixture does this today. |
| `support/mod.rs:232` | INFEASIBLE, introduced, note | `Fixture::load` does not check the declared `id` against the `<fixture-id>.fixture.yaml` convention in the module doc, so two files can share an id. Only one fixture exists today. |
| `support/mod.rs:392` | CONFIRMED, introduced, warning | Reasoned, not built: changing `found == expected` to `found <= expected` passes all earlier tests. Canon orders True < False < Unknown, so TRUE found against UNKNOWN expected would pass silently. Killed by my `check_reports_true_where...` test. |
| `support/mod.rs:448` | CONFIRMED, introduced, note | Reasoned: `leap = year % 4 == 0`, dropping `year % 400`, and `2 if leap => 28` all pass pass 1, which only tried 2026. Killed by my leap-year test. |
| `support/mod.rs:261` | CONFIRMED, introduced, note | Reasoned: no earlier test checks the wording of a case refusal (:261), an evidence refusal (:274-281) or a `CompileError::Invalid` (:173). Dropping the code or message survives. Killed by my three refusal tests. |

**5. Attacked and could not break**
- **Aliases and anchors:** reach Canon expanded, the same way Canon's text reader sees them.
- **Merge keys:** not applied (`serde_yaml_ng` only does that through `apply_merge`), so they are refused, as Canon's reader refuses them.
- **Repeated struct fields, case keys and record keys:** all refused.
- **Unknown vs absent in an expectation:** both directions are reported.
- **Refusal pass-through:** the evaluate path already carries Canon's code (pass 1); the case, evidence and compile paths do too (my tests).
- **Canon pin:** `Cargo.lock` is at `33540b14b9ac…`, and `--locked --offline` builds.
- **Identifier adaptation:** `ProtocolId::new`, `as_str` and the `model::` paths in `crates/els/src` match the old tuple fields. Nothing was lost; the old types had no serde.
- **Module docs vs behaviour:** they match, apart from the first row in section 4.
- **Mutants:** none were built. Disk was at 7.0G free and the coordinator ordered no extra build dir. `els-w6-mutants` was never created. Every mutant verdict above comes from reading the code. I also did not take a worktree session lease.

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w6/els-fixture-harness/scratch/adv2/red-inputs.log`
- `~/.cache/ga-wave-2026-10-04-w6/els-fixture-harness/scratch/adv2/mutants.log`
- `~/.cache/ga-wave-2026-10-04-w6/els-fixture-harness/scratch/adv2/suite.log`
- New test binaries in `~/.cache/b10x-target/els-w6-fixture-harness`

**7. Findings block**

```findings
- file: crates/els/tests/support/mod.rs
  line: 199
  category: contract-drift
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "expect.claims is a BTreeMap, so a claim written twice keeps only its last value and check passes on a dropped expectation, contrary to the module doc's promise that no input is dropped"
- file: crates/els/tests/support/mod.rs
  line: 219
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "add_evidence written with no value loads as an empty list, where Canon refuses a key written without a value; no fixture does this today"
- file: crates/els/tests/support/mod.rs
  line: 232
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "Fixture::load does not check the declared id against the <fixture-id>.fixture.yaml file name, so two fixture files can share an id; only one fixture exists today"
- file: crates/els/tests/support/mod.rs
  line: 392
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "found == expected mutated to found <= expected survives the earlier suite and would accept TRUE where UNKNOWN or FALSE is expected (reasoned, not built)"
- file: crates/els/tests/support/mod.rs
  line: 448
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "leap-year mutants (no century rule, no 400 rule, February 28 in leap years) survive the earlier suite, which only tries 2026 (reasoned, not built)"
- file: crates/els/tests/support/mod.rs
  line: 261
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "no earlier test checks the wording of case, evidence or CompileError::Invalid refusals, so dropping Canon's code or message survives (reasoned, not built)"
```
