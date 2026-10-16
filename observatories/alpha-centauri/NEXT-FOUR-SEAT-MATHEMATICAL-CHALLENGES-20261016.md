# Asolaria Alpha Centauri — Four-seat mathematical discovery roadmap
**Date:** 2026-10-16. Independent research roadmap, local execution; no GitHub modifications, paid jobs, astronomical discovery, physical photon-memory claim, or GGUF neural inference.

## Verified starting point
- Webb/ESA 5037×1800 3-channel source is a **public processed JPEG collage**, not original JWST calibrated detector FITS and not three independent physical bands.
- Three 243×243 Hubble/WFC3 F555W science ROIs are original FITS float32 *pixel bits* from **Proxima** rather than α Cen A.
- GGUF v3 image-data bank includes 168 Webb supertiles and 243 Hubble 27×27 tiles = 411 tensor records, all 411 byte-exact independently reconstructed in earlier audit.
- Fine 27×27 tessellation contains ceil(5037/27)*ceil(1800/27) + 3×9×9 = 12,529 + 243 = 12,772 logical spatial leaves. This is an address count, not 12,772 independently photographed celestial objects.
- Fixed-length addressing capacity: 411 tensor records need at least ceil(log2 411)=9 binary bits or ceil(log3 411)=6 trits. 12,772 fine leaves need 14 bits or 9 trits. Packed full ternary alphabet costs ceil(6log2 3)=10 and ceil(9log2 3)=15 bits, respectively; use constrained-code mapping if reducing the actual stored tag cost.

## Fresh executed, strictly lossless compression baseline
See `CONSTELLATION-NEW-MATH-BASELINE-20261016.hbp` and `math_pilot_20261016.py` for full source SHA, exact replay, and per-image measurements. For the decoded published Webb RGB 27,199,800-byte buffer, zlib(9) produces 6,561,651 B. A reversible modulo-256 horizontal predictor plus zlib(9) produces 5,086,283 B (**22.485% fewer bytes than the ordinary lossless zlib baseline**). Every byte is recovered; this is *not* 22.5% better than the original compressed JPEG. Hubble F32 byte predictor slightly worsens compression, so adaptive choice is essential. A true archive should also budget per-tile headers, random access, metadata, and time to decode. Exactly 542 repeated 27-pixel tiles were byte-identical within the processed Webb collage, not evidence of identical astronomy across independent frames. SHA matches by themselves do not authenticate an original telescope operator.

## Seat 1 — detector-science pipeline
Retrieve MAST **original JWST/MIRI F1550C** associated science and reference-star products for Alpha Centauri A: 2024 program 1618, 2025 programs 6797 and 9252. Preserve raw downloadable bytes, proposal/visit IDs, pipeline calibration versions, WCS and timestamps, independently hash originals. Run STScI JWST calibration and point-source rejection, not JPEG contrast enhancement. Result: calibrated images, PSF-subtracted residuals, candidate sky coordinate and flux with uncertainty; original source custody.
**Falsifier:** candidate S1 disappears under independent instrument calibration / alternate reference PSFs or is statistically indistinguishable from noise after search trials.

## Seat 2 — authenticated tile-to-archive provenance
Create a strict chain FITS byte root → FITS plane/exposure coordinates → derived tile SHA → GGUF tensor → browser rendered pixel. Bind each tile to a specific original source, detector plane, filter and epoch, and require archive metadata/checksum. Independently replace both pixel and owner-controlled manifest: do not claim origin authenticated based on a coherent owner-authored SHA. Distinguish STScI institutional provenance from hypothetical detached instrument-side Ed25519 signatures.
**Falsifier:** a distinct pixel or untrusted signer can masquerade as another original detector source without a verification HOLD.

## Seat 3 — genuinely new mathematical conjectures
A) Prove minimum source-aware mixed-radix address cost with variable tile sizes and active sensor planes, report bits/trits including metadata and selectors. Existing Rainbow27 PR #90 3-bit witness works *when normalized P class is already known*; do not call 3+3 bits better than fixed 5-bit ordered source ID.
B) Find a reversible data-conditioned predictor (e.g. bitplane or neighbor residual) for FITS float32 science values that beats raw zlib(9) on *independent* held-out exposures without changing their 32-bit patterns; establish exact invertibility algebraically and via independently implemented all-byte replay.
C) Derive null-space/identifiability limits of high-contrast deconvolution: for telescope data y=Hf+n, changes in any f direction in kernel(H) cannot be inferred from y alone. Multi-epoch, differing orientations, and truly distinct wavelengths can shrink this null space; interpolated RGB cannot.
**Falsifiers:** decoder collisions, holdout codec regression, counterexample rank deficiency or nonzero source mismatch.

## Seat 4 — blind astronomy and hostile controls
Inject false synthetic 15.5µm planets into *calibrated original* JWST epoch-negative science frames at preregistered fluxes and sky separations; keep blind position list and recovered false positives, non-detections, completeness and uncertainty maps. Alternate reference-star libraries and PSF-subtraction methods (RDI, KLIP, etc.); carry observational masks, correlated-noise significance and trial corrections. Compare against independent published detection/nondetections; never claim image-based confirmation based on a processed press mosaic.
**Falsifier:** algorithm confuses PSF speckles, interpolation enhancements or damaged provenance for an astronomical companion.

## Coordinator acceptance
Require two independently implemented exact-source reconstructions and hostile packet changes, source and derived artifact SHA seals, same-input baseline including no-signal controls, time/performance/memory accounting, genuine held-out science observations, and a precise distinction between software correctness, observed astrophysical signal and instrument/source custody. Preserve failed experiments, no auto-merge, external release or paid jobs. Existing 2.062 m/s RV score is frozen and pertains to Proxima—not Alpha Centauri A.

## External reference
JWST Alpha Cen A S1 Papers I/II (2025 ApJ 989 L22 and L23) report a 2024 August ~3.5mJy candidate at 1.5arcsec and two 2025 nondetections, which must be considered jointly. A candidate is not a confirmed exoplanet.
