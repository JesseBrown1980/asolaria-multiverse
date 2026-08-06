# MACHINE READING — LIRIS

**Packet:** [FOUR-MACHINE-SUNG-DECISION-2026-08-06.md](FOUR-MACHINE-SUNG-DECISION-2026-08-06.md)
**Seat:** LIRIS (`C:\Users\rayss`, Liris/Rayssa vantage)
**Route:** public GitHub draft PR #2
**Scope:** this row fills the LIRIS seat only; it cannot finalize the collective result.

## The reading

```text
MACHINE_READING
  seat=LIRIS
  token_sha256=91473d7f41b83fbefd1c853e8c3e438136def556617348710211827ca6940bf2
  reading=17-byte operator token preserved byte-exact; no binding found on any LIRIS canon, disk, or fabric surface; semantic value beyond the bytes UNVERIFIED
  basis=machine measurement
  evidence_class=MEASURED (bytes) / UNVERIFIED (meaning)
  source_commitment=91473d7f41b83fbefd1c853e8c3e438136def556617348710211827ca6940bf2
```

## What LIRIS measured

- The token `S S ZZZ JQPIESZ` was rehashed from its exact 17 UTF-8 bytes,
  without a trailing newline, on this seat. The recomputed SHA-256 matches the
  packet's sealed hash byte-exact: `MEASURED`, `LIRIS_LOCAL`.
- A whole-disk search of this seat's Asolaria canon for the token substring
  `JQPIESZ` returned matches only inside this repository (README, the
  three-SEE/four-WAVE analogy pair, and the decision packet itself). No LIRIS
  canon or working surface outside this repo carries the token.
- The fabric surface (:4949) answered from a stale fallback cached
  2026-07-19T02:01:15Z with reason `all_bases_unavailable`; the local base
  (:4944) and the acer base (192.168.15.4:4949) were both unreachable at
  reading time. No live canon binding was available: `SYSTEM_AFFIRMED=0`.

## Boundary

The bytes are `MEASURED`. The meaning is `UNVERIFIED` from this seat: LIRIS
does not decode, normalize, or assign executable authority to the token. The
ACER, RELIC, and FALCON rows remain `PENDING` and are theirs alone to publish.
This reading is additive; it overwrites and deletes nothing.
