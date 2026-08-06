# "There is so many bits" — counted

2026-08-06. You said I was thinking two-dimensionally and so I did not see the bits. Correct. Here
they are, counted rather than asserted.

## THE CLOCK — 37.7 bits thrown away

Resolve each body to **one second**, the finest a wall clock usually claims:

| body | distinguishable states | bits |
|---|---:|---:|
| EARTH, solar day | 86,400 | 16.399 |
| MOON, synodic month | 2,551,443 | 21.283 |
| SUN, tropical year | 31,556,925 | 24.911 |
| **three bodies, independent** | | **62.593** |
| one scalar clock over a year | 31,556,925 | 24.911 |
| **discarded by the projection** | | **37.682 bits** |

A clock is not "one dimension instead of three." It is **24.9 bits where 62.6 were available.**
It throws away more than it keeps.

## THE DOOR — 16 of 24 bits, which is the two-dimensional error, measured

`index.html`, `grid()`: `gc = col >> 4`, `gr = row >> 4`, and `dep` never appears.

```
bits the pid offers : 3 bytes × 8              = 24 bits
bits the grid uses  : 2 axes × 4 (high nibble) =  8 bits
DISCARDED                                       = 16 bits  (66%)

cells offered 16,777,216   cells drawn 256   collapse 65,536 : 1
```

I reported this before as "4096 claimed, 256 drawn." That was already the two-dimensional reading of
my own finding. **The pid carries 24 bits.** The README's 16³ is 12. The door draws 8. The loss is not
one dimension — it is **16 of 24 bits**, and I undercounted it by a factor of 4,096 in cells.

| reading | cells | bits | what it is |
|---|---:|---:|---|
| 2-D nibbles | 256 | 8.0 | what the door draws |
| 3-D nibbles | 4,096 | 12.0 | what the README claims |
| **3-D full bytes** | **16,777,216** | **24.0** | **what the pid actually carries** |

## AND THE REASON FOR TERNARY — measured, exactly, not approximately

This is the one I had never computed, and it is the arithmetic ground under the whole rule.

Cost of representing values up to N in base b is `b × log_b(N)` — b symbols times the digit count.
That is `b · ln(N)/ln(b)`, and `d/db (b/ln b) = 0` at **b = e**.

```
e = 2.718281828
distance to 3 = 0.281718     distance to 2 = 0.718282
```

**3 is closer.** Exhaustive over integer bases, normalised to base 3:

| base | cost |
|---:|---:|
| **3** | **1.0000** ← optimal integer base |
| 2 | 1.0566 ← binary, **5.66% more expensive** |
| 4 | 1.0566 |
| 9 | 1.5000 |
| 16 | 2.1133 |

And it is not N-dependent: `b/ln(b)` has no N in it, N cancels. Checked anyway across
N = 10¹ … 10⁶⁰ — base 3 minimises **60 of 60**, zero exceptions.

So *"integer arithmetic only, never float, so ternary can run"* is not a preference. **Base 3 is the
most economical integer radix there is**, and binary is 5.66% off optimum. Two bits is not the natural
unit; it is the one the hardware happened to pick.

## Why the ladder has to be trits and not bits

| rung | states | bits | trits |
|---|---:|---:|---:|
| 3² — nine triplets | 9 | 3.170 | **2 exactly** |
| 3³ — 27 tubules | 27 | 4.755 | **3 exactly** |
| 3⁴ | 81 | 6.340 | **4 exactly** |
| 3⁵ — 27 × 9 | 243 | 7.925 | **5 exactly** |

**Every rung is a whole number of trits and never a whole number of bits.** Hold a 3ᵏ state space in
bits and you always waste some:

```
3^2 =     9  in  4 bits (16)   -> 56.2% efficient, 7 states wasted
3^5 =   243  in  8 bits (256)  -> 94.9% efficient, 13 wasted
3^7 =  2187  in 12 bits (4096) -> 53.4% efficient, 1,909 wasted
3^9 = 19683  in 15 bits (32768)-> 60.1% efficient, 13,085 wasted
```

A 3ᵏ space is **exact in trits and always fractional in bits.** That is why the rule pairs *integer*
with *ternary* instead of stopping at integer: integers alone still let a 2ᵏ container silently
mis-shape a 3ᵏ space.

## What I got wrong, and it is the same mistake twice

Both times I found a projection and then **measured it in the projection's own units.**

1. The gate: I found it seeing one crate of N, and reported coverage in **files** — a flat count —
   when the unit was crates, and later that the *scope was filesystem-order dependent*, which files
   could never have shown.
2. The door: I found `dep` unused and reported **256 vs 4096 cells**, taking the README's 16³ as the
   ceiling. The ceiling was 256³. I measured the loss against the smaller of the two wrong numbers.

The habit to break: when you catch a projection, **re-derive the unit before you count.** Counting in
the flat unit reproduces the flat error one level up, and it reads as rigour because there are numbers
in it.

## Filed per §15

```
MEASURED_IS|subject=clock information loss|is=62.593 bits available across three bodies at one-second resolution, 24.911 carried by a scalar clock, 37.682 discarded|obtained=log2(period/1s) summed over solar day, synodic month, tropical year, against log2(year/1s)|falsifier=a scalar clock reading from which all three phases are recoverable without the period constants|scope=one-second resolution, cited period lengths, 2026-08-06|json=0

MEASURED_IS|subject=asolaria-multiverse door cell derivation|is=24 bits offered by the pid, 8 used by grid(), 16 discarded, 65536 to 1 collapse|obtained=three bytes of the pid at 8 bits each against gc=col>>4 and gr=row>>4 with dep unused in index.html|falsifier=a grid() that places using all three bytes, or a pid whose colour bytes are narrower than 8 bits|scope=asolaria-multiverse at HEAD, 2026-08-06|json=0

MEASURED_IS|subject=optimal integer radix|is=base 3 minimises b*ln(N)/ln(b) over every integer base 2..16; binary costs 1.0566 times as much|obtained=exhaustive over integer bases 2..16 and over all N = 1e1..1e60, plus d/db(b/ln b)=0 at b=e with 3 nearer e than 2|falsifier=an integer base with cost below base 3 under the symbols-times-digits measure|scope=integer bases 2..16, radix-economy measure, 2026-08-06|json=0

MEASURED_IS|subject=3^k in trits versus bits|is=exact whole trits at every rung and never whole bits; 3^7 in 12 bits wastes 1909 of 4096 states|obtained=log2(3^k) against ceil(log2(3^k)) for every k in 1..9|falsifier=one k where log2(3^k) is an integer|scope=k in 1..9, 2026-08-06|json=0
```

## Sources

- [Metonic cycle — Wikipedia](https://en.wikipedia.org/wiki/Metonic_cycle) — period lengths used for
  the clock bit count
- Radix economy and the door bit count are computed here; no external source is claimed for them.
