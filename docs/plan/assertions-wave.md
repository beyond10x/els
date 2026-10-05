# Assertion implementation

Approved by the operator's “Implement the plan” instruction on 2026-10-05.
Objective: O2, decisions as data with evidence. AEP artifact integration and Kubernetes
collection are later work. No source release is requested.

The coordinated units are Canon's generic expression language and this repository's
engineering catalog, collection, replay and CLI. Implementation uses the
`aep:implementing` 0.19.2 role references through Codex agents; dedicated plugin agent
types are not exposed by this host. Review uses the adversary role.

| Unit | Branch | Managed tree | Target | Scratch | Stage |
| --- | --- | --- | --- | --- | --- |
| Canon expressions | feat/canon-assertions | `$HOME/.local/state/worktree/trees/b10x/canon/canon-assertions` | `$HOME/.cache/b10x-target/canon-assertions` | `$HOME/.cache/b10x-assertions/core` | verified; publication blocked |
| Engineering gate CLI | feat/engineering-assertions | `$HOME/.local/state/worktree/trees/b10x/engineering-protocols/engineering-assertions` | `$HOME/.cache/b10x-target/engineering-assertions` | `$HOME/.cache/b10x-assertions/runner` | verified; upstream publication blocked |
| Provider implementation | feat/assertion-providers | archived; tree removed through exact-id GC | `$HOME/.cache/b10x-target/assertion-providers` | `$HOME/.cache/b10x-assertions/providers` | integrated and verified |

Provider work is a bounded implementation part of the engineering story. The coordinator
owns the store and integrates its commit. Scope avoids all existing protocol YAML and the
unrelated live control-plane-protocols worktree. Initial bases: Canon `66c8d4b`, engineering
protocols `bd4ec7a`. There were 29 GiB free; builds disable debug information and use two
jobs. Primary checkouts remain clean. Approval covers specification/test commits,
implementation commits, integration, review corrections and planning evidence commits,
and bot source-branch publication. No release tag or deployment.

Required evidence: red test before implementation, full workspace formatting/lint/tests,
ESS validation/compilation/synthesis, generated documentation drift, and independent
adversarial tests. External providers never execute during validate or offline evaluate.

## Verified implementation and handoff

Canon's expression implementation is `cf2ae40c2284c86d05b33c3003c45c7351f1150b`; its final
planning/blocker head is `3324faa`. The core has 21 dedicated cases (including four independent
adversarial regressions); the full workspace passes 598 tests, with ESS-first checks additionally
repeating 42. Its public site builds and 17 generated documentation files are current.

Engineering Protocols implementation is `bd654c1`, following provider integration `f8af612`.
Its full `task check` passes 171 tests across 49 summaries, including ESS compilation/synthesis,
contract parity, DNS/TCP, the external Rust extension and offline replay. All 16 generated files
are current and the public site builds. Independent review reproduced four runner defects; their
regressions now pass. A separately reproduced expired-during-collection case also passes.
The detailed record is `verification-report:engineering-assertions`; both review rounds and their
outcomes are in the AEP store. No existing protocol YAML was changed.

Publication is blocked, not claimed complete. Canon's signed scan passed, but Gates refused
publication with `branch authority missing or ambiguous`. Its remote rulesets list is empty.
Engineering Protocols tests resolved the exact dependency from the local Canon Git object with
Cargo offline; a clean remote consumer cannot fetch it yet. No alternate publisher was used.
The operator or authorized repository administrator must restore Canon's required App-only branch
authority, then publish Canon and this dependent branch through Gates. See
`blocker:assertion-publication`. No release tag or live documentation publication was performed.

Both source trees are retained for review with verified archives under
`$HOME/.local/state/worktree/archives/{canon/canon-assertions,engineering-protocols/engineering-assertions}`.
The provider-only tree was archived and removed through exact-id GC after its commits were
integrated; its archive remains under `engineering-protocols/assertion-providers`.

## Pull request submission resumed

On 2026-10-05 the operator requested submitted green pull requests. Canon's missing exact App-only
branch-authority rule was restored through the bot API (rule 24531196). Its source branch was
published through signed Gates delivery, making this repository's pinned Canon commit remotely
available. The publication blocker is cleared. Current main is integrated for PR checks.
Submission continues in managed tree `engineering-assertions-pr`, branch
`feat/engineering-assertions-pr`; earlier archives remain recovery snapshots. Canon should merge
before this dependent PR. No release or deployment is part of PR submission.
