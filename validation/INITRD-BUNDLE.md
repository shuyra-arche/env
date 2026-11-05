# Versioned initrd bundle

The independent `env-initrd` package owns the Unit bundle implementation under
`initrd/src/ledger`. The root `env` package keeps its canonical Kernel facade
unchanged. The capsule reader is directly re-exported from `kernel-api-env` at
the same exact Kernel revision as the facade; deleted legacy ABI/ledger source
files are not restored. No second capsule implementation is compiled.

The `VNI1` bundle preserves the existing eight-byte kernel capsule preamble.
It contains Unit records and named image/resource bytes. Unit IDs and Cargo bin
names are independent; several Unit declarations can reference the same image.
The kernel need not interpret Unit policy. There is no serialized Deployment.

Readers validate bounds, version, reserved fields, record counts, names, duplicate
identities, complete record consumption, zero alignment padding and the complete
boot capsule before exposing any records. Input strings and file bytes are
borrowed; validation allocates nothing. Host packing constructs the complete
bundle before publication.

On `nightly-2026-10-09`, five bundle Rust tests passed, covering borrowed roundtrip,
every truncated prefix, appended bytes, corrupted headers/records/padding,
duplicates, invalid resource boot entries and capsule type identity with the
canonical Kernel owner. All 52 workspace host tests and the RV64 workspace check
passed, including the facade's existing identity/frame/wire checks. Strict
Clippy passed for `env-initrd` and the bundle/identity/frame/wire-code tests.
Whole-workspace strict Clippy reports two existing constant-chunk diagnostics in
`tests/memory.rs`; those retained tests were not rewritten. Host results alone
are not a runtime acceptance claim.
