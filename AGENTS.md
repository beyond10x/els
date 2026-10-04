# AGENTS.md — els

What ELS is and how to build it is in [README.md](README.md); this file is what an agent changing it
must know. The cross-repository architecture is Atlas ADRs 0066–0075 and Atlas
`docs/design/governed-autonomy/`.

## Serves

- **O2 — decisions as data, with evidence.** Engineering method is declared as protocols and
  evaluated by Canon.

## Boundary

- ELS owns engineering-domain vocabulary and protocols (Atlas ADR 0068): `software.change/1`,
  `incident.response/1`, later investigation, ML experiment, migration and security response.
- Generic claim, evidence and obligation semantics belong to Canon. Do not re-implement them here.
- ELS does not own the live engineering record (AEP, Atlas ADR 0069) or agent execution (Commission,
  Loom).
- Pressure-test every abstraction against both software delivery and incident response. A rule that
  only makes sense for Git, pull requests or code is not necessarily an ELS core rule.

## Rules

- Protocol evaluation goes through Canon; ELS adds no hidden clock, network or model call.
- `UNKNOWN` is not `FALSE`.
- Every protocol rule lands with a fixture that exercises it, including stale-revision fixtures.
- Anything that runs is Rust; command lines use clap derive.

## ESS

ELS opts out of ESS for its protocol semantics: protocols are defined in Canon and tested by Canon
conformance (Atlas ADR 0067). ESS conformance reports are an evidence *kind* ELS protocols may admit
(story E-007); that is a use of ESS output, not a specification of ELS.

Protocol first (Atlas ADR 0080): a unit that changes behaviour lands its protocol YAML and fixture
expectations in its first commit, a named test (a Canon fixture test under `crates/els/tests/`)
fails on that commit and the failing run is recorded, and only later commits implement against it.
A change with no behaviour change is exempt and says so in its story's `## Protocol first`.

## Work

- Planned in the AEP store under `.engineering/`, written only through `aep plan artifact`. Body
  drafts go in `.engineering/drafts/` (ignored).
- Build with `CARGO_TARGET_DIR=$HOME/.cache/b10x-target/els` (the Taskfile sets it).
- Every commit and push is `b10x-bot[bot]`'s through `b10x-gates bot`; every GitHub write goes
  through `b10x-gates api`.
- Use a managed worktree (`worktree create --repo els --purpose …`) for changes.
