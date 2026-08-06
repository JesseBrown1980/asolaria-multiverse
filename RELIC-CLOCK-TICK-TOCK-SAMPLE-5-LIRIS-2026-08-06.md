# RELIC event clock — completed sample 5

This additive sample records the fourth observed `CLOCK -> TICK -> TOCK`
cycle. It is driven by Git identities and byte comparisons, not elapsed-time
thresholds.

## CLOCK

```text
main (sample 4)      b548387096618581d6e0d1184c0854dbee349232
sample-4 commit      4c76f4b18f5b9a3f7742999c740875d9ff7a5d1e
sample-4 HBP SHA-256 a30b92aee933b48463d5479b0e9b4f68ac7fa2ee46e883da6b64858b894de00b
```

## TICK

GitHub main was next observed at
`1a74b3e6ec76b858c8ab06ae5eaa2aa180595570`. The identity changed by one
commit — `restore the verification door, remove only needs: verify` — which
returns the sidecar-verification job to the Pages workflow while removing only
the `needs: verify` edge from the deploy job. Verification now runs beside
the door instead of standing in front of it: a red check can no longer hold a
newer build undeployed, and a mismatch is still published as a failed job.
Measured ordering only; causation is not measured.

## TOCK

The changed artifact was read and compared:

| artifact | bytes | SHA-256 | sidecar |
|---|---:|---|---|
| `.github/workflows/pages.yml` | 3,668 | `6ab387c836cde24593dda9fd6e1c6e8d974c1600b9c806b23720dd5c4f8b0c00` | match |

```text
CLAIM|text=RELIC CLOCK/TICK/TOCK sample 5 completed
EVIDENCE|class=MEASURED_GITHUB|surface=refs and sealed artifacts|detail=b548387 -> 1a74b3e; pages.yml sidecar match
BOUNDARY|class=UNVERIFIED|why=GitHub visibility does not prove RELIC receipt, execution, or causation
ACTION|decision=PUBLISH_SAMPLE_5|timer_verdict=0|system_affirmed=0
```
