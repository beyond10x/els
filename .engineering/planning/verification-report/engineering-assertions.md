---
format: aep.planning-md/3
id: verification-report:engineering-assertions
kind: verification-report
status: draft
title: Typed assertion collection and replay verification
relations:
- verifies: story:engineering-assertions
revision: 1
---
The implementation adds a typed catalog, gate runner, bounded local providers, retained evidence
and offline replay to the existing Rust CLI. Generic language semantics remain in Canon at exact
commit `cf2ae40c2284c86d05b33c3003c45c7351f1150b`. Kubernetes and AEP acceptance integration are
not part of this implementation; Codegate uses its existing dependency report, not invented scores.

## Evidence

The gate specification and failing CLI acceptance test were committed first (`a35b77d`). The
provider contract/red suite was committed separately (`3b9228f`), followed by provider implementation
(`f8af612`, integrated from the provider worktree). Author and committer are b10x-bot[bot].

`task check` passes formatting, clippy, 171 tests across 49 summaries, and 16 generated documentation
files. This includes ESS 0.53.0 validation/compilation/synthesis, generated gate/provider types and
native envelope parity; independent adversarial regression tests; a compiled external Rust provider
which extends a namespace without an engine rebuild; and replay that performs no provider IO.
The public documentation site builds successfully.

Independent review reproduced four runner defects: output-directory symlink replacement, command
executable mutation after admission, TCP admission delayed until collection, and explicit generated
inputs losing nested cache-named directories. Each now has an executable regression. Command failure
before any evidence acquisition was already enforced. The provider unit also corrected malformed
negative DNS answers, and its suite uses local DNS/TCP services and actual Codegate/ESS report shapes.
A subsequent red/green freshness test ensures live evaluation uses collection completion time: a
slow collection cannot pass an observation that expired while it ran.

The full existing suite caught outdated copied-repository fixtures and empty output-tree assumptions.
Fixtures now include the catalog and ESS model; document generation creates the model directory and
keeps the existing drift diagnostic contract. Existing assertions were preserved.

Logs are retained under `$HOME/.cache/b10x-assertions/runner/` (`task-check-final.log`,
`site-final.log`, `clock-red.log`), `runner-review/`, and `providers/`. Generated ESS Rust output is
byte-exact; its generator's final blank line is retained rather than silently rewriting the contract.

## Publication boundary

The exact Canon commit was fetched from its managed local Git tree into Cargo's Git cache. Tests
used `CARGO_NET_OFFLINE=true` with the real Git revision, not a path override. Canon's signed Gates
check passed, but publication refused `branch authority missing or ambiguous`; the read-only remote
ruleset query returned an empty list. The required App-only authority rule must be restored through
the authorized administration path before Canon can publish. Engineering Protocols is therefore
held locally too: a clean remote consumer cannot yet resolve its Canon revision. No alternate push,
release tag, deployment or live documentation publication is claimed. Committed managed trees and
verified archives retain both units for the next operator session.

Final Engineering Protocols source implementation: `bd654c1`.
