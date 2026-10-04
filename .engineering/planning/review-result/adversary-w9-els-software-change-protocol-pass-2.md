---
format: aep.planning-md/3
id: review-result:adversary-w9-els-software-change-protocol-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w9 adversary, els story:software-change-protocol, pass 2
relations:
- reviews: story:software-change-protocol
revision: 1
---
unit: els/software-change-protocol, working tree at 1978fc0 with the uncommitted F1 change on top
verdict: CONFIRMED (2 red cases)
cases: executed 91→94, red 2
origin: introduced 4 / pre-existing 0 / undecided 0
wrote-outside-worktree: 10 log files in scratch/adversary-p2/ (all listed in part 6)
needs-coordinator: yes. Does A1 hold the unit? Fixing it needs a new term or a Canon change. Two new cases are in files the brief did not name (see part 2).

**1. `git --no-pager diff --stat`** is unchanged from the F1 change: 7 files, +196 −26. My only writes are two untracked test files: `crates/els/tests/adversary2_software_change_protocol.rs` and `crates/els-docs/tests/adversary2_software_change_render.rs`. I touched no non-test path. The brief named only the first file. The second has to live in `els-docs` because only that crate's tests can run the `els-docs` binary.

**2. Cases added**

| Case | Asserts | Now | Red output, case run alone |
|---|---|---|---|
| `accepted_is_blocked_while_the_merge_is_denied_and_r2_never_released` | Takes CHG-1842's own case, where release v0 and deployment d0 are listed from `initial`. Adds tests passing on R2, the merge **denied**, build provenance for v0, d0 healthy and the objective satisfied about i1. Expects `accepted` blocked | red | `left: (True, "legitimate")` `right: (Unknown, "blocked")` |
| `the_committed_graph_shows_the_subject_of_every_evidence_match` | For each claim, the artifact its evidence matches are bound to appears in the committed graph's predicate or edge qualifiers | red | `left: ["deployment.healthy: about `deployment`", "implementation.reviewed: about `implementation`", "implementation.verified: about `implementation`", "objective.realized: about `intent`", "release.proven: about `release`", "tests.pass: about `implementation`"]` |
| `the_page_and_graph_do_not_depend_on_the_order_the_yaml_is_written_in` | Generates the shipped protocol twice: as written, and with every mapping and list reversed at every depth. Every generated file must be byte-identical | green | It can fail: a scratch mutant that writes the YAML's first line into the page turns it red (`left: {"docs/protocols/software-change/1.mdx"}`) |

Inverse check on the graph case: a scratch `els-docs` that emits `subject` and regenerates the graph turns it green (`1 passed`).

**3. Gate**, unit build dir, run after the cases existed: `EXIT fmt=0` `EXIT clippy=0` `EXIT test=101` `EXIT docs=0` (`els-docs: 11 generated files fresh (2 protocols)`). My first fmt run failed on my two files only. I ran rustfmt on those two and re-ran the whole gate.
```
tests/adversary2_software_change_protocol.rs  test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out;
tests/adversary2_software_change_render.rs    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;
```
The other 25 binaries are unchanged from the F1 report: each says `test result: ok. N passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;`. The `--no-fail-fast` total is 27 binaries, 92 passed, 2 failed. `--list` shows all three new tests in this tree.

**4. Findings**

- **A1** `protocols/software-change/1.yaml:63`, CONFIRMED / introduced / warning. `accepted` holds while the merge of R2 is denied. `release.proven` = `implementation.verified` (bound to R2) plus `build_provenance` about the **release revision** (v0), and nothing ties v0 to R2. The F2 fix therefore doesn't stop "accepted before merged, released or deployed".
  - What reaches it: chg-1842's own case snapshot. Canon requires a revision for every declared artifact, so a pre-change v0 and d0 always exist.
  - Design § 9 has the same `release.proven` but gates `released` with transitions and `production.deploy`, which Canon lacks.
  - Fix options: a merge or release evidence kind (a new term, outside the story's scope), Canon multi-revision binding (§ 9 `binds:` list, lines 671-675), or at least drop "comes from a verified implementation" from the `release.proven` description.
- **A2** `crates/els-docs/src/graph.rs:30`, NEEDS-CHANGE / introduced / warning. The committed graph is byte-identical to that of the unbound, pre-F1 protocol. The doc comment at :28 says "canon-ir/1 shape", and in canon-ir/1 a missing `subject` means "about any artifact".
  - Answer to "two matches drawn as one edge?": not in either shipped protocol, because each evidence kind has exactly one subject. A protocol with two matches of one kind and result but different subjects would collapse to one edge.
  - Fix within the 929f965 schema: carry the subject in the `establishes` edge `qualifier`, which is a free string (e.g. `pass about implementation`).
- **A3** `protocols/software-change/1.yaml`, CONFIRMED / introduced / note. Canon 8d1599e `check::check` reports 7 `unproduced-evidence` findings over 1536 states. `accepted` reads 3 evidence kinds that no action produces. The gate does not run `canon check`. From a scratch probe, not a case.
- **A4** `crates/els/tests/adversary_software_change_protocol.rs:383`, CONFIRMED / introduced / note. This pass-1 test's failure message and the doc comment at :371 say "design § 9 says intent" for `objective_observation`. § 9 declares no such evidence kind; see the first item in part 5 for the real basis.

**5. Attacked and could not break**
- **`objective_observation` → `intent`:** this is right by § 9 (lines 641-647), whose `intent` carries `required: objective`. That is the only place § 9 puts the objective. The F1 report cited the fixture instead, which the same unit wrote.
- **The four other bindings:** they match § 9's `evidence:` subjects exactly (lines 677-699).
- **Actions and outcomes:** `repository.merge` and `accepted` contain claim tests only, so there is no binding to check there. Their evidence comes through the claims, which are bound.
- **Canon bump:** no fixture pins decision bytes. The harness (`support/mod.rs` `check`, lines 675-718) compares only the statuses of claims, obligations and actions. It ignores `explanation` and `outcomes`. Invalidation still excludes nothing.
- **Determinism:** generated pages and graph do not change when the YAML is reordered (the case above). Canon's IR uses BTreeMaps and sorts predicates.

**6. Written outside the worktree**
- In `~/.cache/ga-wave-2026-10-04-w9/els-software-change-protocol/scratch/adversary-p2/`: `gate-{fmt,clippy,test,docs}.log`, `gate-test.log.nff`, `red-els.log`, `run-docs-determinism.log`, `mutant-docs-order.log`, `mutant-graph.log`, `probe1.log`.
- The scratch `copy/` (2.3M) and `target/` (162M) are deleted.
- The determinism test creates temporary roots under the unit build dir's `CARGO_TARGET_TMPDIR` and removes them itself.

```findings
- file: protocols/software-change/1.yaml
  line: 63
  category: acceptance
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "accepted is legitimate with the merge of R2 denied, because release.proven binds build_provenance only to the release revision and nothing ties the case's pre-existing release v0 to the verified implementation R2"
- file: crates/els-docs/src/graph.rs
  line: 30
  category: contract-drift
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "the committed protocol graph drops every evidence subject, so in canon-ir/1 terms it draws the unbound pre-F1 protocol; the schema's free-string edge qualifier could carry it"
- file: protocols/software-change/1.yaml
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "Canon 8d1599e check reports 7 unproduced-evidence findings on software.change/1, three of them on accepted, and the ELS gate does not run canon check"
- file: crates/els/tests/adversary_software_change_protocol.rs
  line: 383
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "the pass-1 test attributes objective_observation's subject intent to design section 9, which declares no such evidence kind; the real basis is the intent artifact's required objective field"
```
