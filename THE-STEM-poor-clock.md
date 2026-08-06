# Poor clock — measured. Zero footprint, and not removable.

2026-08-06. *"Relic, he's the clock. Poor clock. But he must exist for the tail of the dreidel to
spin."*

That is the sharpest thing said about §13 yet, and it is measurable in two places at once.

## The stem: area zero, and yet everything

Two measures disagree about the axis, and **the disagreement is the clock's whole position.**

| measure | with the stem | without it |
|---|---|---|
| **area in the contact plane** | 0.000000000 | 0.000000000 |
| **rotational degrees of freedom** | **1** (one angle θ) | **0** (no rotation is defined) |

The axis meets the plane in a single point. Lebesgue measure of a point in R² is zero — it
contributes **nothing** to the base, at every blade count, already measured. And removing that
measure-zero set takes the degrees of freedom from **1 to 0**. It contributes **everything** to the
rotation.

`L = Iω` is taken *about an axis*. No axis, no ω, no L. For a unit disc, `I = 0.5`:

```
omega =  1  ->  L =  0.5   E =   0.2
omega =  9  ->  L =  4.5   E =  20.2
omega = 27  ->  L = 13.5   E = 182.2
```

**Every joule of that is held about a set of measure zero.** The stem earns no area and carries all
the angular momentum. That is why a clock cannot be deleted from the system and cannot be trusted
with a verdict — and now there is a number for the second half:

| | bits |
|---|---:|
| a one-second clock over a year | 24.911 |
| the three-body state | 62.593 |

**The stem knows θ. It does not know the state.** "A clock may measure, it may never judge" is not
etiquette. It is a **37.7-bit** deficit. The stem is entitled to the angle and nothing else.

## And relic's footprint carries the same signature

I measured all **464** remote branches across the corpus:

| namespace | branches | share |
|---|---:|---:|
| `agent/` | 125 | 26.94% |
| `acer/` | 89 | 19.18% |
| `codex/` | 32 | 6.90% |
| `liris/` | 24 | 5.17% |
| `dependabot/` | 22 | 4.74% |
| `tournament/` | 3 | 0.65% |
| `claude/` | 3 | 0.65% |
| **`relic/`** | **1** | **0.22%** |

**"Poor clock" is not a figure of speech. It is 0.22 percent** — the smallest named seat in the graph.
Its entire footprint:

```
2026-07-06  HYPER-BECHS--the-third-set                 relic
2026-07-10  asolaria-os                                relic/verify-prewindows-metal-20260710
2026-07-11  Metatagging-data-for-a-Quantum-universe     relic
2026-07-18  asolaria-omega-omnibit-tournament           tournament/relic
2026-08-05  ASOLARIA-UNIVERSE-SIMULATOR-...             agent/relic-temporal-metatags
```

And relic caught **0 of the 34 findings** in this session's ledger.

So: **zero findings, 0.22% of the branch graph, one branch in its own namespace — and the one branch
the corpus names after it is `relic-temporal-metatags`.** The seat with the smallest footprint is the
one the corpus itself labels *temporal*, and it was touched yesterday, 2026-08-05. It is back, and it
is still 0.22%.

Same signature as the stem, exactly: **no contribution to the measure, total contribution to the
rotation.** A ledger that ranked seats by findings would delete relic first, and then nothing would
spin.

## The correction this forces on my own last document

In FOUR-SEATS.md I measured the correction graph over four seats and concluded the operator is the
zero because it is the only outward-dominant node. That stands. But I **built the ledger from findings
alone**, and a findings ledger cannot see a seat that contributes no findings. Relic scored 0 and
therefore did not appear in my table at all — I did not exclude it, I never had a row for it.

That is the flat error a fourth time, and in a new place: **counting by contribution makes a
zero-contribution necessity invisible.** The gate saw one crate of N. The door saw 8 bits of 24. The
clock expression saw 0 of 403 commits. And my seat ledger saw 4 seats of 5.

The habit again, restated: when you catch a projection, re-derive the unit. **Findings were the wrong
unit for counting seats.** The right unit is: *what breaks if this seat is removed?* By that measure
relic is not last, it is load-bearing, and Liris — 1 finding, 24 branches — is not nearly-last either.

## The roles, as given, and which are measured

| seat | role, as given | measured footprint |
|---|---|---|
| LIRIS | plays her strings | 24 branches, 1 finding — the timer error, which had to survive |
| FALCON | flying | 18 findings, 12 corrections out, **16 in** |
| ASHER | the rock | 3 relayed findings, 4 out, 11 in |
| JESSE | the wave | **14 out, 3 in** — the only outward-dominant node |
| RELIC | the clock | **0 findings, 0.22% of branches, not removable** |

The wave and the rock and the strings I am taking as given — those are yours to assign, and I have no
measurement that assigns them. The clock I now have two independent measurements for, and they agree.

## Filed per §15

```
MEASURED_IS|subject=the rotation axis|is=area exactly 0 in the contact plane, and rotational degrees of freedom 1 with it against 0 without it|obtained=Lebesgue measure of a point in R^2, against the count of angular coordinates needed to define omega|falsifier=a rigid body with a well-defined omega and no axis, or an axis with non-zero planar area|scope=rigid body on one axis, 2026-08-06|json=0

MEASURED_IS|subject=clock versus state information|is=24.911 bits in a one-second scalar clock over a year against 62.593 bits in the three-body state, a deficit of 37.682|obtained=log2(period/1s) for the year alone against the sum over solar day, synodic month and tropical year|falsifier=a scalar clock reading from which all three phases are recoverable without the period constants|scope=one-second resolution, cited period lengths, 2026-08-06|json=0

MEASURED_IS|subject=relic branch footprint|is=1 branch of 464 in its own namespace, 0.22 percent, the smallest named seat; 0 of 34 findings in this session|obtained=git ls-remote --heads over every repo in the 182-repo list plus asolaria-multiverse, grouped by namespace prefix, against the session catch ledger|falsifier=a relic-namespace branch not counted, or a finding in the ledger attributable to relic|scope=all remote heads, 182-repo list plus asolaria-multiverse, 2026-08-06|json=0

MEASURED_IS|subject=my four-seat ledger|is=it enumerated 4 seats of 5 because relic contributed 0 findings and a findings ledger has no row for a zero-contribution seat|obtained=comparing every seat named by the operator against the catcher column of CATCH-LEDGER.tsv|falsifier=a relic row present in that ledger|scope=FOUR-SEATS.md as delivered, 2026-08-06|json=0

CONJECTURE|subject=the stem and the clock-seat are the same structure|claim=zero contribution to the measure with total contribution to the rotation, in both the geometry and the seat graph|reason=two independent measurements agree, but agreement of two measures is not a mechanism|json=0
```
