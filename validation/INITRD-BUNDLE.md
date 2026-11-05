# Versioned initrd bundle

The `VNI1` bundle preserves the existing eight-byte kernel capsule preamble.
It contains Unit records and named image/resource bytes. Unit IDs and Cargo bin
names are independent; several Unit declarations can reference the same image.
The kernel need not interpret Unit policy. There is no serialized Deployment.

Readers validate bounds, version, reserved fields, record counts, names, duplicate
identities, complete record consumption, zero alignment padding and the complete
boot capsule before exposing any records. Input strings and file bytes are
borrowed; validation allocates nothing. Host packing constructs the complete
bundle before publication.

On `nightly-2026-10-09`, four new Rust tests passed, covering borrowed roundtrip,
every truncated prefix, appended bytes, corrupted headers/records/padding,
duplicates and invalid resource boot entries. No QEMU was run. Strict Clippy
also reports existing diagnostics in unrelated ABI/ecall code; this is not a
claim that the complete repository passes Clippy.
