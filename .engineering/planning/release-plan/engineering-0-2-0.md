---
format: aep.planning-md/3
id: release-plan:engineering-0-2-0
kind: release-plan
status: approved
title: Release Engineering Protocols 0.2.0
relations:
- delivers: story:engineering-assertions
revision: 3
transitions:
- {from: "draft", to: "approved", at: "2026-10-05T21:35:19Z", actor: "human:timo", revision: 2}
---
## Intent and authorization

The operator requested new releases after both assertion PRs merged. Publish Engineering Protocols
0.2.0 after Canon 0.1.0. The minor version introduces the assertion catalog, collectors and CLI,
and also includes the unreleased support.triage/1 protocol already on main.

## Scope and procedure

Set workspace version and lockfile to 0.2.0, pin the Canon dependencies to its 0.1.0 release,
update CHANGELOG.md and installation/status prose, and regenerate derived documentation.
No protocol or assertion semantics change, so no new ESS/red specification is required.
ESS remains the newest release, verified as 0.53.0.

Run task check and the documentation build; land a green bot PR on main. Verify the exact main
commit's source checks, publish annotated bot tag 0.2.0 and the GitHub Release, then verify tag
peeling, Release identity and downloadable source archives. There is no binary release workflow
or crates.io publication. Docs publication remains asynchronous; no unrelated consumer release,
Atlas or Website changes are included.

The prior feature candidate passed 171 workspace tests and the public site build (PR #9).
The new version and released Canon dependency must pass the candidate's gate and CI. Retain final
CI, tag and Release evidence in the delivery report.

## Release validation correction

The release gate exposed a scheduling race in the malformed-provider conformance fixture:
`printf` can close stdin before the supervisor writes the request. Instrumenting the existing
assertion and repeating its focused test reproduced `Unavailable(Broken pipe)` instead of
reaching response validation. A Rust fixture now consumes the request to EOF before emitting
the same three malformed envelopes; the existing rejection assertions are retained, with
response/result diagnostics. This changes only test setup, with no product or protocol change.
