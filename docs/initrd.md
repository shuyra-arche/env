# Initrd bundle contract

`env-initrd` owns the Unit bundle implementation in `initrd/src/ledger`.
The capsule reader is re-exported from `kernel-api-env` at the same Kernel
revision as the adapter; there is one capsule implementation.

The `VNI1` bundle retains the eight-byte kernel capsule preamble and contains
Unit records and named image/resource bytes. Unit IDs and Cargo bin names are
independent; multiple Unit declarations may reference one image. There is no
serialized Deployment, and Kernel does not interpret Unit policy.

Before exposing records, readers validate bounds, version, reserved fields,
counts, names, duplicate identities, full record consumption, zero alignment
padding and the complete boot capsule. Strings and file bytes borrow the
input; validation allocates nothing. Host packing completes the whole bundle
before publication. The bundle and identity tests under `tests/` exercise this
contract; native boot acceptance belongs to the product consumer.
