---
title: What ELS is
sidebar_position: 1
slug: /
description: ELS states what makes engineering work legitimate, as protocols Canon evaluates.
---

ELS, the Engineering Lifecycle Specification, is the engineering domain built on
[Canon](https://github.com/beyond10x/canon): engineering vocabulary and protocols such as
`software.change/1` and `incident.response/1`.

ELS states what makes engineering work legitimate: which claims must hold, which evidence counts and
for which revision, which actions need authority, and which outcomes a case can end in.

## Protocols are data

Each protocol is a Canon `protocol/1` document at `protocols/<name>/<major>.yaml`. Canon validates
it, compiles it and evaluates it; ELS adds no hidden clock, network or model call. A protocol
declares artifacts, evidence kinds, claims with the predicate that makes each true, obligations,
actions with the authority they require, and the outcomes a case can end in.

What a claim, a piece of evidence or an obligation *is*, and how one is decided, belongs to Canon.
ELS owns the engineering [vocabulary](./vocabulary.md) and the [protocols](./protocols/index.md)
written in it.

ELS does not hold the live engineering record; that is AEP. It does not run agents; that is
Commission and Loom. Every rule is pressure-tested against both software delivery and incident
response: a rule that only makes sense for Git, pull requests or code is not necessarily an ELS core
rule.

## UNKNOWN is not FALSE

A claim is `TRUE`, `FALSE` or `UNKNOWN`. Missing or stale evidence makes a claim `UNKNOWN`; it never
makes it `FALSE`.

### A stale revision

A software change whose tests passed on revision R1 but whose implementation is now R2 has
`tests.pass = UNKNOWN`, and merge stays blocked until evidence for R2 exists.

```text
implementation = R2

Evidence:
  tests pass for R1

Claims:
  tests.pass = UNKNOWN

Actions:
  repository.merge = blocked
```

After `tests.run` on R2, `tests.pass` is `TRUE` and merge needs approval; after authority is
granted, merge is admissible. Connecting a repository host never grants merge by itself.

### Progress without a cause

An incident can leave emergency mode while its cause is still `UNKNOWN`. After a rollback and fresh
health evidence, `service.healthy` and `impact.bounded` are `TRUE` while `cause.identified` stays
`UNKNOWN`: operational restoration and causal investigation progress independently.
