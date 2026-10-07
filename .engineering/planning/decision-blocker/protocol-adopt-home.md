---
format: aep.planning-md/3
id: decision-blocker:protocol-adopt-home
kind: decision-blocker
status: open
title: 'Where protocol.adopt/1 lives: this repository or Canon'
relations:
- blocks: story:protocol-adopt-protocol
revision: 1
---
## Question

Where does `protocol.adopt/1` live: in this repository or in Canon? Atlas ADR 0086 § Open leaves it open ("Where `protocol.adopt/1` lives: ELS or Canon"). The answer fixes which crate a governor depends on to adopt a generated protocol, so it is a contract between two repositories and the operator decides it.

## Options

| option | what it does | what it costs |
|---|---|---|
| A | This repository ships `protocol.adopt/1` as a built-in at `protocols/protocol-adopt/1.yaml`. Its terms stay out of `protocols/vocabulary.yaml`, as `support.triage/1`'s do. | `AGENTS.md` § Boundary gains a second protocol outside engineering, and a non-engineering domain adopts its generated protocols through this crate (Atlas ADR 0068 keeps other domains out of it). `diff_report` and `check_report` describe `canon diff` and `canon check` output, so a change to either report's shape needs a release here as well. It reuses what is here: the build-time registry (`story:protocol-registry`), the fixture harness (`story:fixture-harness`), the generated protocol pages, and the pin Loom's crates already hold on this crate. |
| B | Canon ships `protocol.adopt/1` as its first built-in protocol. | Canon ships no protocol today (canon checkout `main` `761239f` has no `protocols/` directory). It would build what this repository built in `story:protocol-registry` and `story:fixture-harness`: a `protocols/` directory embedded at build time, a registry, a fixture harness and generated pages. A consumer then looks up built-ins in two crates. The protocol and the two report formats it reads release together. `story:protocol-adopt-protocol` here is archived. |

Recommendation: A. Everything the story needs exists here today, and B builds a second registry and harness for one protocol.

## Clears when

The operator's answer is written into this body and the blocker moves to `cleared`. Under A, `story:protocol-adopt-protocol` is scoped as written. Under B it is archived and Canon plans the protocol in its own store.
