---
format: aep.planning-md/3
id: story:agents-serves-section
kind: story
status: active
title: AGENTS.md names the Atlas objectives the repository serves
summary: 'A ## Serves section naming O1 and O2 grounds the repository in the Atlas map.'
relations:
- serves: vision:O2
scope:
- confidence: cited
  path: AGENTS.md
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-10-08T18:53:47Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-08T18:53:47Z", actor: "human:timo", revision: 4}
---
## Outcome

`AGENTS.md` carries a `## Serves` section naming the Atlas objectives this repository serves, by
id, in the form the other workspace repositories use (`- **O<n> — <objective>.** <how>`), so the
Atlas map check can ground the repository.

- **O2 — decisions as data, with evidence.** Engineering rules are declared as Canon protocols and
  evaluated against cited evidence, with `UNKNOWN` kept apart from `FALSE`.
- **O1 — governed reach.** Every write action in a protocol names the capability that authorises it,
  and Loom's governor admits actions from these protocols.

No behaviour change: the story touches documentation only.

## Scope

- `AGENTS.md` (new section between the introduction and `## Boundary`)

`CHANGELOG.md` is not edited by this unit; the release commit records the change.

## Shared surface

None.

## Protocol first

Exempt: no protocol, fixture or behaviour changes.

## Acceptance

`grep -c '^## Serves' AGENTS.md` prints `1`, and every line of the section names an objective id
that exists in Atlas `ROADMAP.md` § Objectives (O1 to O6). `task check` stays green.

## Source

Atlas `ROADMAP.md` § Objectives; the Atlas map check reports this repository red for lacking the
section.
