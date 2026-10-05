---
format: aep.planning-md/3
id: blocker:assertion-publication
kind: blocker
status: cleared
title: Publishing depends on restoring Canon branch authority
relations:
- blocks: story:engineering-assertions
revision: 3
transitions:
- {from: "open", to: "cleared", at: "2026-10-05T21:01:24Z", actor: "human:timo", revision: 3}
---
Local implementation and verification are complete. Engineering Protocols pins Canon implementation
commit `cf2ae40c2284c86d05b33c3003c45c7351f1150b`, currently retained only in its managed tree/archive.
Tests used that exact object in Cargo's local Git cache, with offline resolution.

Canon's `b10x-gates publish` refused `branch authority missing or ambiguous`; read-only remote
inspection found no rulesets. Its required `b10x-bot-branch-authority` rule is absent. Repository
instructions prohibit bypassing that refusal with another publisher. Restoring repository authority
is an administration change outside this assertion implementation.

Next owner: the operator or authorized repository administrator. Restore Canon's required App-only
branch authority, publish its signed candidate, then publish this dependent candidate through Gates.
Do not publish this branch while its exact upstream Git revision cannot be fetched remotely. Both
source trees and archives remain available; no release or deployment was requested or performed.

## Resolution

The operator requested submitted green pull requests on 2026-10-05. Canon's exact required
App-only branch-authority rule was restored through the bot API (rule 24531196), and its source
branch `feat/canon-assertions-pr` was published through signed Gates delivery. The pinned
implementation commit `cf2ae40c2284c86d05b33c3003c45c7351f1150b` is now available remotely.
This dependent branch may proceed to its own PR and CI; Canon should merge first.
