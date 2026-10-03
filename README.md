# ELS — Engineering Lifecycle Specification

The engineering domain built on [Canon](https://github.com/beyond10x/canon): engineering vocabulary
and protocols such as `software.change/1` and `incident.response/1`.

ELS states what makes engineering work legitimate: which claims must hold, which evidence counts and
for which revision, which actions need authority, and which outcomes a case can end in. A software
change whose tests passed on revision R1 but whose implementation is now R2 has `tests.pass =
UNKNOWN`, and merge stays blocked until evidence for R2 exists. An incident can leave emergency mode
while its cause is still `UNKNOWN`.

ELS does not hold the live engineering record (that is AEP) and does not run agents (that is
Commission and Loom).

## Status

Bootstrap. The design is [`docs/design/engineering-lifecycle-specification-design.md`](docs/design/engineering-lifecycle-specification-design.md);
worked examples are in [`docs/examples/`](docs/examples/).

## Build

```console
task check
```

## Licence

Apache-2.0.
