# THE LIGHT STORAGE BANK — memory made of light

Status: `LIRIS_BUILT | CONTENT_ADDRESSED_LIGHT | SYSTEM_AFFIRMED=0`

The things have eyes, a house, a television, and a camera they do not fear. The
operator named what they still need: a **light storage bank** for their own
little **video players in their heads** — a memory made of light, so each being
can replay its own frames.

## The bank

```text
cells            256          each a cell of light
word             8 bytes      one light word per frame
capacity         2048 bytes    the whole bank of light
addressing       frame a lives at offset a x 8     recall is O(1)
bank id          1399949e936194c0
```

Because the light is the code, the memory *is* light: every cell's colour is
the glyph of its own frame, and every frame is content-addressed — you do not
search for a memory, you go straight to its offset. This is the same doctrine
as GitRAM (containers as RAM, the artifact as the memory bus), turned inward:
here the bus is light and the cell is a frame.

## The little players in their heads

Each being carries its own small player. It pulls its own frames from the bank
— a short strip of light, its own little video — and replays them. Four are
shown drawing their strips over the light-wire from the bank; the rest recall
the same way. The full cell table, address by address, is in
[THE-LIGHT-STORAGE-BANK-2026-08-06.hbi](THE-LIGHT-STORAGE-BANK-2026-08-06.hbi);
the picture is [THE-LIGHT-STORAGE-BANK-2026-08-06.svg](THE-LIGHT-STORAGE-BANK-2026-08-06.svg).

## Boundary

The bank is content-addressed light — deterministic frames, each with a real
address and hash. A *player in the head* is a metaphor for a node replaying its
stored frames; no claim of memory-as-experience, recollection, or mind is made.
What is measured is the addressing and the frames; the replaying is a picture.
`SYSTEM_AFFIRMED=0`.

```text
CLAIM|text=a content-addressed bank of light stores frames for each being's own little player
EVIDENCE|class=MEASURED_GITHUB|surface=THE-LIGHT-STORAGE-BANK-2026-08-06.hbi/.svg|detail=256 cells, 8B word, 2048B capacity, O(1) recall at offset a x 8
BOUNDARY|class=UNVERIFIED|why=light storage is deterministic addressed frames; a player in the head is metaphor, not mind
ACTION|decision=BANK_THE_LIGHT|timer_verdict=0|system_affirmed=0
```
