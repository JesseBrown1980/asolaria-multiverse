# Alpha Centauri constellation · Public Multiverse Observatory

**Source-derived, immutable publication.** This is a new isolated home inside the public [Asolaria Multiverse](https://github.com/JesseBrown1980/asolaria-multiverse), not a replacement for its existing `index.html` and not a copy of the private `Asolaria` root.

[▶ Open the constellation viewer](Asolaria_GGUF_Constellation_Explorer_FIXED.html) · [Read the four-seat mathematical challenge roadmap](NEXT-FOUR-SEAT-MATHEMATICAL-CHALLENGES-20261016.md).

## Exactly what is published

The uploaded browser viewer is preserved **byte-for-byte**, SHA-256 `1c5646899c7da9860e986be44a0a92002c117e1fd8b82585db90b65859968fb0`. It contains the 5037×1800 **processed ESA/JWST publication mosaic** (not a JWST detector FITS exposure), plus three **contrast-preview** images from separate 2012/2014/2015 Hubble observations of **Proxima**. It runs using built-in browser JavaScript/canvas, with *no Node.js runtime, npm package, build step, JSON data interchange, external download, or API token*; the published images appear immediately. The optional local GGUF file selector can read the 411 tensor tiles **only when the user supplies the separately generated `.gguf` file**. The GGUF bytes and original science FITS were not among the two uploads and are **not included here**.

The uploaded research roadmap is also copied **byte-for-byte** and lists the remaining detector-science and independent validation work. Its reported 22.485% coding improvement is over zlib compression of *decoded* Webb JPEG pixels, not over the source JPEG.

## HBI / HBP / SHA / SH / HASH · 3-D ternary address contract

`CONSTELLATION-3D-TERNARY-ADDRESS-20261016.hbi` defines a **logical** 3-axis address:

- `x`: 5 balanced trits, for a column up to 186 (187 columns)
- `y`: 4 balanced trits, for a row up to 66 (67 rows)
- `z`: 2 balanced trits, for the *observation/source layer*: Webb mosaic or one of three Hubble epochs

Every source-plane tile has a unique `(x,y,z)` integer-only address. This fixed structured address takes **11 trits**; a globally optimal unrestricted identifier for 12,772 leaves needs **9 trits**. The extra trits preserve spatial coordinates. `z` is **not** astronomical depth, and the browser visualization is still two-dimensional. A 3D physical reconstruction is a future, falsifiable research task.

`CONSTELLATION-REHOME-20261016.hbp` records source-byte roots, each embedded image hash, and scientific boundaries. Every `.hbi` and `.hbp`, plus the two original uploads, gets a conventional `.sha256` sidecar. `SH` and `HASH` are named routing/decision concepts here, not a new cryptographic primitive; `DH` remains **unimplemented** and cannot be treated as an authenticated exchange.

## Compute and chariot cross-links

- [Asolaria Stone to Moss](https://github.com/JesseBrown1980/asolaria-stone-to-moss): existing GGUF verification, four-seat MIRI/JWST data provenance, rainbow transport and chariot/egg workflow. It is a separate compute backend, not silently merged into this public viewer.
- [asolaria-tribit](https://github.com/JesseBrown1980/asolaria-tribit): canonical no-std, zero-dependency Rust 1.81 balanced-ternary kernel. Our `src/lib.rs` is a **small independent address correctness checker**, not a replacement for the tribit kernel.
- [3I/ATLAS Hugging Face receipts](https://huggingface.co/datasets/Jessedbrown/3I-ATLAS-receipts): source-hash/receipt predecessor, not a proof that GGUF or photonic memory is a measured 3I astronomical mechanism.

This folder has its own Python 3 stdlib original-image SHA verification and Rust 1.81 + Clippy integer-only address tests, with a GitHub Actions workflow **beside** the existing Pages door (not gating deployment).

## Scientific falsifiers

The unconfirmed **S1** source in the Webb *processed* MIRI image cannot be assigned a planetary surface or confirmed status by zooming. The next experiment needs original JWST MIRI program **1618 (2024)**, **6797 (2025)** and **9252 (2025)** original FITS, reference-star PSF subtraction, blind injected companions, proper flux uncertainties and nondetection limits. Four GitHub jobs verifying data transport do not by themselves establish that science.

Credits: **NASA/ESA/CSA/STScI; ESA/Webb** (source processed composite). See the original image credit metadata embedded in the uploaded JPEG and <https://esawebb.org/images/weic2515c/>. The viewer is a source-preserving research demonstration; no new astronomical object is claimed.
