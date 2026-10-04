---
format: aep.planning-md/3
id: architecture-decision-record:ml-protocol-shape
kind: architecture-decision-record
status: accepted
title: ML research is its own protocol, ml.experiment/1
relations:
- decides: story:ml-protocol-shape-decision
revision: 2
transitions:
- {from: "proposed", to: "accepted", at: "2026-10-04T02:22:39Z", actor: "human:timo", revision: 2}
---
## Decision

ML research is its own protocol, `ml.experiment/1`, not a profile of another protocol. It will
live at `protocols/ml-experiment/1.yaml` in this repository, with header `id: ml.experiment`,
`revision: 1` (canon `crates/canon/src/model/mod.rs:52-57`), and its fixtures under
`fixtures/ml-experiment/`. Its terms go in `protocols/vocabulary.yaml`.

This record builds nothing. The protocol gets its own story with its own § Protocol first.

Sources are read at canon `46dc424`, els `feab40c` and atlas `cd9e5ee8`. Paths prefixed `canon:`
or `atlas:` are in those repositories; unprefixed paths are in this one.

## Why its own protocol

1. ELS creates a distinct protocol only when "its semantic graph or outcomes differ materially"
   (`docs/design/engineering-lifecycle-specification-design.md:1714`). They do. ML research ends
   in `supported`, `refuted`, `inconclusive`, `invalid_experiment` or `superseded`
   (canon `docs/design/canon-protocol-calculus-design.md:961-965`). A software change ends in
   `accepted`, `declined`, `superseded`, `inconclusive`, `rolled_back` or `abandoned`
   (`docs/design/engineering-lifecycle-specification-design.md:586-591`). And a research case may
   end without any deployed change (canon design `:170`, `:930`). So it is not a profile of
   `software.change/1`.
2. `engineering.investigation/1`, the obvious other host, does not exist. No file sits under
   `protocols/`, and `aep plan artifact list` in this store shows no investigation story. Atlas
   lists it only as "likely later" (`atlas:architecture/adr/0068-els-is-an-engineering-domain-layer.md:19-23`).
   A profile of it would have to wait for that protocol to be specified first.
3. A profile cannot be expressed in Canon today. In ELS a profile is the value of a case input in
   the host document, not a file of its own (`.engineering/planning/story/software-change-profiles.md:40`).
   `protocol/1` has no case-input section and no import section: the model declares only artifacts,
   evidence kinds, claims, obligations, actions and outcomes, with `deny_unknown_fields`
   (canon `crates/canon/src/model/mod.rs:28-47`). An outcome's `requires` is a predicate over
   evidence and claims only (canon `crates/canon/src/model/mod.rs:124-130`,
   `crates/canon/src/model/predicate.rs:57-65`). So even with case inputs, a profile could not
   switch on ML-specific outcomes. The investigation document would have to carry
   `refuted` and `invalid_experiment` for every case.
4. Canon treats hypothesis testing as its own shape. Canon's genericity bar lists "investigation"
   and "hypothesis test" as separate cases (canon design `:1695`, `:1697`) and plans separate
   example directories for them (`:1309`, `:1311`). Phase 1 validates the kernel against
   `software.change/1`, `incident.response/1` and `ml.experiment/1` (`:1719-1727`). Atlas names the
   same id (`atlas:architecture/adr/0068-els-is-an-engineering-domain-layer.md:22`).
5. It is data, as Atlas requires: each ELS protocol is a `protocol/1` YAML document at
   `protocols/<name>/<major>.yaml` (`atlas:architecture/adr/0077-canon-replaces-aep-protocol-engine.md:44-51`;
   that file is marked `Status: accepted` at `:3`, though the story calls it a draft).

Rejected:

| option | reason |
|---|---|
| profile of `engineering.investigation/1` | host protocol does not exist (reason 2); profiles need case inputs and cannot select outcomes (reason 3) |
| profile of `software.change/1` | outcomes and subject differ (reason 1) |
| `ml.experiment/1` importing `reproducibility/1` (canon design `:1680`) | `protocol/1` has no imports (canon `model/mod.rs:28-47`, `story:protocol-source-model` lists imports as out of scope at `.engineering/planning/story/protocol-source-model.md:71`); revisit when Canon gains them |

## Capability key

C-001 source model, C-002 `canon-ir/1`, C-003 three-valued claims, C-004 evidence revision
binding, C-005 obligations, C-006 action admissibility, C-007 outcomes (C-007d: the decision half,
`story:decision-outcomes`), C-008 freshness and invalidation, C-009 explanation, C-010 conformance,
C-011 semantic diff. These are the `taskboard:` refs on the canon stories (`aep plan artifact list`
in canon). C-001 and C-002 are implemented. The rest are `proposed`.

## Artifacts

Each is declared as a `protocol/1` artifact, which has only a description
(canon `crates/canon/src/model/mod.rs:59-65`). What a dataset or checkpoint is belongs to ELS
vocabulary, not Canon (canon design `:968-970`). Evidence binds to one subject artifact revision
(canon `.engineering/planning/story/three-valued-claims.md:56-60`,
`.engineering/planning/story/evidence-revision-binding.md:30-34`). Other artifacts an evidence
record depends on are handled by invalidation rules: when an upstream revision moves, the evidence
stops supporting the named claims (canon `.engineering/planning/story/invalidation-rules.md:41-46`).

| artifact | role | Canon |
|---|---|---|
| `research_question` | what the case asks | C-001 |
| `hypothesis` | the falsifiable statement under test | C-001, C-004 |
| `experiment_design` | metrics, splits, seeds, success criterion | C-001, C-004; upstream of `experiment.valid` (C-008) |
| `dataset_snapshot` | the frozen data the runs read | C-001; upstream of every result claim (C-008) |
| `baseline` | the reference model or method | C-001; upstream of `candidate.beats_baseline` (C-008) |
| `candidate` | the implementation under test | C-001, C-004 (subject of result evidence) |
| `run_configuration` | hyperparameters, environment, code revision | C-001; upstream of result claims (C-008) |
| `results` | metrics produced by runs | C-001, C-004 |
| `analysis` | interpretation of the results | C-001, C-004 |
| `conclusion` | the written finding the outcome records | C-001 |

## Claims

Every claim is a `true_when` predicate (canon `crates/canon/src/model/mod.rs:76-82`) evaluated to
TRUE, FALSE or UNKNOWN (C-003). The predicate language matches an evidence kind and result, or
tests another claim's value (canon `crates/canon/src/model/predicate.rs:67-81`). It has no
arithmetic, by design (canon design `:1650-1654`). So "beats" or "robust" is a result reported by
the evaluating producer (for example `comparison` with result `candidate_better`), not a number
Canon compares.

| claim | true when | Canon | gap |
|---|---|---|---|
| `experiment.valid` | design review passed, and no integrity violation in the runs | C-003, C-004, C-008 (design or dataset revision change) | none |
| `result.reproducible` | a reproduction run on the same snapshot and configuration matched | C-003, C-004, C-008 | **independence**: the reproducer should be someone other than the original runner. No item C-001…C-011 provides an independence form (canon design `:1665-1669`; `.engineering/planning/story/security-independence-rules.md:74`). **Composition**: belongs in an imported `reproducibility/1` (canon design `:1680`) |
| `candidate.beats_baseline` | comparison evidence on the current candidate reports `candidate_better` | C-003, C-004, C-008 (baseline, dataset, configuration upstream) | none |
| `effect.robust` | seed or ablation sweep reports `stable` | C-003, C-004 | none |
| `hypothesis.supported` | `all` of the four claims above | C-003 (claim tests) | none |
| `latency.acceptable` | not in /1 | — | **case input**: whether a latency budget applies differs per case, and `protocol/1` has no case inputs (`.engineering/planning/story/software-change-profiles.md:72`) |

`refuted` depends on the distinction between FALSE and UNKNOWN. Without it, a missing comparison
would read as a refutation. That is C-003's rule: UNKNOWN is not FALSE, and only TRUE satisfies a
positive requirement (canon `.engineering/planning/story/three-valued-claims.md:38-49`).

## Terminal outcomes

An outcome is legitimate only when its `requires` predicate is TRUE. A case may end only through a
declared outcome (canon `.engineering/planning/story/outcomes.md:30-34`).

| outcome | requires | Canon | gap |
|---|---|---|---|
| `supported` | `claim: hypothesis.supported` | C-007, C-003 | none |
| `refuted` | `experiment.valid` TRUE, `result.reproducible` TRUE, `candidate.beats_baseline` `is: false` | C-007, C-003 | none |
| `invalid_experiment` | `experiment.valid` `is: false` | C-007, C-003 | none |
| `inconclusive` | `decision: explicitly_inconclusive` (as in canon design `:660-662`) | C-007d (canon `.engineering/planning/story/decision-outcomes.md:44-49`) | none once C-007d lands; plain C-007 excludes it (canon `.engineering/planning/story/outcomes.md:58-60`) |
| `superseded` | `decision: superseded` | C-007d | **cross-case reference**: a `canon-decisions/1` record names the outcome, the principal and the case revision (canon `decision-outcomes.md:47-48`), not the case that supersedes this one |

`supported`, `refuted` and `invalid_experiment` cannot be legitimate together: each requires a
different value for `experiment.valid` or `candidate.beats_baseline`.

## Other constructs the protocol story will need

- Obligations `preregister.design` and `freeze.dataset_snapshot`, each with a `discharged_when`
  predicate: C-005 (canon `.engineering/planning/story/obligations.md:30-41`). Today `Obligation`
  carries only a description (canon `crates/canon/src/model/mod.rs:84-90`). Obligations that apply
  only to some cases, such as preregistration for confirmatory but not exploratory research, need
  **obligation derivation from case inputs**. That is a gap, the same one as
  `.engineering/planning/story/software-change-profiles.md:72`, and it is out of scope for /1.
- Actions `run_experiment`, `reproduce_run`, `compare_to_baseline` and `record_conclusion` with
  `may_produce`: C-006.
- A blocked outcome must name the claims that block it: C-009. Fixtures: C-010. Changes after /1:
  C-011. Compilation: C-002.

## Gaps, in the order they bind

| gap | blocks | Canon source |
|---|---|---|
| independence form | full meaning of `result.reproducible`. /1 can ship without it and gain it in place, as `story:security-independence-rules` does for `software.change/1` | canon design § 39.4 `:1665-1669` |
| cross-case reference | naming the successor in `superseded` | none planned |
| case inputs and obligation derivation | `latency.acceptable`, conditional obligations | ELS design § 9–10 (`:596`, `:886`); `software-change-profiles.md:72` |
| composition | moving reproducibility into `reproducibility/1` | canon design § 39.5 `:1671-1683` |

None of these gaps stops a first `protocols/ml-experiment/1.yaml` from validating under C-001 today.
Evaluating it needs C-003, C-004, C-007, C-007d and C-008.
