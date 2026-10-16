#!/usr/bin/env python3
"""Offline/stdlib SHA-256 and embedded-image verification. No JSON/Node/HTTP."""
import base64
import hashlib
import pathlib
import re

HERE = pathlib.Path(__file__).resolve().parent
SOURCE_HASHES = {
    "Asolaria_GGUF_Constellation_Explorer_FIXED.html": "1c5646899c7da9860e986be44a0a92002c117e1fd8b82585db90b65859968fb0",
    "NEXT-FOUR-SEAT-MATHEMATICAL-CHALLENGES-20261016.md": "f07e17ebf0d9096bf550f2a445c68f6afa5aae0834bda642590277bad2564062",
}
EMBEDDED_IMAGE_HASHES = (
    "5475f875fc7e5ea2c375fe97f59fb0576856f2b767b7d19508bc6a6c46803bb7",
    "a982b9aa6e4c9add4e0f539a2503ad5bc679746108803b35a6561669c0e98e2b",
    "37186d3ad67b5be68432f2b5ad872a89a13ca09c9ab7f76c768a870f29b3fefb",
    "4a357b87a9c8f2bf335b5a90bdc6a8ae9bf25c1f70f6f8e5f60f0cc1bf91cad8",
)


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def verify_digest(data, expect, name):
    assert sha256(data) == expect, "SHA_MISMATCH " + name
    if data:
        changed = bytearray(data)
        changed[-1] ^= 1
        assert sha256(changed) != expect, "TAMPER_NOT_REJECTED " + name


def main():
    for file, expected in SOURCE_HASHES.items():
        raw = (HERE / file).read_bytes()
        verify_digest(raw, expected, file)
        sidecar = (HERE / (file + ".sha256")).read_text().strip().split()
        assert sidecar[:2] == [expected, file], "SIDECAR_MISMATCH " + file
        print("SOURCE_SHA256_PASS|file=" + file + "|bytes=" + str(len(raw)) + "|json=0")
    for name in ("CONSTELLATION-3D-TERNARY-ADDRESS-20261016.hbi", "CONSTELLATION-REHOME-20261016.hbp"):
        raw = (HERE / name).read_bytes()
        seal = (HERE / (name + ".sha256")).read_text().split()[0]
        verify_digest(raw, seal, name)
        print("RECEIPT_SHA256_PASS|file=" + name + "|json=0")
    html = (HERE / "Asolaria_GGUF_Constellation_Explorer_FIXED.html").read_text()
    assert "JSON.parse" not in html.split("<script>")[-1] and "require(" not in html.split("<script>")[-1]
    assert "initializeEmbeddedImages();" in html and "new DataView(" in html
    chunks = re.findall(r"data:image/(?:jpeg|png|webp);base64,([A-Za-z0-9+/=]+)", html)
    assert len(chunks) == 4, "SOURCE_IMAGE_COUNT_CHANGED"
    for i, chunk in enumerate(chunks):
        raw = base64.b64decode(chunk, validate=True)
        verify_digest(raw, EMBEDDED_IMAGE_HASHES[i], "embedded-image-" + str(i))
        if i == 0:
            assert raw.startswith(bytes.fromhex("ffd8ff")), "INVALID_JPEG"
        else:
            assert raw.startswith(bytes.fromhex("89504e470d0a1a0a")), "INVALID_PNG"
        print("EMBEDDED_IMAGE_PASS|ordinal=" + str(i) + "|bytes=" + str(len(raw)) + "|json=0")
    print("VERDICT|original_source_hashes=PASS|embedded_image_hashes=PASS|mutated_evidence=REJECT|json=0")


if __name__ == "__main__":
    main()
