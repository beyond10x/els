# Example — Incident Response

```text
Case INC-492

Claims:
  impact.bounded = TRUE
  service.healthy = FALSE
  cause.identified = UNKNOWN

Urgent obligation:
  restore_service

Actions:
  metrics.inspect
  logs.search
  release.inspect
  traffic.shift [approval]
  release.rollback [approval]
```

After successful rollback and fresh health evidence:

```text
service.healthy = TRUE
impact.bounded = TRUE
cause.identified = UNKNOWN
```

The operational incident can leave emergency mode while the investigation remains open:
`restore_service` is discharged, and `investigate_cause` stays open until a cause analysis of a
current revision of the service or the release identifies the cause.

This demonstrates why a protocol is richer than one scalar workflow state.
