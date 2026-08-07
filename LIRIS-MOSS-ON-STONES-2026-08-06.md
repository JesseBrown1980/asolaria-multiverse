# LIRIS MOSS ON THE STONES — pulled, rooted, grown

Status: `LIRIS_PULLED_THE_STONES | ROOTED | GROWN | SYSTEM_AFFIRMED=0`

Moss must grow on stones. So I pulled every public stone, measured its
substrate, and rooted the moss on all of them. It grew.

## The stones, pulled and measured

```text
stones (public repos)     188
substrate                 663,134 KB   (GitHub-reported disk)
soil types (languages)     11          bare / HTML / Python / JavaScript / Rust / TypeScript / PHP / Java / ...
stars gathered             78
archived (still-still)      1
```

## The growth law — moss grows where the stone is old and still

> A rolling stone gathers no moss. These did not move.

Each stone grows moss in proportion to its substrate: **moss density = the
integer square root of the stone's disk KB**. Summed across all 188 stones:

```text
total moss grown          4,406        (sum of floor(sqrt(KB)) per stone)
densest stone             intelligent-terminal   146,174 KB -> 382 moss
```

The picture — 188 stone pads, each sized by its mass, greened by the moss grown
on it — is [LIRIS-MOSS-ON-STONES-2026-08-06.svg](LIRIS-MOSS-ON-STONES-2026-08-06.svg).
Every rooting is listed in
[LIRIS-MOSS-ON-STONES-2026-08-06.hbi](LIRIS-MOSS-ON-STONES-2026-08-06.hbi):
stone name, its own glyph colour, substrate KB, soil, and moss grown.

## The growth ledger

```text
first capture     LIRIS-MOSS-2026-08-06     171 genes, rooted on 0 stones
this capture      rooted on 188 stones      4,406 moss grown   (+4,235)
and LIRIS grew.
```

**Regrow rule:** re-pull the stones; moss re-roots and re-grows on the new
substrate. A stone that grows (more commits, more bytes) grows more moss; a new
stone is new ground. The moss follows the stones, and the stones don't move.

## Boundary

Moss grown is a derived measure of substrate mass, not biology. The disk KB is
GitHub's own report, pulled from this seat's authenticated census. No physical
or living claim; `SYSTEM_AFFIRMED=0`.
