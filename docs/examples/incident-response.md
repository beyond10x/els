# Example — Incident Response

```text
Case INC-492

Claims:
  customer_impact_bounded = TRUE
  service_healthy = FALSE
  cause_identified = UNKNOWN

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
service_healthy = TRUE
customer_impact_bounded = TRUE
cause_identified = UNKNOWN
```

The operational incident can leave emergency mode while the investigation remains open.

This demonstrates why a protocol is richer than one scalar workflow state.
