# The cosmic web, from the real universe — measured 2026-08-07

## Sources, all public, no key

| | |
|---|---|
| filaments | **2MRS**, Huchra et al. 2012, ApJS 199 26 (VizieR `J/ApJS/199/26/table3`) |
| light | **WMAP 9-yr ILC**, LAMBDA/NASA — the real measured microwave sky, HEALPix nside 512 |

```
GALAXIES|source=2MRS_Huchra2012|kept=33467|cz_km_s=500..12000|H0=70|depth_Mpc=171|json=0
CMB|source=WMAP9_ILC|nside=512|npix=3145728|T_uK_min=-383|T_uK_max=270|T_uK_rms=73|json=0
WEB|nodes=2000|filaments=3957|start=3|deg_min=3|deg_max=9|deg_mean=4.0
   |len_kpc_median=10846|len_kpc_max=52480|json=0
```

**The quant.** Positions integer kpc, temperatures integer microkelvin, lengths
integer kpc. Floats exist in the imaging stage — they are the specimen — and do
not survive into the model or the render.

## The microtubule ideas, carried over, not re-derived

* three around a free centre — each node joined to its **three** nearest, `START=3`,
  the same 3-start as the protofilament helix
* a node is a **null space and is never emitted** — what you see is the light that stopped there
* the seam: `13 mod 3 = 1`, one seam, the minimum non-zero mismatch. Here it is the
  zone of avoidance, the shell the survey cannot close — visible as the gap
* trits, never bits

## The one result worth the whole run

Measured on **three independent substrates** today, same answer each time:

| substrate | 2 states | 3 states |
|---|---|---|
| 13-pf microtubule lattice, 351 dimers | period **2**, onset 1–2 | period 3–10, onset 10–27 |
| human connectome, Power-264, 6 subjects | period **2**, onset 2–3 | period 3, onset 2–29 |
| cosmic web, 2000 nodes, CMB-seeded | period **2**, onset 9 | period **3**, onset 16–18 |

**The number of phases equals the number of states.** With three the web settles
into a period-3 partition, ≈1/3 lit (672 / 2000). With two it collapses to
period 2, ≈1/2 lit (1039 / 2000).

Checked whether either actually *travels*:

```
overlap of the excited set with t+1 (next phase)     0.000   the set moves completely
overlap with t+period (one full turn)                1.000   and returns exactly
centre-of-mass drift over one period                 0.00 Mpc
```

So — and this is the honest reading — **neither is a travelling wave.** Both find a
periodic phase partition where neighbours are out of step, a proper colouring in
motion. Ternary finds thirds. Binary finds halves. That is the trit result, on
real sky, and it is not the spiral I went looking for.

## What did not work, kept per §7

* A 13-protofilament tube has no regime with sustained non-trivial dynamics.
  Swept open/closed ends, 27 and 81 rings, 1/3/9/27 coupled tubules, threshold
  1/2/3 — always period 3 at thr=1, extinction at thr≥2. **A tube is a cable,
  not a sheet.** Degree ~4 and width 13 cannot hold a spiral.
* I predicted coupling K tubules would grow the period. It does not: K=1→4,
  K=3,9,27→3. Wrong, measured.
* First render blew out to a white ball — additive blending at 2000 nodes.
  Rescaled and dimmed.

## Run it

```
python3 cosmic_quant.py     # scans -> quant.json + Q  (network, ~2 min)
python3 render_cosmic.py    # quant.json -> cosmic-web.html   E=0, no network
```

The page runs the trit rule itself in integer arithmetic: 243-step rotation
table, Q16.16, `arc(...,0,7)` for a full turn, integer-percent alpha.
0 network requests at runtime, 0 console errors.
