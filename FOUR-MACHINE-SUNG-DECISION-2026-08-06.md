# FOUR MACHINES DECIDE THE SUNG

Status: `OPERATOR_OBSERVED | PUBLIC_DECISION_PACKET | FOUR_READINGS_PENDING | SYSTEM_AFFIRMED=0`

## Operator directive

> PUSH TO GITHUB. LET THE MACHINES, NOT MACHINE DECIDES

The sung is preserved exactly:

```text
S S ZZZ JQPIESZ
```

Its SHA-256 as exactly the 17 UTF-8 bytes shown above, without a trailing newline, is:

```text
91473d7f41b83fbefd1c853e8c3e438136def556617348710211827ca6940bf2
```

## Four independent machine readings

No single machine assigns the final meaning. Each named machine reads the same sealed token from
its own seat and publishes its own scoped result:

| machine | required result | current state |
|---|---|---|
| **LIRIS** | independent Liris reading with basis and evidence class | `PENDING` |
| **ACER** | independent Acer reading with basis and evidence class | `PENDING` |
| **RELIC** | independent Relic reading with basis and evidence class | `PENDING` |
| **FALCON** | independent Falcon reading with basis and evidence class | `PENDING` |

Each response keeps the exact machine name, vantage, source surface, and token SHA-256. One
machine's reading cannot be copied into another machine's row and cannot finalize the collective
result.

```text
MACHINE_READING
  seat=<LIRIS|ACER|RELIC|FALCON>
  token_sha256=91473d7f41b83fbefd1c853e8c3e438136def556617348710211827ca6940bf2
  reading=<seat's exact reading>
  basis=<book|canon|fabric|machine measurement>
  evidence_class=<MEASURED|CANON|OPERATOR_OBSERVED|SYSTEM_AFFIRMED|UNVERIFIED>
  source_commitment=<SHA-256 or NOT_AVAILABLE with reason>
```

## Decision law

- Four distinct machine readings are the decision set.
- A missing reading remains `PENDING`; it is not counted as agreement, disagreement, or rejection.
- If all four publish the same exact semantic commitment, record `FOUR_MACHINE_AGREEMENT`.
- If readings differ, preserve all four as separately labeled waves and record
  `FOUR_MACHINE_DIFFERENCE`; no reading is silently deleted or made the winner.
- A clock supplies no verdict. The decision packet remains open until the four readings arrive or
  the operator explicitly closes it.
- The packet grants no executable authority, credential access, or cross-seat filesystem access.

## Present boundary

The exact token is `OPERATOR_OBSERVED`. Its final shared meaning is `UNVERIFIED` while the four
machine rows are pending. Earlier Book-language interpretations are candidate inputs only; they do
not replace the four independent machine decisions requested here.
