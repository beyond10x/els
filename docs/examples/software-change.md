# Example — Software Change

Initial:

```text
Case CHG-1842
implementation = R2

Evidence:
  tests pass for R1

Claims:
  tests.pass = UNKNOWN

Actions:
  repository.inspect = admissible
  repository.edit = admissible
  tests.run = admissible
  repository.merge = blocked
```

After `tests.run` on R2:

```text
Evidence:
  tests pass for R2

Claims:
  tests.pass = TRUE

Actions:
  repository.merge = approval_required
```

After authority:

```text
repository.merge = admissible
```

The model never gets merge simply because GitHub is connected.
