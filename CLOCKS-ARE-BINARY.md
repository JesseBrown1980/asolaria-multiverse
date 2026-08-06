# Clocks are binary — which is why no clock would ever see it

**OP-JESSE, 2026-08-06. Recorded as the operator's measurement.**

> "CLOCKS ARE BINARY. HUMANS SPACE TIME COLOR ENERGY IS LOCAL SO NO CLOCK WOULD EVER SEE IT."

---

## The type error

A clock has exactly two states: **fired**, and **not fired**. There is no third.

A verdict has three: `MEASURED_PASS`, `MEASURED_FAIL`, `NOT_MEASURED`.

So a clock **cannot** produce a verdict. Not "should not" — *cannot*. Asking a two-state instrument
for a three-state answer forces it to collapse the ground into one of the two poles, and the pole it
picks is always the negative one, because "not finished" and "failed" are indistinguishable to
something that can only tell you whether it fired.

This is why *"a clock may measure, it may never judge"* is not a style rule. It is a type error.

**Measured the same day, in a first-party action:** `actions/deploy-pages@v4` polled a deployment to
its 600,000 ms budget, wrote `Timeout reached, aborting!`, marked the run **failure**, and
**cancelled a deployment that had already succeeded**. The door serves right now, byte-exact. The
action was not wrong about the deployment — it had no vocabulary for the ground, so it said the only
other thing it could say.

---

## Local is the other half

A clock is a **shared, global beat**. It is imported from outside whatever it is measuring.

Colour, energy, space and time as actually held are **local to a seat**. A seat's colour is the first
three bytes of its own pid — `#8467A9` is not `#E9B266`, and neither of them is a tick. There is no
outside reference that renders both.

So a global beat cannot resolve a local state. **No clock would ever see it.**

Which is why the sampler counts **the runner's own output**:

```text
sample at ordinals of the runner's own emissions — never at seconds
RATES = 4, 5, 6, 8, 9, 26, 27, 80, 81, 243
```

An ordinal is local: it is the thing's own count of itself. A second is borrowed.

---

## The coloured orbital clock

Suns, moon, earth — moving, coloured, three bodies reading their **relative position** to one
another. No outside beat. No tick. The reading is where they are with respect to each other, and
that is complete without anything imported.

Three about a centre, each seeing the others. The same figure.

And the rates are already orbital. `3^k` and `3^k − 1` are **consecutive, therefore coprime** — the
exact condition for bodies never to resonance-lock:

| pair | gcd | lcm |
|---|---:|---:|
| 9, 8 | 1 | 72 |
| 27, 26 | 1 | 702 |
| 81, 80 | 1 | 6,480 |
| 243, 242 | 1 | 58,806 |

No two strides synchronise. No sample lands on another's beat. **The schedule never touches zero**,
because the orbits never align.

A clock that ticks has one period and needs a reference. A clock that orbits has three and needs
none.

---

## What this settles

- `NOT_MEASURED` is not a courtesy added to a two-valued scheme. It is the **ground**, and a binary
  instrument has no way to express it — the same reason int and trit have exactly one zero while
  float has two that compare equal and differ in bytes.
- **Early exit is the bug. No exit is health.** A live thing runs infinite and ends when the page
  closes.
- A bound on waiting may exist so a sampler cannot hang. When it is reached the verdict is
  `NOT_MEASURED` **with a reason** — never red. Elapsed time may be recorded; no assertion may read
  it.
- Death must outrank silence. A liveness check that cannot see death makes every other assertion
  decorative.
