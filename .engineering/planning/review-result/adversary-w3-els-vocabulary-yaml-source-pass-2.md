---
format: aep.planning-md/3
id: review-result:adversary-w3-els-vocabulary-yaml-source-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w3 adversary, els story:vocabulary-yaml-source, pass 2
relations:
- reviews: story:vocabulary-yaml-source
revision: 1
---
unit: els/vocabulary-yaml-source, uncommitted working tree on 4d773af (base feab40c)
verdict: NEEDS-CHANGE (one acceptance/scope finding; no red case)
cases: executed 14→20, red 0
origin: introduced 3 / pre-existing 0 / undecided 0
wrote-outside-worktree: 7 paths (scratch, listed in part 6)
needs-coordinator: whether "at least 35" is the intended contract; if it is, the story's acceptance item 1, item 4 and Shared surface need amending

**Result:** I could not make anything fail. Writing YAML out and reading it back keeps every meaning byte-for-byte, and the relaxed pin is sound. Two gaps are real: 11 mutants passed every test in the tree until my new cases existed, and the tree no longer checks the story's "exactly 35" (acceptance item 1).

**1. Diff stat**
```
 Cargo.lock                               |   2 +
 crates/els/Cargo.toml                    |   3 +-
 crates/els/src/vocabulary.rs             | 525 +++++++++++++++++--------------
 crates/els/tests/adversary_vocabulary.rs |  18 +-
 crates/els/tests/vocabulary_yaml.rs      |  48 ++-
?? crates/els/tests/adversary2_yaml_errors.rs      (mine)
?? crates/els/tests/adversary2_yaml_roundtrip.rs   (mine)
?? crates/els/tests/adversary_yaml_reader.rs       (pass 1)
```
All tracked changes are the implementor's. My only paths are the two new test files, so I changed no implementation file.

**2. Cases added** (all pass now; each test file was run on its own before the suite)

| Case | Asserts | Kills mutant |
|---|---|---|
| `adversary2_yaml_errors.rs::adversary2_yaml_refusals_name_the_offending_id` | Spelling, DuplicateTerm, NoMeaning and Format messages name the id or format; spelling names the right rule | A, B, J |
| `…::adversary2_yaml_parse_refusal_locates_the_entry` | a parse refusal says `terms[1]` and `category` | C |
| `…::adversary2_yaml_refuses_each_edge_of_the_spelling_rule` | 12 edge ids are refused as `Spelling{id,category}` (`_leading`, `trailing_`, `with2digit`, `kebab-case`, `café`, `release.`, `.proven`, …) | D, E, F, G, K |
| `…::adversary2_yaml_builder_appends_in_order_and_lookup_refuses_by_name` | `with_term` keeps order; `Vocabulary::lookup` refusal names the id | H, I |
| `adversary2_yaml_roundtrip.rs::adversary2_yaml_to_yaml_preserves_every_meaning_byte_for_byte` | 55 meanings survive write-then-read byte-for-byte, and writing again gives the same text (whitespace, line endings, YAML symbols, control characters, very long lines) | none: probe, no defect |
| `…::adversary2_yaml_to_yaml_of_unchecked_ids_is_refused_by_name` | ids `with_term` never checked (`a\nb`, `null`, `""`, …) are written without a panic and refused by name, never read back as a different id | none: probe |

Runs: `adversary2-roundtrip-green.log` (1 passed), `adversary2-roundtrip-second-run.log` (2 passed), `adversary2-errors-first-run.log` (4 passed).

**Mutants** (on a copied tree in scratch, `adversary2-mutants.log`): A spelling message drops the id, B duplicate message drops the id, C parse message dropped, D digits allowed in ids, E leading `_` allowed, F trailing `_` allowed, G non-ASCII lowercase allowed, H `with_term` adds to the front, I lookup refusal without the id, J rule text swapped, K hyphen allowed. Every one left all other test targets green and failed only my file (EXIT=101 each). The copy and its build dir are deleted.

**3. Suite** (run after the cases existed)
- `cargo fmt --all --check`: exit 0, after running rustfmt on my two files only.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: exit 0.
- `cargo test --workspace --locked`: exit 0, 20 passed / 0 failed across 8 result lines.
- The same with `-- --skip adversary2_` (my 6 cases left out): 14 passed. That 14 is the "before" number.
- `cargo test -- --list` shows all 6 `adversary2_*` tests in this tree.

**4. Findings**

| # | file:line | Verdict / origin | What was measured | What reaches it |
|---|---|---|---|---|
| F1 | `crates/els/tests/vocabulary_yaml.rs:237`, `crates/els/tests/adversary_vocabulary.rs:100` | NEEDS-CHANGE / introduced | Item 1 says "exactly 35 entries", and the phase-1 commit 4d773af checked `== 35`. Now both checks are `>= 35`. The story says it "does not edit adversary_vocabulary.rs" (Shared surface) and that file must "pass unedited" (item 4), yet it was edited. The base version of that test passes on the current 35-term YAML, so this story did not need the edit. The implementor's mutant N7 (`mutant-later-term.log`, EXIT=0): adding a 36th term in this story leaves everything green. | Phase 2 changed the phase-1 tests; the brief says to stop in that case. Either restore the exact pins, or amend the story to "the first 35, in order". |
| F2 | `crates/els/src/vocabulary.rs:188, 136, 209, 282` | CONFIRMED / introduced | 11 mutants (list above) passed all pre-existing tests. My cases now catch every one. | The public `from_yaml` / `with_term` / `Vocabulary::lookup` API. No further change needed if my file is kept. |
| F3 | `crates/els/src/vocabulary.rs:136` | CONFIRMED / introduced | The spelling refusal prints the Rust name ("`planned` is a ClaimId") where the YAML says `claim_id`. | A person fixing a YAML file. Note only. |

**5. Attacked and could not break**
- **Writing then reading back:** 55 meanings and 12 unchecked ids, no loss and no panic. serde_yaml_ng puts strings with line breaks in a literal block and quotes values that look like null, booleans or numbers.
- **Relaxed pin:** moving, inserting before, or changing the id, category, marking or meaning of any of the first 35 terms fails both pinned tests (the implementor's mutant N6 is the swap case). A duplicate added later is refused by `from_yaml`, so the built-in vocabulary panics on first use and the tests fail.
- **One rule set or two copies?** There are two copies of the spelling rule: `src/vocabulary.rs` and the test oracle in `tests/vocabulary.rs`. They cannot drift silently, because the released file always goes through `from_yaml`, and `vocabulary_yaml.rs` item 2 compares the file read at run time with the copy built into the library. The acceptance test's own YAML parser is stricter: it rejects numeric meanings and extra fields with a panic.
- **Panics left:** the built-in vocabulary panics on first use if the released YAML is invalid (`:296`, caught by every vocabulary test), and `to_yaml` has an `expect` (`:205`) that never fired on any input I tried.

I did not acquire a worktree session lease; the coordinator holds the tree.

**6. Paths written outside the worktree** (all under `~/.cache/ga-wave-2026-10-04-w3/els-vocabulary-yaml-source/scratch/`)
`adversary2-mutants.sh`, `adversary2-mutants.log`, `adversary2-roundtrip-green.log`, `adversary2-roundtrip-second-run.log`, `adversary2-errors-first-run.log`, `adversary2-suite.log`, `adversary2-suite-deselected.log`. I also created and deleted `adversary2-mutant/`, `adversary2-mutant-target/` (125M) and `adversary2-vocabulary.rs.orig`.

**7.**
```findings
- file: crates/els/tests/vocabulary_yaml.rs
  line: 237
  category: acceptance
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "Acceptance item 1 (exactly 35 entries) and item 4 (adversary_vocabulary.rs passes unedited) are no longer what the tree checks: phase 2 relaxed both pins to at least 35, so a 36th term added by this story stays green (N7 EXIT=0); either restore the exact pins or amend the story."
- file: crates/els/src/vocabulary.rs
  line: 188
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "11 mutants (refusal messages dropping the id or the parse message, widened spelling rule, with_term prepending, unnamed lookup refusal) passed all pre-existing tests; tests/adversary2_yaml_errors.rs now catches each one."
- file: crates/els/src/vocabulary.rs
  line: 136
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "The spelling refusal prints the Rust variant name (ClaimId) where the vocabulary document spells claim_id."
```
