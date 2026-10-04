---
format: aep.planning-md/3
id: story:release-provenance-binding
kind: story
status: draft
title: Accepted requires a release built from the verified implementation
relations:
- decomposes: epic:els-first-domain
- depends_on: story:software-change-protocol
- serves: vision:O2
- serves: vision:governed-autonomy
revision: 1
---
## Outcome

`accepted` in `software.change/1` cannot be legitimate unless the release it rests on was built from
the verified implementation revision. Today `release.proven` binds `build_provenance` only to the
release revision, so a release that predates the change (every case snapshot carries one, because
Canon requires a revision for every declared artifact) proves nothing about the change, and
`accepted` holds with the merge denied.

## Found by

Adversary pass 2 of story:software-change-protocol (wave 2026-10-04-w9), finding A1. The design's
§ 9 has the same `release.proven` and closes the gap with transitions Canon does not have.

## Options

- Canon multi-revision binding: a record bound to several artifacts' revisions (design § 9
  `binds:`), so build provenance names the implementation revision it was built from.
- A merge or release evidence kind produced by `repository.merge` / a release action, about the
  implementation, required by `release.proven`.

## Acceptance

The removed case `accepted_is_blocked_while_the_merge_is_denied_and_r2_never_released` (kept at
`ga-wave-2026-10-04-w9/els-software-change-protocol/scratch/a1-case.rs`): CHG-1842's case with tests
passing on R2, the merge denied, build provenance for the pre-existing release v0, d0 healthy and
the objective satisfied gives `accepted` blocked.

## Notes

- The ELS gate does not run `canon check`; on Canon 8d1599e it reports 7 `unproduced-evidence`
  findings for kinds that come from outside the protocol's actions. Decide here whether the gate
  runs it with those findings expected.
