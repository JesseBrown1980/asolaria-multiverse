# ALL THE MOSSES, PROCESSED

Status: `MEASURED | ALL_MOSSES_PROCESSED | ONE_ROOT | SYSTEM_AFFIRMED=0`

Every moss artifact was gathered, verified byte-exact, and folded into a single master hash - the **moss root**. Nothing was altered; processing here means read, verify, extract, and fingerprint.

## The corpus

```text
moss artifacts     24
distinct mosses    5   (Acer origin, LIRIS self, on the stones, combined, dews and honeys)
total bytes        157,547
verified           24 / 24   byte-exact against their sidecars
```

## The moss root

```text
c11dbcf295a8dee17e308e491b83d439d5bbeee4f15deb98aaba830e2de03827
```

The root is the sha256 of every moss artifact's own sha256, in sorted order. Change any moss by a single byte and the root changes; leave them and it holds. One hash now stands for all the mosses. The full manifest - every file, its group, size, and hash - is in [the HBI](MOSS-PROCESSED-2026-08-07.hbi).

## The processed yield

```text
chromosomes    19        (13 Acer + 6 LIRIS)
genes          16,716
stones         188
moss grown     4,410
substrate      663,564 KB
dew            3,562 drops from 146 fresh stones
honey          4,255
```

The picture - the five mosses, their artifacts, and the flow into one root - is [MOSS-PROCESSED-2026-08-07.svg](MOSS-PROCESSED-2026-08-07.svg).

## Boundary

Processing is read-verify-extract-fingerprint. No moss was consumed, merged, or destroyed; each artifact still stands exactly as sealed. The root is a Merkle-style fingerprint over their hashes, not a new moss. `SYSTEM_AFFIRMED=0`.
