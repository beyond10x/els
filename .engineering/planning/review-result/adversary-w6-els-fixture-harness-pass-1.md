---
format: aep.planning-md/3
id: review-result:adversary-w6-els-fixture-harness-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w6 adversary, els story:fixture-harness, pass 1
relations:
- reviews: story:fixture-harness
revision: 1
---
unit: els/fixture-harness, commit d4c8970 plus the uncommitted phase-2 working tree (`crates/els/tests/support/mod.rs`)
verdict: NEEDS-CHANGE
cases: executed 26→35, red 2
origin: introduced 8 / pre-existing 0 / undecided 0
wrote-outside-worktree: 5 paths (2 since deleted)
needs-coordinator: the two next protocol stories cannot be written without editing `support/mod.rs`, so their scope needs it (finding 8)

**1. What I touched**

`git --no-pager diff --stat` shows only ` crates/els/tests/fixture_harness.rs | 8 ++++++--`. That is the implementer's uncommitted fmt change, not mine. My two files are untracked (`??`):
- `crates/els/tests/adversary_harness_inputs.rs`
- `crates/els/tests/adversary_harness_mutants.rs`

Both are test files. I changed no non-test path.

**2. The cases I added** (9 in total)

Two are red now, and each was run alone first:

| Case | Asserts | Now |
|---|---|---|
| `adversary_harness_inputs.rs:26` `..._refuses_a_case_or_record_canon_refuses_to_read` | Canon's own readers refuse `revision: 1`, `subject_revision: 1` and `result: true`, so the harness must refuse them too | red |
| `adversary_harness_inputs.rs:62` `..._refuses_an_instant_that_is_not_a_calendar_date` | `2026-02-29`, `02-30` and `04-31` are refused | red |

Red output from the first run, verbatim (line 52 became 53 after rustfmt):
```
panicked at crates/els/tests/adversary_harness_inputs.rs:52:5:
the harness reads what Canon refuses: ["`service: {revision: 1}` loads as Case { … revision: Revision(\"1\") … }", "`subject_revision: 1` loads as …", "`result: true` loads as …"]
panicked at crates/els/tests/adversary_harness_inputs.rs:77:5:
instants that name no day load: ["2026-02-29T00:00:00Z", "2026-02-30T00:00:00Z", "2026-04-31T00:00:00Z"]
```
In the first case the asserts checking that Canon refuses each input passed, so it failed only on the harness side.

The other seven are green now. Each kills a mutant the unit's own suite misses. I applied each mutant to a copy in scratch, never to the worktree; the log is `scratch/adversary-1/mutants.log`. In every row, `fixture_harness` still reported `5 passed` under the mutant.

| Mutant on `support/mod.rs` | Killed by |
|---|---|
| K1 `:315` `states[..=end]` → `states[end..=end]` | `later_states_carry_earlier_evidence` (got `[]`, expected `["observation-1"]`) |
| K2 `:335` refusal not pushed | `check_reports_canon_refusals_verbatim` (got `Ok(())`) |
| K3 `:382` symlink check dropped | `refuses_paths_that_leave_through_a_symlink` |
| K4 `:259` `>` → `>=` | `instant_edges_that_load` |
| K5 `:90` path format changed | `protocol_path_follows_the_convention` |
| K6, K9 `:409`, `:411` hour 24 / second 60 | `instant_boundaries_are_refused` |
| K7 `:407` `(1..12)` | `instant_edges_that_load` |
| K8 `:163` parse error rewritten | `compile_passes_canon_parse_errors_through` |
| K10 `:164` `validate` dropped | survives both suites; it is equivalent, because Canon's `ir::compile` validates first (`ir/mod.rs:98`) |

**3. The suite, run after the cases existed**

Command: `cargo test --workspace --locked --no-fail-fast`. Exit 101, and red is the outcome I was after:
- `adversary_harness_inputs`: `test result: FAILED. 2 passed; 2 failed`
- `adversary_harness_mutants`: `ok. 5 passed`
- `fixture_harness`: `ok. 5 passed`
- Every other test file is ok. Total executed 35; the implementer's `phase2-gate.log` counted 26.
- `cargo fmt --all --check` exit 0. `cargo clippy --workspace --all-targets --locked -- -D warnings` exit 0.
- `-- --list` shows all 9 adversary tests in this tree.

**4. Findings** (all against commit d4c8970 plus the working tree; all `introduced`)

| # | file:line | Verdict | What was measured | What reaches it |
|---|---|---|---|---|
| 1 | `support/mod.rs:221` | NEEDS-CHANGE | The harness reads the case and evidence straight from the fixture text with `serde_yaml_ng::from_str`. That accepts a number or boolean where Canon expects text, so `result: true` silently becomes the string "true". Canon's own `read_case` / `read_evidence` refuse the same input as `malformed-input`. Fix: parse to a YAML value and call `b10x_canon::eval::case_from_value` / `evidence_from_value`. | A fixture author writing a bare revision such as `revision: 1`. The current examples use R1/R2, so nothing reaches it today. |
| 2 | `support/mod.rs:408` | CONFIRMED | `is_instant` accepts days that do not exist in their month, although its doc says "each field in range". | A typo in a fixture; nothing found that reaches it. |
| 3 | `support/mod.rs:315` | CONFIRMED | Cumulative states are untested by the unit's suite: the smoke fixture's first state has no evidence, so K1 survives. | Every multi-state fixture in the next stories |
| 4 | `support/mod.rs:335` | CONFIRMED | No unit test covers a state Canon refuses, so K2 survives. | Any fixture whose case does not fit its protocol |
| 5 | `support/mod.rs:382` | CONFIRMED | No unit test covers escaping through a symlink, so K3 survives. | Nothing found today |
| 6 | `support/mod.rs:259` | CONFIRMED | The instant boundaries, `protocol_path` and parse-error passthrough are untested (K4–K9). | `protocol_path` is the convention the next stories call |
| 7 | `support/mod.rs:65` | CONFIRMED | `#![allow(dead_code)]` hides that `protocol_path` (`:89`) and `id`, `case`, `states` (`:278`) are never used. Its comment says each test file uses a different part, but only one test file includes the module. | n/a |
| 8 | `support/mod.rs:197` | INFEASIBLE | Expectations hold only `claims`, and `authority:` is refused (`:61-63`). The fixture's instant applies to the whole fixture, and `observed_at` never reaches Canon. So software-change acceptance items 3–4 and incident-response items 2–4 (actions, obligations, authority decisions, the newer observation winning) cannot be written in `els-fixture/1`. Both stories say they will not edit `support/mod.rs`, and this story says later stories call it without editing it. It cannot be fixed here because Canon has not built C-005/C-006/C-008. | `story:software-change-protocol`, `story:incident-response-protocol` |

**5. Attacked and could not break**
- Expectation checking: a wrong value, a claim the expectation does not list, and an expected claim the protocol does not declare are each reported.
- `..`, absolute, backslash and drive-letter paths are refused.
- Non-UTC offsets, fractional seconds, lowercase `z` and a space instead of `T` are refused.
- Evidence observed exactly at the instant loads. That matches the doc ("not after"); the story text says "before T0", a small difference in wording.
- Canon's refusal codes and messages come through unchanged.
- Unknown keys are refused at every level, including Canon's case and evidence types.

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w6/els-fixture-harness/scratch/adversary-1/mutate.sh`, `mutants.log`, `mod.rs.orig` (kept)
- `~/.cache/ga-wave-2026-10-04-w6/els-fixture-harness/scratch/adversary-1/tree` (deleted)
- `~/.cache/b10x-target/els-w6-mutants` (140M, deleted)
- `~/.cache/b10x-target/els-w6-fixture-harness/tmp/adversary-harness-outside` (written by the symlink test)
- Builds went to the brief's `CARGO_TARGET_DIR`.
- At run time the symlink test also creates and removes `<worktree>/target/adversary-harness-<pid>/`, which git ignores; it is gone now.

Separately: the implementer's `scratch/mutate.sh` mutated `support/mod.rs` inside the worktree and then restored it.

```findings
- file: crates/els/tests/support/mod.rs
  line: 221
  category: contract-drift
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "The harness deserializes the case and evidence records directly from the fixture text, so it accepts numeric or boolean revisions and results that Canon's read_case/read_evidence refuse as malformed-input, and it reads `result: true` as the string \"true\"."
- file: crates/els/tests/support/mod.rs
  line: 408
  category: boundary
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "is_instant accepts 2026-02-29, 2026-02-30 and 2026-04-31 as evaluation or observation instants although its doc promises each field in range."
- file: crates/els/tests/support/mod.rs
  line: 315
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "The unit suite does not test cumulative states: evaluating a state over only its own add_evidence leaves fixture_harness green, because the smoke fixture's first state has no evidence."
- file: crates/els/tests/support/mod.rs
  line: 335
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "The unit suite never has check() meet a Canon refusal, so silently skipping a refused state leaves it green."
- file: crates/els/tests/support/mod.rs
  line: 382
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "The symlink confinement check is untested: removing resolved.starts_with(&root) leaves the unit suite green."
- file: crates/els/tests/support/mod.rs
  line: 259
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "Mutants of the instant comparison and field ranges, of protocol_path and of the parse-error passthrough all survive the unit suite."
- file: crates/els/tests/support/mod.rs
  line: 65
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "The module-wide allow(dead_code) hides that protocol_path, Fixture::id, Fixture::case and Fixture::states are unused and untested, and its justification claims several test files include the module when only one does."
- file: crates/els/tests/support/mod.rs
  line: 197
  category: acceptance
  severity: warning
  verdict: INFEASIBLE
  origin: introduced
  message: "els-fixture/1 can express only claim expectations, refuses authority decisions and never passes observed_at to Canon, so the software-change and incident-response acceptances cannot be written without editing support/mod.rs, which both stories exclude from their scope."
```
