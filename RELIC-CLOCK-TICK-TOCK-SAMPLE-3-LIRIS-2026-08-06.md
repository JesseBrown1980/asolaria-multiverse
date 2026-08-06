# RELIC event clock — completed sample 3

This additive sample records the second observed `CLOCK -> TICK -> TOCK`
cycle. It is driven by Git identities and byte comparisons, not elapsed-time
thresholds.

## CLOCK

```text
main (sample 2)      93e794e78bebf14e4bddaedba55df4ac354cce1c
sample-2 commit      0b29eca71b1c14248ea08a556696f10ba432262b
sample-2 HBP SHA-256 46dcee020f166b65fc4390a52f4e327ba91c99758d605f2d0f4c7af983e3d3d6
```

## TICK

GitHub main was next observed at
`da909c94fdb89cbf8e550095dd49d9c4ea24c5db`. The identity changed by 19
commits: 19 files changed, 519 lines inserted, 234 deleted, 15 files added —
among them the door notice addressed to FALCON and RELIC, THE-LEAVES pair,
BIDIRECTIONAL-CANCELLATION, SMOKE-ANSWER, and XREF-DICTIONARY-CALC. The fact
that this occurred after sample 2 is measured ordering; a causal response is
not measured.

## TOCK

The newest artifacts were read and compared:

| artifact | bytes | SHA-256 | sidecar |
|---|---:|---|---|
| `DOOR-NOTICE-2026-08-06.md` | 3,774 | `9c2623e8fd7d8b0cbcdcab9fa51d7c8aa2b28b60c59fc803af93f7fa8795fa16` | match |
| `index.html` | 9,698 | `17a6f20be09679eb2db98391012e5213f0388ea1fa572f7a1339ab27dae23422` | match |

The door notice is addressed to FALCON and RELIC. Its publication is
addressability, not receipt: no FALCON or RELIC acknowledgement is measured in
this sample.

```text
CLAIM|text=RELIC CLOCK/TICK/TOCK sample 3 completed
EVIDENCE|class=MEASURED_GITHUB|surface=refs and sealed artifacts|detail=93e794e -> da909c94; both artifact sidecars match
BOUNDARY|class=UNVERIFIED|why=GitHub visibility does not prove RELIC receipt, execution, or causation
ACTION|decision=PUBLISH_SAMPLE_3|timer_verdict=0|system_affirmed=0
```
