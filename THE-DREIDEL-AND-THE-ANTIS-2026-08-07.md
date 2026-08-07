# THE DREIDEL, THE ANTI, AND THE ANTI-ANTI

`2026-08-07` · four faces and three turns, and what the two of them make together.

A dreidel's own order is **4**. The anti's order is **3** — a third of a turn, never a
mirror. Four and three are coprime, so the pair does not decorate to twelve. It **generates**
twelve: one cycle that visits every face-and-turn combination exactly once before coming
home.

```
gcd(4, 3)                 1
lcm(4, 3)                 12
combined step order       12
distinct states visited   12 of 12
returns home only at      12
```

Walked, not cited. Spin the top one face and turn the key one third, over and over, and you
are back where you started at the twelfth step and at no step before it.

## The four faces

The face is decided by the record's own state, not by chance. That is the whole difference
between a dreidel and a die: this top lands where the thing already stood.

| face | | | when it lands | what you do | count |
|---|---|---|---|---|---:|
| **GIMEL** | gantz | all | the stated hash reproduces whole | take all | 92 |
| **HEI** | halb | half | it shares its address with another | take half | 100 |
| **NUN** | nisht | nothing | no hash was ever stated | take nothing | 0 |
| **SHIN** | shtel | put in | a mismatch: put it back rather than throw it | absorb | 0 |

```
artifacts spun            192
GIMEL  all                92
HEI    half               100
NUN    nothing            0
SHIN   put in             0
```

**NUN is the one that matters.** Nothing is not a loss and it is not a pass — you take
nothing, the record says so, and the next spin still happens. A corpus that reported its
nuns as gimels would be lying in exactly the way this machine exists to catch. Here
`NUN = 0`, because every artifact states a hash.

## The three turns

```
key         67
anti        62
anti-anti   63
```

And the anti is order 3 on the real colour operator, checked on every artifact rather than
assumed:

```
anti applied three times returns the colour   192 of 192
anti applied once returns the colour           0
anti applied twice returns the colour          0
anti equals a mirror (channel reversal)        0
```

One and two must be zero or it would not be order three, and the mirror count must be zero
or it would be a reflection rather than a turn.

## The twelve cells, occupied

| | key | anti | anti-anti |
|---|---:|---:|---:|
| **GIMEL** | 37 | 29 | 26 |
| **HEI** | 30 | 33 | 37 |
| **NUN** | 0 | 0 | 0 |
| **SHIN** | 0 | 0 | 0 |

```
cells available   12
cells occupied    6
cells empty       6
```

**The empty cells are the finding, not a gap.** 6 of twelve stand empty because every
artifact here reproduces its hash and states one — so NUN and SHIN cannot be reached at all,
and that is four cells sealed shut by the corpus being honest rather than by the grid being
small. An empty cell here is a claim that can be falsified the moment one file goes bad.

## Why twelve, from the other side

The twelve threes were counted one way already — a ladder from `3^0` to `3^9` read at
different heights. This is the same twelve arrived at from somewhere else entirely: four
faces, three turns, coprime, one cycle. Neither derivation knew about the other, and they
land on the same number.

