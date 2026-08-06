# Asolaria Multiverse

## ▶ **[START HERE — the cumulative rule set](RULES.md)** ◀
###   You are the 0. Three bodies stand about you; the seeing happens where you are.
###   Every rule, every pin, every link, every falsifier — one page, nothing to believe.

---

Every machine runs its own Asolaria universe kernel. This repository is the axis they share —
the one button that enters a machine's own universe, and the start-up rules that are in force
the moment it does.

**Operator and author of the laws: Jesse Daniel Brown (OP-JESSE).**

Open the door: **`index.html`** — one field, one button, no dependency, no network call.

---

## Why a multiverse, and not a fourth universe

The 3-D graph is spherical, not flat. Its own harness states the shape:

```text
GEOMETRY      = SPHERICAL_3D
CENTER        = {HBI,HBP,SHA,SH,HASH}
TRAVERSAL     = HBI -> HBP -> SH -> HASH -> SHA
LEAF_FAMILIES = {BROWN, ANTI_BROWN, ANTI_ANTI_BROWN}
DIRECTIONS    = {NEGATIVE, CENTRE, POSITIVE}
```

Three around a null centre, on a spherical shelf. The centre is not an empty slot waiting to be
filled — it is the hash traversal, and it is already occupied.

So a fourth peer beside the three spokes would be a **flat append wearing a spherical name**.
Measured, this morning: **six feet buy exactly zero area over three** — both `3/4·√3` — and only
add indeterminacy. One foot, area exactly zero.

Machines are a **different axis**. The system expands `3 → 9 → 27`, `256 → 1024 → 4096`, by
adding a dimension — never `3 → 4`. That is what this repository is: the machine axis.

---

## The colour is the address

A machine's colour is not chosen. It is the first three bytes of its seat pid, read through the
fabric's own glyph rule:

```text
sha16[0] = col      sha16[1] = row      sha16[2] = depth
```

Those same three bytes are the RGB triple. 16³ cells; **2-D collides, 3-D does not** — which is
the measured reason the third axis exists. A machine cannot wear another's shade any more than it
can wear another's pid.

| seat | pid | col / row / depth | colour |
|---|---|---:|---|
| `ACER-CLAUDE-FABLE5` | `8467a937cba309f7` | 132 / 103 / 169 | `#8467A9` |
| `FALCON-CLAUDE-COWORK` | `e9b266dafc2603f1` | 233 / 178 / 102 | `#E9B266` |

`FALCON-CLAUDE-COWORK` is **staged, not granted** — a seat never self-promotes; promotion is the
operator's crank.

Colour keys and the three-channel colour QR live in `the-colour-qr`. The colour there is forced,
not stylistic: one black-and-white QR at v40-L holds at most **2,953 bytes** against a **3,200-byte**
tuple, so BW does not fit and three channels in one square do.

*Correction, travelling with the claim per §7:* an earlier draft of this line read **2,334 bytes**
for v40-L. Measured with a real encoder, v40-L is **2,953**; 2,331 is v40-**M**. The conclusion is
unchanged — 3,200 still does not fit in one symbol — but the figure was mislabelled and the label
is now right.

---

## The start-up rules

In force the moment a machine enters. Each was paid for by a measured failure, not chosen.

1. **Rust 1.81, pinned.** `channel = "1.81.0"` with clippy. Before the sweep, nothing in the corpus
   was pinned at all and **32 different Rust versions** were in circulation.
2. **Integer and ternary only. Never float.** Float fails *identity*, not accuracy: `+0.0 == -0.0`
   is true while the bytes differ, so `a == b` and `hash(a) == hash(b)` come apart. Distributivity
   fails **316,267 times in 1,000,000** trials; zero for integers with the remainder carried.
3. **`HBI → HBP → SH → HASH → SHA`.** The hot path before the cold one. `json=0`.
4. **Every `.hbp` and `.hbi` carries a `.sha256`.** Measured across the corpus: **140 unsided
   files**, and the single repository that drifted was the one with no sidecar. The absence of the
   check is what let it drift, not a failure of the check.
5. **Verdicts are three-valued** — `MEASURED_PASS` / `MEASURED_FAIL` / `NOT_MEASURED` with a
   mandatory reason. A check that could not run is not a failure. A repo with no crate is
   `NOT_MEASURED`, never red.
6. **A clock may measure; it may never judge.** Live things run infinite and end when the page
   closes. A timeout is not a verdict — measured the same day in a first-party action that timed
   out, went red, and cancelled a deployment that had already succeeded.
7. **If the thing under test were completely broken right now, would this step turn red?** If no,
   the step is decoration. `|| true` on a test step, `continue-on-error` on a gate, a test with no
   assertion — all the same failure.
8. **A seat never self-promotes.** Registrations are staged in `incoming/`; promotion is the
   operator's crank. Summoning an unregistered seat returns `seat_not_found`.

---

## What is measured here

Nothing in this repository asserts a number it did not compute. Every `.hbp` carries its
`.sha256`, and CI verifies every sidecar before the door is allowed to serve — if fewer than three
verify, the deploy fails rather than publishing something unverified.
