# Engineering protocols

The engineering domain built on [Canon](https://beyond10x.github.io/canon/)
([GitHub](https://github.com/beyond10x/canon)): engineering vocabulary, protocols such as
`software.change/1` and `incident.response/1`, and typed assertion gates.

[Documentation](https://beyond10x.github.io/engineering-protocols/) ·
[Assertion guide](website/docs/guides/assertions.md)

Engineering protocols state what makes engineering work legitimate: which claims must hold, which
evidence counts and for which revision, which actions need authority, and which outcomes a case can
end in. A software change whose tests passed on revision R1 but whose implementation is now R2 has
`tests.pass = UNKNOWN`, and merge stays blocked until evidence for R2 exists. An incident can leave
emergency mode while its cause is still `UNKNOWN`.

The library is the crate `b10x-canon-engineering` (`canon_engineering`), and the command line is
`canon-engineering`: `canon-engineering protocols list` names the built-in protocols.
`assertions list` discovers the catalog; `gates validate`, `gates run` and `gates evaluate`
check `.engineering/gates.yaml`, collect observations and replay retained evidence.
The generic expression core lives in Canon; this repository supplies the engineering providers.

The project does not hold the live engineering record (that is AEP) and does not run agents (that is
Commission and Loom).

## Status

Source implementation includes local files/documents, explicit process bindings, normalized test
reports, ESS and Codegate dependency reports, DNS and TCP observations, recipes and typed external
Rust providers. Kubernetes providers and AEP acceptance integration remain planned.
The protocol design, written under the project's earlier name, is
[`docs/design/engineering-lifecycle-specification-design.md`](docs/design/engineering-lifecycle-specification-design.md);
worked examples are in [`docs/examples/`](docs/examples/).

## Build

```console
task check
```

## Licence

Apache-2.0.
