---
format: aep.planning-md/3
id: review-result:adversary-w21-els-crate-rename-pass-1
kind: review-result
status: active
title: Wave 2026-10-05-w21 adversary, els story:crate-rename, pass 1
relations:
- reviews: story:crate-rename
revision: 1
---
```
unit: els/crate-rename, uncommitted worktree on top of 4fe56de (~/.local/state/worktree/trees/b10x/els/els-w21-crate-rename)
verdict: green
cases: executed 121→129, red 0
origin: introduced 2 / pre-existing 1 / undecided 0
wrote-outside-worktree: 1 (~/.cache/ga-wave-2026-10-05-w21/els-crate-rename/scratch/adv-suite.log)
needs-coordinator: yes (stale scope in seven draft stories; acceptance item 5 has no release process)
```

Cases added (8, green): `crates/canon-engineering/tests/adversary_rename.rs` (help and usage text name `canon-engineering`, never "els"; unknown subcommand exits 2; unknown-term error text literal; the acceptance grep has no hits outside an allowlist built from the coordinator's scope decisions; `has_word` matches `grep -iw`) and `crates/canon-engineering-docs/tests/adversary_rename.rs` (`--help` names `canon-engineering-docs`; a stale page fails `--check` and names `canon-engineering-docs generate`; fresh generation carries the new header and no project "ELS").

Suite after the cases: fmt exit 0; clippy exit 0; `cargo test --workspace --locked` exit 0, 129 passed; `generate --check` exit 0, "12 generated files fresh (2 protocols)".

| file:line | verdict | origin | finding | what reaches it |
|---|---|---|---|---|
| .engineering/planning/story/software-change-profiles.md:23 | NEEDS-CHANGE | introduced | Seven draft stories still scope `crates/els/…`, which the move removed. | Future wave briefs take their files from these scopes |
| Taskfile.yml:4 | CONFIRMED | introduced | `CARGO_TARGET_DIR` renamed to `b10x-target/canon-engineering` while the repository is still `els`; breaks `b10x-target/<repo>` and orphans the old directory. | `task check`, `task docs` |
| story:crate-rename acceptance item 5 | CONFIRMED | pre-existing | The repository has no release process (no release workflow, no tags, nothing in README or AGENTS). | The acceptance statement |

Could not break: 49 moved files with no assertion lost; `Cargo.lock` under `--locked`; `#[path]`, `include_str!`, `CARGO_BIN_EXE_*` and root joins; `-p` references in `pages.yml` and `Taskfile.yml`; site manifest and docusaurus routes unchanged; governor and intake still pin `b10x-els` at ac7dd03.

```findings
[
  {"file": ".engineering/planning/story/software-change-profiles.md", "line": 23, "category": "contract-drift", "severity": "warning", "verdict": "NEEDS-CHANGE", "origin": "introduced", "message": "Seven draft stories still give their scope under crates/els/, which the move removed, so future briefs derived from those scopes name paths that no longer exist."},
  {"file": "Taskfile.yml", "line": 4, "category": "judgement", "severity": "note", "verdict": "CONFIRMED", "origin": "introduced", "message": "CARGO_TARGET_DIR was renamed to b10x-target/canon-engineering while the repository is still els, which breaks the b10x-target/<repo> convention and orphans the old directory."},
  {"file": ".engineering/planning/story/crate-rename.md", "category": "acceptance", "severity": "warning", "verdict": "CONFIRMED", "origin": "pre-existing", "message": "Acceptance item 5 requires a release through the repository's own release process, but the repository has no release workflow, tags or documented process."}
]
```
