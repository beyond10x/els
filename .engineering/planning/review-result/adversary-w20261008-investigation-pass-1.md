---
format: aep.planning-md/3
id: review-result:adversary-w20261008-investigation-pass-1
kind: review-result
status: active
title: Wave 2026-10-08-w1 adversary, story:incident-investigation-obligation, pass 1
relations:
- reviews: story:incident-investigation-obligation
revision: 1
---
unit: story:incident-investigation-obligation
verdict: red
cases: executed 113→116, red 1
origin: introduced 1, pre-existing 0, undecided 0
wrote-outside-worktree: $HOME/.cache/engineering-protocols-w20261008/investigation-adversary/ (case-alone.log, suite.log, suite-nff.log, docs.log)
needs-coordinator: yes

The work is red on one point, and this needs your decision. A cause analysis of the **release** discharges `investigate_cause` while the service is at `s2`. The unit's own description and example page say it stays open until the **service's** current revision is analysed. The stale-`s1` claim holds.

**1. Diff stat** (`1a16b0f..1d25ac4`, committed as the bot, not pushed). Only a test file changed; no implementation file was touched.
```
 .../tests/adversary_investigation_obligation.rs    | 122 +++++++++++++++++++++
 1 file changed, 122 insertions(+)
```

**2. Cases added** in `crates/canon-engineering/tests/adversary_investigation_obligation.rs`. Each one swaps out only `cause-2` in inc-492's last state.

| Case | Asserts | Now |
|---|---|---|
| `a_cause_analysis_of_the_release_leaves_the_service_investigation_open` | `cause_analysis` subject `release` rev `r42` → `investigate_cause` open | red |
| `a_late_cause_analysis_of_the_stale_service_revision_leaves_it_open` | subject `service` rev `s1` after restore → `cause.identified` UNKNOWN, open, `emergency.leave` admissible | green |
| `an_inconclusive_cause_analysis_leaves_it_open` | result `inconclusive` on `s2` → open | green |

Red output from running the case on its own, verbatim (after one compile fix of my own code):
```
assertion `left == right` failed: a cause analysis of release r42 discharged the investigation of service s2 (cause.identified = True)
  left: "discharged"
 right: "open"
```

**3. Package suite**, run after the cases existed:

| Command | Exit | Result |
|---|---|---|
| `cargo test -p b10x-canon-engineering --locked --no-fail-fast` | 101 | 115 passed, 1 failed (the red case above) |
| `cargo run --locked -p canon-engineering-docs -- generate --check` | 0 | — |
| `cargo test -p canon-engineering-docs --locked --no-fail-fast` | 0 | 51 passed |

The 113 "before" is the 116 cases that ran minus my 3; I did not run the suite before writing them.

**4. Finding**

| file:line | Verdict / origin | What was measured | What reaches it |
|---|---|---|---|
| `protocols/incident-response/1.yaml:7-9` (and `docs/examples/incident-response.md:30-32`) | NEEDS-CHANGE / introduced, warning | Both texts promise `investigate_cause` stays open until the service's current revision is analysed. But `cause.identified`'s `true_when` has no `subject`, so a release analysis at `r42` discharges it with the service at `s2`. The rollback does not move `r42`, so the stale-revision protection is bypassed. | `release.inspect` declares `may_produce: cause_analysis`, and a release is the natural subject of what it produces. |

Background: `review-result/adversary-w12-...-pass-1.md:60` records that a release cause analysis moving `cause.identified` was a decision you made. So there are two possible fixes:
- **A:** reword the description and the example to "a cause analysis of a current artifact revision".
- **B:** add `subject: service` to `cause.identified`, which reverses that earlier decision.

**5. Attacked and not broken**
- `emergency.leave` never depends on the cause: it stays admissible in all three variants.
- A stale `s1` analysis after the restore does not discharge the obligation.
- An inconclusive result does not discharge it.
- The fixture harness compares obligations both ways, so a state missing `investigate_cause` cannot pass.
- The vocabulary entry is appended after the first 35 terms, which matches the convention.
- The generated pages, the graph and `status.json` (36 terms, 2 obligations) agree with `generate --check`.

I did not acquire a worktree session lease.

**6. Paths written outside the worktree:** `$HOME/.cache/engineering-protocols-w20261008/investigation-adversary/` with `case-alone.log`, `suite.log`, `suite-nff.log`, `docs.log`.

```findings
- file: protocols/incident-response/1.yaml
  line: 7
  category: contract-drift
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: investigate_cause is discharged by a cause analysis of release r42 while the service is at s2, contradicting the protocol description and the incident example that it stays open until the service's current revision is analysed
```
