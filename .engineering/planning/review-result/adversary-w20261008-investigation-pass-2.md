---
format: aep.planning-md/3
id: review-result:adversary-w20261008-investigation-pass-2
kind: review-result
status: active
title: Wave 2026-10-08-w1 adversary, story:incident-investigation-obligation, pass 2
relations:
- reviews: story:incident-investigation-obligation
revision: 1
---
unit: story:incident-investigation-obligation
verdict: green
cases: executed 116→121, red 0
origin: introduced 1, pre-existing 0, undecided 0
wrote-outside-worktree: $HOME/.cache/engineering-protocols-w20261008/investigation-adversary/ (pass2-case-alone.log, pass2-suite.log, plus the pass-1 logs)
needs-coordinator: yes

Pass 2 is green: no failing case. The new wording ("a current revision of the service or the release") matches what Canon decides. Two places still describe a service-only investigation: one test comment, and the story's Outcome, which is in your planning store.

**1. Diff stat** (`681ce28..f039013`, committed as the bot, not pushed). Only a test file changed.
```
 .../tests/adversary2_investigation_obligation.rs   | 159 +++++++++++++++++++++
```

**2. Cases added** in `crates/canon-engineering/tests/adversary2_investigation_obligation.rs`. Each gives inc-492's last state a cause analysis that is not of a current service or release revision. All 5 are green; this is what each one observed when run on its own:

| Case | Observed |
|---|---|
| release at stale `r41` | `investigate_cause` stays open |
| subject `deployment` | Canon refuses it: "about artifact `deployment`, which the protocol does not declare" |
| `service` with no `subject_revision` | refused when the fixture loads: "missing field `subject_revision`" |
| no `subject` | refused when the fixture loads: "missing field `subject`" |
| `s2` analysis observed at 12:30, after the fixture's 12:00 instant | refused when the fixture loads: "observed … after the evaluation instant" |

The last case first failed on my own `expect("loads")`. That was my mistake, not a finding; I fixed it before the suite ran.

**3. Gates**

| Command | Exit | Result |
|---|---|---|
| `cargo test -p b10x-canon-engineering --locked --no-fail-fast` | 0 | 121 passed, 0 failed |
| `cargo run --locked -p canon-engineering-docs -- generate --check` | 0 | 16 generated files fresh |
| `cargo clippy -p b10x-canon-engineering --test adversary2_investigation_obligation --locked -- -D warnings` | 0 | — |

The 116 "before" is 121 minus my 5 cases.

**4. Findings**

| file:line | Verdict / origin | Finding |
|---|---|---|
| `crates/canon-engineering/tests/incident_response_protocol.rs:363` | CONFIRMED / introduced, note | The acceptance test's doc comment still says "until a cause analysis of the service's current revision exists". It is a one-line fix by the implementor; I did not edit an existing case. |
| `.engineering/planning/story/incident-investigation-obligation.md:40` | CONFIRMED / introduced, note | The story's Outcome still says "until a cause analysis of the current service revision exists", which option A contradicts. The store is yours to change; this is the needs-coordinator item. |

My pass-1 red case was inverted to assert `discharged`. That follows your option-A decision, so I am not reporting it as a finding.

**5. Attacked and not broken**
- The release analysis now discharges `investigate_cause` and leaves `emergency.leave` alone.
- The protocol page, the obligation table, the protocols index and the graph JSON all carry the new wording, and `generate --check` confirms they are fresh.
- The example page agrees with the protocol.
- The vocabulary's meaning for `investigate_cause` makes no claim about the subject.

**6. Paths written outside the worktree:** `$HOME/.cache/engineering-protocols-w20261008/investigation-adversary/pass2-case-alone.log` and `pass2-suite.log`, plus the four pass-1 logs in the same directory.

```findings
- file: crates/canon-engineering/tests/incident_response_protocol.rs
  line: 363
  category: contract-drift
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: the acceptance test's doc comment still promises investigate_cause stays open until a cause analysis of the service's current revision, while option A discharges it on a current release analysis too
- file: .engineering/planning/story/incident-investigation-obligation.md
  line: 40
  category: contract-drift
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: the story Outcome still says the investigation stays open until a cause analysis of the current service revision exists, contradicting the option-A wording now shipped
```
