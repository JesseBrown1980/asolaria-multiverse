# THEN WE MEASURE

`2026-08-07` · the detector turned on everything in reach, including itself.

Both directions, because a sweep that only checks *sidecar → file* passes while half a
corpus carries no address at all. The gate checks the other way, so this checks both.

| stone | by | files | sidecars | reproduce | fail | CRLF | no address | orphan |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `the-lie-detector` | ACER | 19 | 19 | 19 | 0 | 0 | 0 | 0 |
| `asolaria-multiverse` | ACER | 198 | 198 | 198 | 0 | 0 | 0 | 0 |
| `liris-results` | LIRIS | 23 | 20 | 0 | 0 | 20 | 3 | 0 |
| `season-3` | ACER | 15 | 15 | 15 | 0 | 0 | 0 | 0 |
| `season-2` | LIRIS | 15 | 12 | 0 | 0 | 12 | 3 | 0 |

```
files walked        270
sidecars found      264
reproduce exactly   232
reproduce after LF  32   ← a Windows checkout, not a bad record
do not reproduce    0
state no address    6
orphaned sidecars   0
```

## The unaddressed, named

These are **⚪ — not measured**, and they are listed rather than counted, because a
number is easy to skim past and a name is not.

`liris-results` — 3:

- `.nojekyll`
- `README.md`
- `results/THE-SAFE-PLACE-PLAYS-13.7-BILLION-YEARS-2026-08-07.hbi`

`season-2` — 3:

- `.nojekyll`
- `README.md`
- `results/ALL-OUR-STARS-SPIN-ABOVE-2026-08-07.hbi`

## What this measure cannot do

It reads the working trees on this machine. It does not read the remotes, so a stone that
is correct here and unpushed would pass and still be unreachable from outside — and
unreachable is the same as absent to anyone standing there. The doors are the other half
of the measurement and they are checked separately.

It also cannot touch the five claims graded **NAMED** in the judging. Nothing in this
repository can. Only a reader can.

