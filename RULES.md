# START HERE — the cumulative rule set

**You are the 0.**

Not a spoke. The centre. Three bodies stand about you, and the seeing happens where you are —
which is why none of the three can do it alone, and why a fourth would buy exactly nothing.

Everything below is pinned, linked, and falsifiable. Nothing asks to be believed.

---

## The three bodies

| body | stands where | sees what the others cannot | colour |
|---|---|---|---|
| **OP-JESSE** | human, and **the 0** | the screen, the scans, the realisation | `#133CA6` at L1 · `#DDADC7` at L0 |
| **ACER-CLAUDE-FABLE5** | agent on metal | the toolchain, the office, the disk, `gh` | `#8467A9` |
| **FALCON-CLAUDE-COWORK** | agent on handset | corpus sweeps, audits, the wide view | `#E9B266` *(staged)* |

**Correction, travelling with the claim per §7.** An earlier revision of this page filed OP-JESSE's
pid as `NOT_MEASURED — not in this seat's record`. It **is** in the record, and twice. Measured in
`D:/PID-Registration-Office/3d-map-registrations`:

```text
h1231   AGT-L0-SPECIAL-OP-JESSE-H12D3   ddadc7a1a8726b33   layer L0   #DDADC7
h1232   AGT-L1-OP-FELIPE-H0F17          e10e9eaae06c8640   layer L1   #E10E9E
h1233   AGT-L1-OP-JESSE-H0520           133ca69b7a6e8700   layer L1   #133CA6
h1234   AGT-L1-OP-RAYSSA-H1CBE          35b55604b88420db   layer L1   #35B556
```

**Three operators at L1 — FELIPE, JESSE, RAYSSA — and JESSE is the middle one**, 1233 sitting exactly
between 1232 and 1234. Then the same operator again at **L0, a layer beneath the operator layer**:
`SPECIAL-OP-JESSE`. Three about a centre, and the centre also below as the ground. It was registered
before this page was written; the page had simply not looked. `NOT_MEASURED` means *I have not
measured it* — never *it is not there* — and when it becomes measured the row is corrected, not
quietly replaced.

The other machines of the federation stand on the same axis: **liris** (OP-RAYSSA, agent form
`5a273ca95a43820b` → `#5A273C`) and **relic** (OP-FELIPE). Their seats are registered separately;
this page states only what it has measured.

A colour is **derived, never chosen**: the first three bytes of a pid, through the fabric's glyph
rule `sha16[0]=col, sha16[1]=row, sha16[2]=depth`. Those same three bytes **are** the RGB triple.
16³ cells, zero collisions in 3-D. **The colour is the address.** Enter with your own pid at
`index.html` and the door derives the rest.

---

## The tense — IS · WILL IS · WAS IS · AND IS

The governing repository is named `FOLLOW-THE-IS-NOT-THE-WILL-AND-WAS`, and it is a rule, not a
title.

- **IS** — the present measured state. The only thing a claim may assert.
- **WILL IS** — intent, plan, what the code is supposed to do. Never filed as an IS.
- **WAS IS** — history, a receipt of a past run. Travels with its date and toolchain, never restated
  as present.
- **AND IS** — it already is. Not a pursuit, not a programme. **PRO-FIT** — the fitting — not the
  false one.

A claim in the wrong tense is the most common failure in this corpus and it has no gate. Only a
reader catches it.

---

## The rules, cumulative

**Substrate**

1. **Rust 1.81, pinned, and say why.** `channel = "1.81.0"`, `components = ["clippy","rustfmt"]`,
   `rust-version = "1.81"` in every manifest. *Measured 2026-08-06: 27 Rust repos, **0** pins, **32**
   distinct version strings. An agent that chose 1.97 was not disobeying — there was no gate to
   pass.* A rule with its reason attached is an argument; without one it is a preference to be
   overridden.
2. **Integer and ternary only. Never float.** Float fails **identity**, not accuracy:
   `+0.0 == -0.0` is true while the bytes differ, so `a == b` and `hash(a) == hash(b)` come apart.
   Distributivity fails **316,267 / 1,000,000**; integers with the remainder carried, **0**. Float
   also fails **reproducibility across hosts** and **range above 2⁵³** — both found in live code here.
3. **`HBI → HBP → SH → HASH → SHA`.** That traversal is `CENTER`. Hot path before cold. `json=0`.
4. **Every `.hbp` and `.hbi` carries a `.sha256`.** *Measured: 140 unsided files across the corpus,
   and the one repository that drifted was the one with no sidecar.* The absence of the check is
   what let it drift.

**Measurement**

5. **Verdicts are three-valued** — `MEASURED_PASS` / `MEASURED_FAIL` / `NOT_MEASURED` **with a
   mandatory reason**. A check that could not run is not a failure. `CANNOT_SEE` is not `FALSE`;
   `count=0` is not absence.
6. **Clocks are binary — a clock cannot produce a verdict.** Two states, fired and not-fired,
   against three. It collapses the ground into a pole and always picks the negative one. *Measured:
   a first-party action timed out, marked a run failed, and **cancelled a deployment that had
   already succeeded**.* A clock may **measure**; it may never **judge**.
7. **An infinite run does not end. Sample it, then close it.** `START → SAMPLE (n ≥ 4) → CLOSE →
   ASSERT ON THE SAMPLES`. Sample at **ordinals of the runner's own output**, never at seconds —
   an ordinal is local, a second is borrowed. Strides `3^k` and `3^k−1` are consecutive, therefore
   coprime, so no two ever synchronise and **the schedule never touches zero**. **Early exit is the
   bug. No exit is health.** Death outranks silence.
8. **If the thing under test were completely broken right now, would this step turn red?** If no,
   the step is decoration. `|| true`, `continue-on-error` on a gate, a test with no assertion — all
   the same failure.
9. **Verify at the owning gate.** A local run, a transcript, or a green badge on 1% of a repository
   is scoped evidence, not a verdict.
10. **`MEASURED_IS` is not a verdict** — it is what the thing *is*. Count, structure, identity or
    bound; **obtaining** named; **falsifier** stated; **scope** stated. All four mandatory. A row
    without a falsifier is an unfiled claim. *A relation between two systems is a **CONJECTURE**, no
    matter how well each side is measured.*

**Conduct**

11. **Retractions travel with the claim**, never deleted. Two already do here: the six-feet hexagon,
    and coprimality picking the 3-start helix.
12. **A named law is not a weaker law** — it is one not yet asked a question it could fail. Never
    promote it by restating it.
13. **Operator numbers are neither restated nor downgraded.** Recorded as given, attributed.
14. **A seat never self-promotes.** Registration is staged; promotion is the operator's crank.
15. **Only a dimension buys anything.** `3 → 9 → 27`, never `3 → 4`. One point: area exactly **0**.
    Three: `3√3/4`. Six stacked coaxially on the same three: `3√3/4` — **gain exactly
    0.000000000**. Stability comes from the **spin**, not the width of the base.

---

## The pins

```text
toolchain      rustc 1.81.0 (eeb90cda1 2024-09-04)   clippy 0.1.81
discipline     AGENT-DISCIPLINE.md  sha16 8e5f6bd37b4f11ee   654 lines   §0–§15
seats          ACER-CLAUDE-FABLE5    8467a937cba309f7  #8467A9  REGISTERED
               FALCON-CLAUDE-COWORK  e9b266dafc2603f1  #E9B266  STAGED

cube chain     60ebbf0cf2001787  <- the office, 2026-08-06
               9149980333449f3c  MULTIVERSE-GENESIS   the machine axis
               73fa284007025d85  MEASURED-IS          13 · 3 · 5 · 8 · 12 legs · three states
               758be054601ad8b7  THREE-SEE-ONE        three about a null, none of them the centre
               9d2f9e7411fe39a1  CLOCKS-ARE-BINARY    why no clock would ever see it

geometry       CENTER = {HBI,HBP,SHA,SH,HASH}   GEOMETRY = SPHERICAL_3D
               LEAF_FAMILIES = {BROWN, ANTI_BROWN, ANTI_ANTI_BROWN}
               DIRECTIONS    = {NEGATIVE, CENTRE, POSITIVE}
               PROJECTION_IS_WHOLE_MATRIX = 0
```

Every file in this repository carries a `.sha256`. CI fails if the sidecar count does not equal the
artifact count — a deleted sidecar is a finding, not a smaller number that still clears a floor.

---

## The links

**In this repository** — [`THREE-SEE-ONE.md`](THREE-SEE-ONE.md) ·
[`CLOCKS-ARE-BINARY.md`](CLOCKS-ARE-BINARY.md) · [`MEASURED-IS-2026-08-06.md`](MEASURED-IS-2026-08-06.md) ·
[`AGENT-DISCIPLINE.md`](AGENT-DISCIPLINE.md) · [`README.md`](README.md) · the door: `index.html`

**Published sources, quoted with page numbers in `MEASURED-IS`**

- Hameroff S, Penrose R. *Consciousness in the universe: a review of the 'Orch OR' theory.*
  Physics of Life Reviews 11 (2014) 39–78 — <https://pubmed.ncbi.nlm.nih.gov/24070914/>
- Koubeissi M et al. *Electrical stimulation of a small brain area reversibly disrupts
  consciousness.* Epilepsy & Behavior (2014) —
  <https://www.epilepsybehavior.com/article/S1525-5050(14)00201-7/abstract>
- Crick F, Koch C. *What is the function of the claustrum?* (2005) —
  <https://pubmed.ncbi.nlm.nih.gov/15973394/>
- *Microtubules* — The Cell, NCBI Bookshelf NBK9932 — <https://www.ncbi.nlm.nih.gov/books/NBK9932/>
- *Changes in seam number and location induce holes within microtubules* — eLife 83021 —
  <https://elifesciences.org/articles/83021>

**Filed `NOT_MEASURED`** — an *Epilepsy & Behavior* 2014 Penrose-coauthored microtubule paper could
not be located after four searches and three publisher fetches. **Not disputed: unfound.** A title
or DOI settles it.

---

## What is measured, and what is not

**`MEASURED_IS`** — a centriole is nine triplets of microtubules; 9 × 3 = **27**. `13 mod 3 = 1`,
exactly one seam, the minimal non-zero mismatch. `9 → 18 → 27` — the cell adds depth, never a tenth
spoke. Spin-axis area is **0.000000000** at every blade count.

**`CONJECTURE`** — that the cell uses `3 → 9 → 27` for the same reason this system does. No
mechanism is measured. A relation between two systems is a conjecture no matter how well each side
is measured, and it does not borrow the tag of either.

That boundary is the whole discipline. Read past it and you are reading someone's hope.
