---
format: aep.planning-md/3
id: review-result:adversary-w13-els-protocol-registry-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w13 adversary, els story:protocol-registry, pass 1
relations:
- reviews: story:protocol-registry
revision: 1
---
unit: els/protocol-registry, at 99d62db plus the uncommitted phase 2 in ~/.local/state/worktree/trees/b10x/els/els-w13-protocol-registry
verdict: NEEDS-CHANGE
cases: executed 95→99, red 1
origin: introduced 1 / pre-existing 0 / undecided 0
wrote-outside-worktree: 9 log files under ~/.cache/ga-wave-2026-10-04-w13/els-protocol-registry/scratch/adversary-p1/ (scratch copies already deleted)
needs-coordinator: yes. The fix that holds in both directions is outside the story's scope (`Taskfile.yml`'s shared build dir), or needs a design decision.

**1. Diff stat.** `git --no-pager diff --stat` shows only the implementor's tracked files, unchanged by me:
```
 Cargo.lock                            | 1 +
 crates/els/Cargo.toml                 | 5 +++++
 crates/els/src/lib.rs                 | 1 +
 crates/els/tests/protocol_registry.rs | 6 ++----
```
The one path I wrote is new and untracked: `crates/els/tests/adversary_protocol_registry.rs`. It is a test file. I ran no `git add`. The implementor's `git add -N` is already gone: the three new files show as `??`.

**2. Cases added** (in `crates/els/tests/adversary_protocol_registry.rs`)

| Case | Asserts | Now |
|---|---|---|
| `a_checkout_sharing_a_target_dir_embeds_its_own_protocols` | Two copies of the workspace share one target dir. Each copy's `els protocols show software-change@1` prints that copy's own edited file. It checks the same copy rebuilding after an edit, then the second copy, then the first copy again. | **red** |
| `a_malformed_show_argument_is_refused_on_stderr` | 8 argument forms (`x`, `x@`, `x@-1`, `x@4294967296`, `x@one`, empty, `@1`, `x@0`) each exit 1, print nothing on stdout, and print `els: …<argument>` on stderr | green; mutants M1 and M2 turn it red |
| `a_usage_error_exits_2` | A command line clap cannot parse exits 2, with nothing on stdout | green; mutant M4 turns it red |
| `list_and_show_print_every_builtin_exactly` | `list` prints exactly the sorted lines with newlines, and `show` matches every built-in byte for byte, not only software-change | green; mutant M3 turns it red |

Red output from running that case alone (`red-shared-target-2.log`; the line number was 215 before rustfmt, 233 after):
```
panicked at crates/els/tests/adversary_protocol_registry.rs:215:5:
assertion `left == right` failed: checkout two, built into a target dir checkout one used, embeds checkout two's protocols/software-change/1.yaml
  left: "...# adversary: edited in checkout one\n"   right: "...# adversary: edited in checkout two\n"
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 3 filtered out; finished in 2.59s
```
The two control steps before the assertion passed. A protocol edit in the same checkout does rebuild. Mutant results are in `mutants.log`: M1 malformed exits 0 (red :68), M2 refusal goes to stdout (red :73), M3 list without newline (red :113), M4 usage exits 1 (red :96).

**3. Gate** (`gate.log`; the commands are from `gate-els.txt`, run with the unit's build dir)
```
EXIT=0 [cargo fmt --all --check]
EXIT=0 [cargo clippy --workspace --all-targets --locked -- -D warnings]
test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.99s
EXIT=101 [cargo test --workspace --locked]
els-docs: 11 generated files fresh (2 protocols)
EXIT=0 [cargo run -q --locked -p els-docs -- generate --check]
```
The first gate run failed `fmt` on my own file. I ran rustfmt on that file alone and re-ran the whole gate; the output above is the second run. `--no-fail-fast` gives `passed=98 failed=1`, which is 99 executed, against the implementor's 95.

**4. Findings**
- **Shared target dir embeds the wrong checkout's protocols.** NEEDS-CHANGE, introduced.
  - **What was measured:** red at `adversary_protocol_registry.rs:233`.
  - **Cause:** `build.rs:23` emits an absolute `rerun-if-changed`, and `build.rs:33` writes absolute paths into `include_str!`. Cargo keys a workspace member's output by its path relative to the workspace root, so a second checkout reuses the first checkout's build-script output and judges it fresh.
  - **What reaches it:** `Taskfile.yml` sets `CARGO_TARGET_DIR: ~/.cache/b10x-target/els` for every worktree, and the acceptance says the test passes under `task check`. Wave 12 edits `incident-response/1.yaml` in another worktree.
  - **Effect:** a gate goes spuriously red. A binary that is not tested would ship the other tree's bytes.
  - **Fixes tried in scratch:**
    - `include_str!(concat!(env!("CARGO_MANIFEST_DIR"), …))` alone: still red.
    - Adding a relative `rerun-if-changed=../../protocols`: passes the second-checkout step, then fails at step 4 (first checkout rebuilt after the second), red at :224 (`candidate-fix-both.log`).
  - **Fix that holds in both directions:** give each tree its own target dir in the Taskfile (outside the story's scope), or a design decision.
- **`show` accepts non-standard major numbers.** Judgement, note, CONFIRMED, introduced, at `main.rs:49`. `show software-change@01` and `@+1` exit 0 and print the protocol, but `build.rs:76` refuses `01.yaml` as a file name. A malformed argument also exits 1, the same as an unknown protocol, not clap's usage code 2. Nothing is harmed.

**5. Attacked and not broken**
- **Odd directory and file names:** uppercase, a dot, or a leading zero or `+` in the major fail the build, by design and matching els-docs. A non-UTF-8 name panics. `.yml`, stray files and nested directories are skipped silently.
- **Symlinked directory:** `build.rs` follows it, els-docs does not. This is a divergence I did not build a case for.
- **Rebuild in the same tree:** a protocol edit is embedded on the next build (checked by the control step).
- **Determinism:** entries are sorted by (name, major), and the bytes are `include_str!` of the file. `list` and `show` output is byte-exact with the trailing newline.
- **`get`:** it parses and validates through Canon on every call. I found no panic path. The registry is keyed by directory name, so `software.change@1` (the document id) is refused as unknown. That follows the story.
- **CLI streams and exit codes:** stdout and stderr are kept apart, refusals exit 1, usage errors exit 2.
- **Concurrency with wave 12:** no test pins the content or hash of `incident-response/1.yaml`. The tests read files from disk at run time.
- **Supply chain:** `Cargo.lock` gains one line (`"clap"` under b10x-els). It is the same version and features as els-docs, and no new crate is added.

**6. Paths written outside the worktree**
- Logs, kept, in `~/.cache/ga-wave-2026-10-04-w13/els-protocol-registry/scratch/adversary-p1/`: `red-shared-target.log`, `red-shared-target-2.log`, `cli-cases.log`, `mutants.log`, `candidate-fix.log`, `candidate-fix-envdep.log`, `candidate-fix-both.log`, `gate.log`, `suite-no-fail-fast.log`.
- Deleted:
  - `scratch/adversary-p1/mut` (540K)
  - `scratch/adversary-p1/mut-target` (205M)
  - `~/.cache/b10x-target/els-w13-protocol-registry/tmp/adversary-registry-shared` (101M; the red case leaves this behind each time it fails)
- I compiled into the unit's build dir, `~/.cache/b10x-target/els-w13-protocol-registry`.
- My lease `w13-protocol-registry-adversary-p1` is released.

```findings
- file: crates/els/build.rs
  line: 23
  category: concurrency
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "Absolute rerun-if-changed and include_str! paths let a checkout sharing a target dir (Taskfile's ~/.cache/b10x-target/els) embed another checkout's protocol files, red at adversary_protocol_registry.rs:233."
- file: crates/els/src/main.rs
  line: 49
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "show accepts non-standard majors (@01, @+1) that build.rs refuses as file names, and exits 1 rather than clap's usage code 2 for a malformed argument."
```
