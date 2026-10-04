---
format: aep.planning-md/3
id: review-result:adversary-w2-els-els-vocabulary-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w2 adversary, els story:els-vocabulary, pass 1
relations:
- reviews: story:els-vocabulary
revision: 1
---
```
unit: els/els-vocabulary — working tree at 943992c plus uncommitted changes, worktree els-w2-els-vocabulary
verdict: CONFIRMED (3 suite gaps; implementation holds against the story)
cases: executed 3→6, red 0
origin: introduced 3 / pre-existing 0 / undecided 0
wrote-outside-worktree: 34 paths (part 6)
needs-coordinator: none
```

The implementation holds, and the story's table matches exactly: 35 terms, 24 core and 11 `software.change`. Nothing I ran fails against the tree as it stands. But the unit's suite stays green on 7 of the 14 mutants I tried. My three new cases are green now and kill those mutants. None of the three gaps should hold the unit.

**1. Diff stat**

`git --no-pager diff 943992c --stat` lists only the unit's tracked changes: `crates/els/src/lib.rs` (+2) and `docs/examples/incident-response.md` (12 lines). My only write in the worktree is the untracked test file `crates/els/tests/adversary_vocabulary.rs`. I changed no non-test path.

**2. Cases added** (in `~/.local/state/worktree/trees/b10x/els/els-w2-els-vocabulary/crates/els/tests/adversary_vocabulary.rs`)

| Test | What it asserts | Now | Mutants it kills |
|---|---|---|---|
| `adversary_vocabulary_is_exactly_the_story_table` | `terms()` is exactly the story table: 35 entries, each id with its category and marking | green | extra term (output: `left: 36 right: 35`, :59) |
| `adversary_lookup_refuses_partial_and_variant_spellings` | `lookup` refuses `""`, `tests`, `release.`, `Service.Healthy`, `" service.healthy"` and 7 more variants, each refusal naming the term | green | prefix match, substring match (output: `` `` must be refused: "intent"``), case-folding, trimming |
| `adversary_every_dotted_id_in_example_text_blocks_resolves` | every dotted token in a ```text block resolves to exactly one claim or action entry; it does not use the unit's header/status rule | green | example with `repository.force_push = denied`, and a bare id after a blank line under `Actions:` (output: `` `repository.force_push` names 0 entries``, :108) |

I ran the file alone first: 3 passed, exit 0. Nothing in the tree is red.

**3. Gate in the worktree**, run after my file existed, with the assigned build dir:
- `cargo fmt --all --check`: exit 0. My first version failed it; I ran `rustfmt` on my file only.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: exit 0.
- `cargo test --workspace --locked`: exit 0. Results: lib 1 passed, `adversary_vocabulary` 3 passed, `vocabulary` 2 passed, doc-tests 0. The `before` count of 3 is this run with my binary left out.

**Mutants**, run on a copy in scratch, never on the tree:

| Mutant | Unit suite | Adversary cases |
|---|---|---|
| flip `release.rollback` to `software.change` | red (vocabulary.rs:81) | red |
| drop `emergency.leave` | red (:63) | red |
| `traffic.shift` category to Claim | red (:70) | red |
| refusal message without the term | red (:116) | red |
| duplicate `logs.search` | red (:63) | red |
| incident example back to `customer_impact_bounded` | red (:98) | green (it has no dot) |
| example action under `Admissible actions:` | red (:93, only through the `any(ActionId)` check) | red |
| extra term `rollback.verified` | **green** | red |
| `lookup` by prefix / substring / case-folded / trimmed (4 mutants) | **green** | red |
| example line `repository.force_push = denied` | **green** | red |
| blank line inside `Actions:`, then a bare undeclared id | **green** | red |

One earlier batch of results was invalid and I threw it away. rsync restored old file times on the copy, so cargo reused the binary built from the `duplicate` mutant. I touched every file after each sync and reran all 15 runs; the table above is from that rerun.

**4. Findings** (all on the working tree above)

| # | file:line | Finding | Verdict / origin | What reaches it |
|---|---|---|---|---|
| 1 | crates/els/tests/vocabulary.rs:54 | `assert_eq!(TABLE.len(), 35)` only checks the test's own copy of the table, never the vocabulary. A vocabulary with an extra term passes. | CONFIRMED / introduced | Five later stories edit `vocabulary.rs`; a term added outside its story would pass this suite. |
| 2 | crates/els/tests/vocabulary.rs:113 | Expectation 4 checks only two misses, so lookup by prefix, substring, case or trimming all pass. Example: `lookup("")` returns `intent`. | CONFIRMED / introduced | `lookup` is the only resolver later protocol stories will call. The current code uses `==` and is correct (vocabulary.rs:93). |
| 3 | crates/els/tests/vocabulary.rs:195-213 | The invented extraction rule skips any id whose status is outside `admissible`, `approval_required` and `blocked` and is not under `Actions:`. It also forgets the section after a blank line. Either way an undeclared id passes silently. | INFEASIBLE / introduced | Nothing today: both examples as left are fully covered. It would bite once an example gains a new status. |

**5. What held**
- All 35 ids, categories and markings match the story table, with no duplicates.
- The incident example is respelled exactly as the story says. `software-change.md` is unchanged and resolves.
- Every claim and action id in both examples' text blocks resolves.
- Both refusals in expectation 4 name the term.
- Spelling holds: claim and action ids are dotted, the rest are snake_case.
- No Canon-semantic leakage: the meaning strings are names only, the module has no truth, evidence or obligation logic, and it uses `ClaimId` and `ActionId` only as identifiers.

**6. Paths I wrote outside the worktree** (all under `~/.cache/ga-wave-2026-10-04-w2/els-els-vocabulary/scratch/`):
- `adv-mutate.sh` (the mutation runner)
- `adv-pristine/` (192K)
- `adv-mut/` (192K)
- `adv-target/` (15M, the build dir for the copy)
- `adv-mut-<mutant>-{unit,adv}.log` (30 files)

I took and released my own worktree lease, `adversary-els-vocabulary-p1`.

**7. Findings block**
```findings
- file: crates/els/tests/vocabulary.rs
  line: 54
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: the 35-term count asserts the test's own fixture, so a vocabulary carrying an undeclared extra term stays green; adversary_vocabulary_is_exactly_the_story_table kills it
- file: crates/els/tests/vocabulary.rs
  line: 113
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: prefix, substring, case-folding and trimming mutants of lookup survive because expectation 4 probes only two non-overlapping misses; adversary_lookup_refuses_partial_and_variant_spellings kills them
- file: crates/els/tests/vocabulary.rs
  line: 195
  category: boundary
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: the example extraction rule silently skips an undeclared id carrying a status outside its closed list or listed after a blank line under Actions:, which no current example contains
```
