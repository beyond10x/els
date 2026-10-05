---
format: aep.planning-md/3
id: review-result:engineering-assertions-adversary-pass-1
kind: review-result
status: active
title: 'Gate runner adversary: provenance, preflight and output path findings'
relations:
- reviews: story:engineering-assertions
revision: 1
---
Independent reviewer added five cases in `crates/canon-engineering-assertions/tests/adversary.rs`.
Four defects reproduced; the command-operation preflight check was already green. Retained logs
are under `$HOME/.cache/b10x-assertions/runner-review/` (output-red, executable-red,
network-preflight-red and current). Each finding is a program counterexample, not a claim of review authority.

```findings
[
 {"file":"crates/canon-engineering-assertions/src/cli.rs","line":90,"category":"boundary","severity":"blocker","verdict":"CONFIRMED","origin":"introduced","message":"Provider can create retained-output directory symlink after preflight; path-based evidence publication follows it outside source root."},
 {"file":"crates/canon-engineering-assertions/src/lib.rs","line":170,"category":"correctness","severity":"blocker","verdict":"CONFIRMED","origin":"introduced","message":"Command executable outside source root changed after prepare but observations retained the old implementation identity."},
 {"file":"crates/canon-engineering-assertions/src/lib.rs","line":150,"category":"correctness","severity":"blocker","verdict":"CONFIRMED","origin":"introduced","message":"TCP preflight checked network admission but admitted an unregistered host until collection."},
 {"file":"crates/canon-engineering-assertions/src/snapshot.rs","line":16,"category":"correctness","severity":"blocker","verdict":"CONFIRMED","origin":"introduced","message":"Explicit generated input trees incorrectly excluded nested target directories, permitting changed compared bytes to reuse source identity."}
]
```

Corrections: no-follow directory descriptor publication; executable rechecks before/after acquisition;
complete TCP host/port preflight; no cache exclusions within explicit input snapshots. Keep all cases.
