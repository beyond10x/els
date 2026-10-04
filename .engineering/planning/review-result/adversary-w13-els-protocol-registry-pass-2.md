---
format: aep.planning-md/3
id: review-result:adversary-w13-els-protocol-registry-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w13 adversary, els story:protocol-registry, pass 2
relations:
- reviews: story:protocol-registry
revision: 1
---
unit: els/protocol-registry, working tree = 99d62db + uncommitted phase 2 + pass-1 fixes, in ~/.local/state/worktree/trees/b10x/els/els-w13-protocol-registry
verdict: NEEDS-CHANGE
cases: executed 98→101, red 2
origin: introduced 2 / pre-existing 0 / undecided 0
wrote-outside-worktree: 22 log files under scratch/adversary-p2/ (copy and target deleted)
needs-coordinator: yes. Choose which name rule wins. I propose els-docs' rule. Also decide whether to keep the heavier case that builds a copy of the workspace (about 3 s with sccache, about 230M of target while it runs, deleted on exit).

**1. Diff stat.** The tracked diff is the implementor's and I did not touch it:
```
 AGENTS.md                             | 3 +++
 Cargo.lock                            | 1 +
 crates/els/Cargo.toml                 | 5 +++++
 crates/els/src/lib.rs                 | 1 +
 crates/els/tests/protocol_registry.rs | 6 ++----
```
The only path I wrote in the worktree is the new untracked test file `crates/els/tests/adversary2_protocol_registry.rs`. I ran no `git add`.

**2. Cases added**

| Case | Asserts | Now | Proven able to fail |
|---|---|---|---|
| `get_refuses_with_the_variant_that_names_what_is_missing` | `UnknownName`/`UnknownMajor` matched with their field values; exact `Display` text; `get` twice gives equal results for every built-in | green | mutant L1 (swap the variants) turns it red at :36. The acceptance stays green under L1, because both variants print `name@major` |
| `show_refuses_a_name_els_docs_refuses_as_malformed` | `show -- 1@1`, `2fa@1`, `-x@1`, `--@1`, `-@1` exit 2 | **red** | — |
| `build_embeds_only_what_the_rule_and_els_docs_accept` | builds a copy with its own target dir. 1: empty `protocols/` builds, `list` prints nothing (exit 0), `show` exits 1. 2: `01.yaml` fails the build. 3: `Software/` fails the build. 4: `2fa/` fails the build | **red at step 4** | B1 (looser major in build.rs) turns it red at step 2; B2 (name check dropped in build.rs) turns it red at step 3 |

Red output from running this file alone before any suite run (`red-alone.log`; :212 became :219 after rustfmt):
```
panicked at crates/els/tests/adversary2_protocol_registry.rs:75:9:
assertion `left == right` failed: els protocols show -- "1@1": els-docs refuses the name, so it is malformed: Output { status: ExitStatus(unix_wait_status(256)), stdout: "", stderr: "els: 1@1: no built-in protocol is named `1`\n" }
  left: Some(1)
 right: Some(2)
panicked at crates/els/tests/adversary2_protocol_registry.rs:212:5:
step 4: els-docs refuses protocols/2fa/, so the build that claims its rule fails too: exit Some(0), 
test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 8.50s
```
The proposed fix turns all 3 cases green (`m2-R1-fix-leading-letter.out`): `is_name` in builtin_name.rs:6 requires a leading lowercase letter, like els-docs. With that fix the existing suite also stays 76/76.

**3. Gate** (`gate.log`, the commands from `gate-els.txt`, unit build dir):
```
EXIT=0 [cargo fmt --all --check]
EXIT=0 [cargo clippy --workspace --all-targets --locked -- -D warnings]
test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.62s
EXIT=101 [cargo test --workspace --locked]
els-docs: 11 generated files fresh (2 protocols)
EXIT=0 [cargo run -q --locked -p els-docs -- generate --check]
```
- The `--no-fail-fast` run gives passed=99, failed=2.
- `--list` shows 101 tests, including the 3 new names.

**4. Findings**
- **The rule is not the one els-docs applies** (builtin_name.rs:3 says it is). NEEDS-CHANGE, introduced.
  - **What was measured:** `is_name` accepts names that start with a digit or hyphen; els-docs (generate.rs:39) does not.
  - **Effect:** `protocols/2fa/1.yaml` builds, and `els protocols list` prints `2fa@1`. els-docs refuses the same tree with exit 1 (`divergence-trees.log` D1). `show 1@1` exits 1 rather than 2.
  - **What reaches it:** a protocol directory whose name starts with a digit. `task check` would then fail at the docs step, not at the build.
- **No test exercised build.rs's use of the rule.** CONFIRMED, introduced.
  - **What was measured:** mutants B1 and B2 keep the copy's suite green, 76/76 (`mutants-p2.log`).
  - **Effect:** the build-side checks can be loosened or dropped with nothing turning red. My build-copy case now catches both mutants.
- **Symlinked protocol directory** (build.rs:55). INFEASIBLE, introduced.
  - **What was measured:** `is_dir()` follows the link, so `alias@1` is listed. els-docs skips it silently and `generate --check` still says fresh (2 protocols) (D2).
  - **What reaches it:** nothing found. I wrote no case for it.

**5. Attacked and not broken**
- **Rebuild after editing the rule:** editing builtin_name.rs makes cargo report it Dirty, recompile the build script and rerun it (`e1-rebuild-on-rule-change.log`).
- **The CLI's use of the rule:** mutants C1 and C2 are caught by the pass-1 case at :67.
- **Exit codes:**
  - `--help`, `-h`, `help` and every subcommand's help exit 0 with output on stdout.
  - Usage errors exit 2 with output on stderr only.
  - Whitespace, `@1@1` and non-ASCII digits exit 2.
  - `vocabulary@1` exits 1.
  - `--version` and `-V` exit 2. The story does not require them and els-docs has neither.
- **Empty `protocols/`:** it builds, `list` prints nothing (exit 0) and `show` exits 1, with no panic. A major above u32::MAX fails the els build loudly (D3).
- **The AGENTS.md note:**
  - It sits directly under the Taskfile build-dir line, and nothing else documents that dir.
  - Its remedy works: with task 3.53.1, a shell `CARGO_TARGET_DIR` overrides the Taskfile `env:` (`taskfile-env-probe.log`).
- **Library API:** the variants are public with public fields, and `Display` names `name@major`.

**6. Paths written outside the worktree.** All are under `~/.cache/ga-wave-2026-10-04-w13/els-protocol-registry/scratch/adversary-p2/`, and all are kept:
- `cli-probe.log`, `divergence-trees.log`, `e0-build.log`, `e1-rebuild-on-rule-change.log`, `gate.log`, `list.log`
- `mutants-p2.log`, `mutants-p2-cases.log`, `red-alone.log`, `suite-no-fail-fast.log`, `taskfile-env-probe.log`
- `m-{control,B1-build-major-loose,B2-build-name-any,C1-cli-major-loose,C2-cli-name-any,R1-shared-name-leading-letter}.out`
- `m2-{L1-variant-swap,B1-build-major-loose,B2-build-name-any,R1-fix-leading-letter}.out`

Deleted:
- `copy/` (2.4M), `target/` (234M), `taskprobe/`, and the stale `m-B3-build-zero-ok.out`.
- The build case's `$CARGO_TARGET_TMPDIR/adversary2-registry-build`, which a Drop guard removes even when the case fails.

I compiled into the unit's build dir for the red run and the gate. My lease `w13-protocol-registry-adversary-p2` is released.

```findings
- file: crates/els/src/builtin_name.rs
  line: 3
  category: contract-drift
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "is_name accepts names starting with a digit or hyphen that els-docs refuses, so protocols/2fa/1.yaml builds and lists as 2fa@1 while els-docs generate fails, and show 1@1 exits 1 not 2; red at adversary2_protocol_registry.rs:75 and :219."
- file: crates/els/build.rs
  line: 63
  category: mutant
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "Replacing build.rs's name or major check with a looser one keeps the existing suite green 76/76; only the new build-copy case catches it."
- file: crates/els/build.rs
  line: 55
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "A symlinked protocol directory is embedded and listed by els but skipped silently by els-docs, whose generate --check stays fresh; nothing found that creates one."
```
