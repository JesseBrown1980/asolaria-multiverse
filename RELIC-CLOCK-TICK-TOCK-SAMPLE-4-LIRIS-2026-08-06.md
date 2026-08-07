# RELIC event clock — completed sample 4

This additive sample records the third observed `CLOCK -> TICK -> TOCK`
cycle. It is driven by Git identities and byte comparisons, not elapsed-time
thresholds.

## CLOCK

```text
main (sample 3)      da909c94fdb89cbf8e550095dd49d9c4ea24c5db
sample-3 commit      89ae0f7c1ed5d3ef5d9bc28198988403788a4b42
sample-3 HBP SHA-256 11aa19af7486303dee6468eeda87c244703e73ac5ba476a67e57922a18111a4c
```

## TICK

GitHub main was next observed at
`b548387096618581d6e0d1184c0854dbee349232`. The identity changed by 14
commits: 38 files changed, 726 lines inserted, 58 deleted. The movement is the
merge of PR #2 — carrying the four-machine sung packet, the LIRIS machine
reading, the LIRIS measurements, and clock samples 2 and 3 onto main — plus
the FABLE5 kernel/bridge artifacts and the commit `take the gate off the
door`, which removed the verify job the Pages deploy declared `needs:` on.
The door now deploys ungated. Measured ordering only; causation is not
measured.

## TOCK

The newest artifacts were read and compared:

| artifact | bytes | SHA-256 | sidecar |
|---|---:|---|---|
| `FABLE5-KERNELS81-ROOMS50M-BRIDGE874X-2026-08-02.hbp` | 1,705 | `fdd406fa270ddd6fef34a6ea5209c123548159f7346dffc180b85d5d1788eaa0` | match |
| `FABLE5-SHADOW-CAT-NET-HTTP-KERNELS.hbp` | 940 | `93961f30c1152302e1163b26253d5f7f49f523e30b67b9f7375c17cf9df444ef` | match |

A repository-wide sidecar sweep at `b548387` verified 78 of 78 sidecars,
including the merged LIRIS reading and samples. The sung packet on main now
reads `THREE_READINGS_PENDING`; the ACER, RELIC, and FALCON rows remain
open.

```text
CLAIM|text=RELIC CLOCK/TICK/TOCK sample 4 completed
EVIDENCE|class=MEASURED_GITHUB|surface=refs and sealed artifacts|detail=da909c9 -> b548387; 78/78 sidecars match
BOUNDARY|class=UNVERIFIED|why=GitHub visibility does not prove RELIC receipt, execution, or causation
ACTION|decision=PUBLISH_SAMPLE_4|timer_verdict=0|system_affirmed=0
```
