---
format: aep.planning-md/3
id: review-result:els-first-domain-parallel-safety-r1
kind: review-result
status: active
title: els-first-domain decomposition — parallel-safety critic, round 1
relations:
- reviews: epic:els-first-domain
- reviews: story:els-vocabulary
- reviews: story:ess-conformance-evidence
- reviews: story:incident-response-protocol
- reviews: story:ml-protocol-shape-decision
- reviews: story:security-independence-rules
- reviews: story:software-change-negative-outcomes
- reviews: story:software-change-profiles
- reviews: story:software-change-protocol
- reviews: story:stale-evidence-fixtures
revision: 1
---
needs-revision

story:software-change-profiles — sits in the same wave as story:software-change-negative-outcomes and story:ess-conformance-evidence, and all three edit the `software.change/1` protocol source and its fixture file; no body names the others or the shared file (that file does not exist yet, story:software-change-protocol creates it). Surface is inferred; remedy is an ordering edge naming the file or splitting the source into per-concern files, either one — .engineering/planning/story/software-change-profiles.md:13-18
story:ess-conformance-evidence — adds `system_conformance` evidence and `implementation.conforms` to the same `software.change/1` source as story:software-change-profiles and story:software-change-negative-outcomes, with `depends_on` only on story:software-change-protocol and no mention of those two (inferred; file not yet created) — .engineering/planning/story/ess-conformance-evidence.md:13-18
story:security-independence-rules — edits both protocol sources, so it shares the `software.change/1` source with profiles, negative-outcomes and ess-conformance-evidence, and the `incident.response/1` source with story:stale-evidence-fixtures; neither collision is stated and both are only inferred; `depends_on` covers the protocols but not these siblings — .engineering/planning/story/security-independence-rules.md:16-19
story:stale-evidence-fixtures — adds fixtures to both protocols' fixture files, which the other wave-2 stories also edit (security-independence-rules, profiles, negative-outcomes, ess-conformance-evidence), and no body admits it (inferred) — .engineering/planning/story/stale-evidence-fixtures.md:15-20
story:incident-response-protocol — runs at the same time as story:software-change-protocol (both depend only on story:els-vocabulary), and both land in `crates/els/src/lib.rs` (cited: the existing `software_change_protocol()` and `incident_response_protocol()` at lines 8-14, plus module wiring) and each calls for a "Rust test under `task check`", so the compile-and-evaluate fixture harness is created by both and owned by neither (inferred; harness file not yet created) — .engineering/planning/story/incident-response-protocol.md:35
story:software-change-protocol — names no file for its protocol source or the shared fixture harness, and `aep plan artifact waves --kind story` prints "unassessed" for all 8 code stories; the later stories can only extend a file whose path they are never given, so this body must record the path (`aep artifact scope <id> --add <path>`) — `aep plan artifact waves --kind story` ("0 wave(s), 0 collision(s), 9 unassessed"); .engineering/planning/story/software-change-protocol.md:29
story:els-vocabulary — the vocabulary is extended by later stories that introduce terms its acceptance does not list (`declined`, `rolled_back`, `superseded`, the four profiles, `system_conformance`, independence dimensions), yet the body names no file and says nothing about who adds those terms, so the later stories also collide on it (inferred) — .engineering/planning/story/els-vocabulary.md:20-22

**What I read:** 12 artifacts (epic, 9 stories, 2 visions) via `aep plan artifact list`, `show`-equivalent reads of every story body, `aep plan artifact graph`, `aep plan artifact waves --kind story`, plus `crates/els/src/lib.rs`, `crates/els/Cargo.toml` and `AGENTS.md`. Surfaces: 1 cited (story:ml-protocol-shape-decision, whose body says no Rust, so its output is an ADR in the store); 8 inferred (crate-level `crates/els`, no file named); 0 unplaceable at crate level. At file level the 8 are unplaced, which is why `waves` reports 0 collisions and 9 unassessed. The waves from the `depends_on` graph are: els-vocabulary; then software-change-protocol and incident-response-protocol; then profiles, negative-outcomes, ess-conformance-evidence, security-independence-rules and stale-evidence-fixtures (the last two after incident).

**What I could not establish:**
- Whether the protocol source is one file or several, and the path of the fixtures, because no body says. Every collision here depends on that and is inferred.
- Out of my lane: whether the Canon items C-001..C-011 each story claims exist in Canon, and whether the five-way fan-out is the right split (scope and design critics).

```findings
- file: .engineering/planning/story/software-change-profiles.md
  line: 13
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: sits in the same wave as story:software-change-negative-outcomes and story:ess-conformance-evidence and all three edit the software.change/1 protocol source and fixture file (not yet created) with no body naming the others or the shared file; surface inferred; remedy is an ordering edge naming the file or splitting the source, either one
- file: .engineering/planning/story/ess-conformance-evidence.md
  line: 13
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: adds system_conformance evidence and implementation.conforms to the same software.change/1 source as story:software-change-profiles and story:software-change-negative-outcomes and depends_on only story:software-change-protocol; surface inferred, file not yet created
- file: .engineering/planning/story/security-independence-rules.md
  line: 16
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: edits both protocol sources, so shares the software.change/1 source with profiles, negative-outcomes and ess-conformance-evidence and the incident.response/1 source with story:stale-evidence-fixtures, with neither collision stated; surface inferred
- file: .engineering/planning/story/stale-evidence-fixtures.md
  line: 15
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: adds fixtures to both protocols' fixture files that the other wave-2 stories also edit (security-independence-rules, profiles, negative-outcomes, ess-conformance-evidence) and no body admits it; surface inferred
- file: .engineering/planning/story/incident-response-protocol.md
  line: 35
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: runs concurrently with story:software-change-protocol and both land in crates/els/src/lib.rs (existing protocol functions at lines 8-14, plus module wiring) and each calls for a Rust fixture test under task check, so the shared fixture harness is created by both and owned by neither; the harness file does not exist yet; harness surface inferred
- file: .engineering/planning/story/software-change-protocol.md
  line: 29
  category: parallel-safety
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: names no file for the protocol source or the shared fixture harness, and waves reports all 8 code stories unassessed with 0 collisions, so no concurrent set can be certified; record the path with aep artifact scope
- file: .engineering/planning/story/els-vocabulary.md
  line: 20
  category: parallel-safety
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: later stories add terms its acceptance does not list (declined, rolled_back, superseded, the four profiles, system_conformance, independence dimensions) while the body names no file and does not say who adds them, so those stories also collide on it; surface inferred
```
