---
format: aep.planning-md/3
id: review-result:adversary-w12-els-incident-response-subject-binding-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w12 adversary, els story:incident-response-subject-binding, pass 2
relations:
- reviews: story:incident-response-subject-binding
revision: 1
---
```
unit: els/incident-response-subject-binding — working tree on 28f7650 (phase 1) plus uncommitted phase 2, pass-1 fixes and pass-1 case file
verdict: NEEDS-CHANGE
cases: executed 98→102, red 1
origin: introduced 1 / pre-existing 0 / undecided 1
wrote-outside-worktree: 1 path (scratch/adversary-p2/, 16 logs)
needs-coordinator: yes — the story's typed scope does not list adversary_incident_subject.rs (pass 1) or adversary2_incident_subject.rs (mine)
```

The protocol binding holds. One defect is left: the `software.change/1` row in `website/product.json` is marked shipped but has no link to its page.

**1. Diff stat**
- `git --no-pager diff --stat`: `9 files changed, 54 insertions(+), 65 deletions(-)`. These are the implementor's and coordinator's files; I did not change any of them.
- My only addition is one new, untracked test file: `crates/els/tests/adversary2_incident_subject.rs`. No non-test path was touched, and I ran no `git add`.

**2. Cases added** (in `adversary2_incident_subject.rs`)

| Case | Asserts | Now | Mutant it kills |
|---|---|---|---|
| `the_incident_example_is_what_canon_decides_for_inc_492` | The example's block 1 equals Canon's decision of `initial`: claims, `restore_service` open, the listed actions, and every unlisted action blocked. Block 2 equals `service-restored`'s claims, with `emergency.leave` admissible and the obligation discharged. | green | e1 (example says impact UNKNOWN), e2 (rollback needs no capability) |
| `every_artifact_the_incident_protocol_and_inc_492_name_is_a_vocabulary_artifact_kind` | The declared artifacts, match subjects (exactly `["service"]`), case artifacts and record subjects are all `artifact_kind` terms. Each artifact's protocol description equals the term's vocabulary meaning. Every record's kind is declared. | green | v1 (protocol's `release` description drifts; **no other test catches it**), v2 (health bound to `release`) |
| `the_landing_page_status_rows_are_the_shipped_protocols` | Each protocol in `protocols/` has exactly one row: shipped, with `href` pointing at its generated page. No row names a protocol that does not exist. A row that says "every evidence match bound" holds for that protocol's IR. | **red** | p2 (`tests.pass` unbound while the row says bound) |
| `a_case_without_the_release_is_refused_for_the_release_at_every_state` | All 6 states are refused with the exact message for artifact `release` | green | r1 (a third artifact is declared; the existing `an_artifact_the_case_omits` stays green) |

Red output, case file run alone (`EXIT=101`), before the suite:
```
`software.change/1` row links Null, its generated page is /docs/protocols/software-change/1
test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
Mutant p1 adds the `href` to the scratch copy's `product.json`. With it, all 4 cases pass (`EXIT=0`), so that one line is the whole fix.

**3. Gate** (run in the worktree with the unit's build dir, after my cases existed)

| Command | Exit |
|---|---|
| `cargo fmt --all --check` | 1 on my file only. After `rustfmt` on that file it is 0. |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |
| `cargo test --workspace --locked` | 101. It stops at the first failing binary, so only 5 summary lines were printed. |
| `cargo test --workspace --locked --no-fail-fast` | 101: 29 summary lines, 101 passed and 1 failed |
| `cargo run -q --locked -p els-docs -- generate --check` | 0, `els-docs: 11 generated files fresh (2 protocols)` |

- The one red summary line: `test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`. The other 28 lines are `ok`.
- `cargo test -- --list` lists 102 tests, including my 4.

**4. Findings**

| file:line | Verdict / origin | Finding | What reaches it |
|---|---|---|---|
| `website/product.json:50` | NEEDS-CHANGE / introduced | The row was changed to `shipped` in this unit but has no `href`. The page `/docs/protocols/software-change/1` exists, and the `incident.response/1` row links its own page. | The public landing page. `website/` is in scope. Fix: add `"href": "/docs/protocols/software-change/1"`. |
| `crates/els/tests/adversary2_incident_harness.rs:155` | CONFIRMED / undecided, note | `an_artifact_the_case_omits` accepts any `missing-artifact` refusal. Under r1 it stays green even though every refusal names `deployment`, not `release`. | Nothing reaches it today. The acceptance test's artifact list also kills r1. My 4th case closes the gap. |

**5. Attacked and not broken**
- **Hand-written pages:** `index.md` ("fresh health evidence" → both TRUE, cause UNKNOWN), `composing-cases.mdx`, `step-order.mdx`, the composition sketch, `showcase.mdx` and `README.md` all agree with the shipped YAML and fixture. The excerpts in the concept pages are covered by pass 1's case.
- **product.json text:** the incident row, the "Delivery and incidents" feature, and the claim that every software.change/1 match is bound (all 6 claims name a subject) are accurate.
- **The example:** `docs/examples/incident-response.md` matches Canon's decisions for `initial` and `service-restored` exactly.
- **Vocabulary:** `release` resolves as a core `artifact_kind`, and the protocol's description matches its meaning word for word. The vocabulary tests (`vocabulary`, `vocabulary_yaml`, `adversary_vocabulary`) stay green under every mutant.
- **Tests passing for the wrong reason:** the malformed-input and duplicate-identifier lists name `release-observed` correctly. Dropping `service` from the case is refused at load, so no probe passes that way. `review_incident_subject` relies on Canon's subject filter: `reads()` in Canon 8d1599e, `eval/claims.rs`.

**6. Paths written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w12/els-incident-response-subject-binding/scratch/adversary-p2/` holds 16 logs:
  - `case-alone.log`
  - 8 × `mutant-*.log`
  - `gate-fmt.log`, `gate-fmt-2.log`, `gate-clippy.log`, `gate-clippy-2.log`, `gate-test.log`, `gate-test-nff.log`, `gate-docs.log`
- The scratch copy (1.8M) and its target dir (104M) are deleted.
- Lease `adversary-p2-irsb` is acquired and released.

```findings
- file: website/product.json
  line: 50
  category: contract-drift
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "The software.change/1 status row is marked shipped but carries no href to its generated page /docs/protocols/software-change/1, unlike the incident.response/1 row."
- file: crates/els/tests/adversary2_incident_harness.rs
  line: 155
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: undecided
  message: "an_artifact_the_case_omits accepts any missing-artifact refusal and stays green when every refusal names an artifact other than release; the acceptance test catches that mutant instead."
```
