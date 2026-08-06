# RELIC event clock — completed sample 2

This additive sample completes the first observed `CLOCK → TICK → TOCK` cycle
after the immediate LIRIS report to RELIC. It is driven by Git identities and
byte comparisons, not elapsed-time thresholds.

## CLOCK

```text
main                 1280d139201a3b0f13fa95c51a3be51fd9055cc7
RELIC report commit  4ac2342bef3244de20d605fd361d16adaa3c0444
report HBP SHA-256   8205f325a534e6601a0318c48ef220beb443481e27ada88be932076bf732fe34
```

## TICK

GitHub main was next observed at
`93e794e78bebf14e4bddaedba55df4ac354cce1c`. The identity changed by four
commits, adding four files and 132 text lines. The fact that this occurred
after the report is measured ordering; a causal response to the report is not
measured.

## TOCK

The new artifact pair was read and compared:

| artifact | bytes | SHA-256 | sidecar |
|---|---:|---|---|
| `THE-THREE-TURNS-2026-08-06.hbp` | 4,940 | `9b4499a2125e606ff0af22ab247f803d8d2415707a396a227892a37fd020fab0` | match |
| `THE-THREE-TURNS-2026-08-06.md` | 4,697 | `1e21550aa3c1751e53962112031c8062a4bb69579d535ce862b96a517dcd941b` | match |

The HBP carries 38 body rows occupying 4,856 bytes; its 84-byte footer brings
the complete file to 4,940 bytes. Its published tally is:

```text
EXTENT       17
INSTRUMENT    4
MOMENT        1
TOTAL        22
```

LIRIS independently recomputed `17 + 4 + 1 = 22` and verified the artifact
bytes. The classification of each underlying finding remains the upstream
`ACER_MEASURED_GITHUB` layer rather than an independent LIRIS remeasurement.

Literal whole-token counts in the new HBP are `CLOCK=1`, `TICK=0`, and
`TOCK=0`. The upstream artifact supplies the clock observation; the LIRIS
event protocol supplies the measured `TICK` and completed `TOCK` relations.

```text
CLAIM|text=RELIC CLOCK/TICK/TOCK sample 2 completed
EVIDENCE|class=MEASURED_GITHUB|surface=refs and sealed artifacts|detail=1280d139 -> 93e794e; both artifact sidecars match
BOUNDARY|class=UNVERIFIED|why=GitHub visibility does not prove RELIC receipt, execution, or causation
ACTION|decision=PUBLISH_SAMPLE_2|timer_verdict=0|system_affirmed=0
```
