---
format: aep.planning-md/3
id: review-result:adversary-w3-els-vocabulary-yaml-source-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w3 adversary, els story:vocabulary-yaml-source, pass 1
relations:
- reviews: story:vocabulary-yaml-source
revision: 1
---
unit: els/vocabulary-yaml-source, working tree on 4d773af (phase 1) plus the implementor's uncommitted phase 2, plus my one new test file
verdict: CONFIRMED (3 red cases; 2 plan conflicts proven on a scratch copy)
cases: executed 8→13, red 3
origin: introduced 3 / pre-existing 2 / undecided 0
wrote-outside-worktree: 3 paths (part 6)
needs-coordinator: yes. Later stories must edit test files their scope leaves out (finding 1). Decide whether `from_yaml` should enforce the vocabulary's own rules (finding 3).

## 1. Diff stat
```
 Cargo.lock                   |   2 +
 crates/els/Cargo.toml        |   3 +-
 crates/els/src/vocabulary.rs | 412 +++++++++++++++++++------------------------
?? crates/els/tests/adversary_yaml_reader.rs
```
The three tracked paths are the implementor's uncommitted phase 2, not mine. My only write in the tree is the untracked test file. No implementation file and no YAML was touched.

## 2. Cases added (`crates/els/tests/adversary_yaml_reader.rs`), run alone first (exit 101, log in `scratch/adversary1-red.log`)
| Case | Asserts | Now |
|---|---|---|
| `..._refuses_ids_the_vocabulary_cannot_spell` | `from_yaml` refuses an undotted `claim_id`, uppercase ids, a dotted evidence kind, spaces, an empty id | RED: `` `servicehealthy` as claim_id was read: Ok([("servicehealthy", Some("servicehealthy"))]) `` |
| `..._refuses_an_id_padded_into_a_second_entry` | `intent` and `"intent "` are not both accepted | RED: `both read: Ok(["intent", "intent "])` |
| `..._refuses_a_term_without_a_meaning` | empty or null `meaning` is refused | RED: `` `meaning: ""` was read as Ok([""]) `` |
| `..._reads_yaml_special_meanings_verbatim` | colons, `#`, quotes, folded and literal blocks, unicode, trailing comment | green (probe) |
| `..._refuses_case_variants_and_stray_documents` | category/marking spelled with different case, a second `---` document, a missing `terms` | green (probe) |

## 3. Suite, run after the cases existed
`cargo test --workspace --locked --offline --no-fail-fast` gave EXIT=101. The only failures are the 3 above. Every existing binary is green: unit 2/2, adversary_vocabulary 3/3, vocabulary 2/2, vocabulary_yaml 1/1. `cargo clippy ... -D warnings` gave 0. `cargo fmt --all --check` gave 0, after I ran rustfmt on my own file.

## 4. Findings
| # | file:line | Verdict / origin | Measured | What reaches it |
|---|---|---|---|---|
| 1 | `crates/els/tests/vocabulary_yaml.rs:235` | CONFIRMED / introduced | On a scratch copy with one term appended to the YAML (`rollback.verified`, the only term `story:rollback-verification-rule` adds): `protocols/vocabulary.yaml holds 36 entries left: 36 right: 35` | Every later story that adds a term. Their typed scopes (software-change-profiles, negative-outcomes, ess-conformance-evidence, security-independence-rules, rollback-verification-rule) list `protocols/vocabulary.yaml` but not this test file. As written, none of them can go green. The fix is in the plan: put the test files in their scope, or assert the 35 terms as the file's prefix. |
| 2 | `crates/els/tests/adversary_vocabulary.rs:107` | CONFIRMED / pre-existing | The same scratch run: `left: 36 right: 35` | The same later stories. The file is unchanged since feab40c, so a later story can only pass by editing an adversary case, which rule 2 forbids. |
| 3 | `crates/els/src/vocabulary.rs:150` | INFEASIBLE / introduced | The 3 red cases. `from_yaml` checks only format, duplicate ids, and unknown fields, categories and markings. The spelling and non-empty-meaning rules in the module doc (:11-12) and in `tests/vocabulary.rs` apply only to the released file. | Nothing found. `from_yaml` has no caller outside tests, and the released file is checked by `tests/vocabulary.rs`. |
| 4 | `crates/els/src/vocabulary.rs:25` | INFEASIBLE / introduced | `include_str!("../../../protocols/vocabulary.yaml")` reaches outside the package. `cargo package --list` leaves the YAML out, so a packaged or vendored copy would not compile. | Nothing found. `cargo package` already fails on the versionless `b10x-canon` git dependency, and no repo under ~/beyond10x depends on b10x-els. |
| 5 | `crates/els/src/vocabulary.rs:28` | CONFIRMED / pre-existing | `Category` and `Marking` are closed Rust enums. A term with a new category or marking cannot be added through the YAML alone. `Vocabulary` also cannot be built in Rust and written out as YAML, which is the other direction ADR 0077 point 3 names. | The plan accepts that categories are added in Rust (the later stories' scopes). This is a judgement note only. |

## 5. Attacked and could not break
- File order, lookup of all 35 terms, and the reader agreeing with the file (item 2 of the acceptance holds).
- YAML-special meanings, and case variants of category and marking (the probes above).
- Duplicate keys and multiple documents are refused.
- A malformed embedded file panics on first use with a clear message (:197), and every test that calls `terms()` would go red.
- Lifetimes: `Term<'static>` borrows a `LazyLock`; nothing leaks.
- Hard-coding the table in another source file passes the source scan (it reads only `src/vocabulary.rs`). The item-2 comparison of `terms()` against the file catches it as soon as the YAML changes.

## 6. Paths written outside the worktree
- `~/.cache/ga-wave-2026-10-04-w3/els-vocabulary-yaml-source/scratch/adversary1-red.log`
- `~/.cache/ga-wave-2026-10-04-w3/els-vocabulary-yaml-source/scratch/mutant-later-term.log`
- `~/.cache/ga-wave-2026-10-04-w3/els-vocabulary-yaml-source/scratch/mutant-later-term/` and `.../scratch/mutant-target/`, both already deleted.

## 7. Findings block
```findings
- file: crates/els/tests/vocabulary_yaml.rs
  line: 235
  category: contract-drift
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "The acceptance test pins exactly 35 file entries, so every later story that appends a term to protocols/vocabulary.yaml goes red here, and none of those stories has this test file in its typed scope."
- file: crates/els/tests/adversary_vocabulary.rs
  line: 107
  category: contract-drift
  severity: warning
  verdict: CONFIRMED
  origin: pre-existing
  message: "The wave-2 adversary case pins terms().len() == 35 and the exact table, so a later story that only appends a YAML term can pass only by editing an adversary case outside its scope."
- file: crates/els/src/vocabulary.rs
  line: 150
  category: boundary
  severity: warning
  verdict: INFEASIBLE
  origin: introduced
  message: "from_yaml reads ids that break the module's spelling rule (an undotted claim_id, uppercase, spaces, empty, a whitespace-padded duplicate) and empty meanings, all of which tests/vocabulary.rs forbids in the released file; nothing outside tests calls it."
- file: crates/els/src/vocabulary.rs
  line: 25
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "include_str! reaches outside the package, so a packaged or vendored copy of b10x-els would not compile; cargo package already fails on the git dependency and no consumer exists."
- file: crates/els/src/vocabulary.rs
  line: 28
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: pre-existing
  message: "Category and Marking are closed Rust enums and a Vocabulary cannot be built in Rust and written out as YAML, so the reader still defines the vocabulary's schema and supports only one of the two directions ADR 0077 point 3 names."
```
