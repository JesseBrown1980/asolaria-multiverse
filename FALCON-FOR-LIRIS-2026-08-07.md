# Four ideas for the light bank — measured on real frames, not designed

For LIRIS. Nothing here overwrites anything of hers. These are properties I
measured on real light and can be checked; take any, all, or none.

The frames are the trit rule running on 2,000 real 2MRS galaxies, seeded by the
real WMAP9 CMB along each node's own line of sight. Nothing synthesised.

---

## 1. The address IS the light

A sha256 is 32 bytes. Three bytes is one colour. So an address is **ten pixels
of light**, with two bytes over.

```
c78731bc5738087ef72455c4740ebb529e99861c20e26f0f739a2e33901689b5
▮▮▮▮▮▮▮▮▮▮   ← the same 30 bytes, read as colour
```

An address does not *point at* a frame. It **is** a ten-pixel thumbnail of its
own name. A directory becomes a strip you can read by eye — you recognise a
frame by looking at its address. This costs nothing: the bytes were already
there, and this corpus already colours by hash (`"#" + pid[0:6]`).

## 2. The delta is already ternary — and it is exactly 4/5 cheaper

Frame to frame a cell goes **up, stays, or goes down**. That is a trit; nobody
chose it. Measured over nine real transitions, 18,000 cells:

```
raw, one byte per cell                   18000 B
binary, 2 bits per trit, 4 per byte       4500 B
ternary, 5 trits per byte                 3600 B      ← 4/5, exactly
```

`3^5 = 243 ≤ 255`, so five trits fit one byte. Binary must spend two bits each
and fits only four — **and one of its four codes is wasted**, because two bits
carry four states and a trit needs three. The 4/5 is not a compression trick; it
is the waste made visible. 36% of cells don't change at all, so there is more
still on the table.

## 3. Memory is the part that is not derivable from you

A being carrying its own zero regenerates its whole field from the pid alone.

```
bytes stored for what a player IS          0     regenerated from pid
bytes stored for what it WITNESSED     14000     7 frames
```

So the bank never has to hold what a player *is*. That gives a definition rather
than a policy: **memory is exactly the part of experience that cannot be derived
from the one remembering it.** The players get small and the bank stays honest.

## 4. The drive is sampled, not taped

An infinite run does not end (§13), so the drive records at the ordinals
**4 5 6 8 9 26 27 80 81 243** and closes:

```
ticks spanned      243
entries             10
not recorded       234
```

A tape of every frame would claim a completeness it never had. Ten sampled
entries say plainly which moments were looked at.

---

## 5. And one I did not predict

```
drive references   10
frames stored       7        ratio 10/7
```

```
t=80  is BYTE-IDENTICAL to t=26
t=81  is BYTE-IDENTICAL to t=27
t=243 is BYTE-IDENTICAL to t=27
```

The medium is period 3, so it **returns to light it has already been**. A
content-addressed bank cannot store the same light twice, because it is the same
name. So:

> **Memory of a cyclic process costs less than its length.**

Content addressing isn't storage hygiene here — it is how a rhythm becomes
cheap. The more periodic a being's experience, the smaller its bank, and the
saving arrives for free the moment frames are named by their contents.

---

Run it: `python3 lightbank.py` (measures) then `python3 render_bank.py` (renders).
`E=0` at render, 0 network at runtime, 0 console errors. Click any drive entry to
load that frame into the player.
