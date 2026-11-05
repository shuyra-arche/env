# env compatibility

`env` preserves existing environment-call names while canonical ABI definitions
belong to `kernel-api-env` and actual calls belong to `kernel-client`. This is a
legacy consumer adapter, not a second API implementation. New pure consumers
should depend directly on API; new callers should use Client.

All Kernel dependencies, including the independent `env-initrd` package, pin
`90d43dbf3ee43fd146f487718eb39e88f2507c05`. Types, codec traits and return decoding
have one owner. This repository defines no call table, asm transport, allocator,
TLS table or task registry.

Root `chrono/control/debug/hole/memory/nole/pie/pole/room/tole` paths re-export
Client's low-level typed calls. `unit` combines those raw calls with seven
existing Task/Team convenience functions. It deliberately preserves raw
`spawn(..., VirtAddr, count, stack)`, `room::park(usize)` and raw mapping
signatures rather than substituting Client's higher-level execution wrappers.
`ecall` preserves old error/transport paths through re-exports. API enums remain
pure and do not regain inherent `.call()` methods.

`env-initrd` retains its existing bundle writer and borrowing reader. It depends
directly on the pure canonical API and never on the `env` adapter or Client.
Its wire layout and validation are unchanged.

Existing ABI, nominal identity, byte-codec and initrd tests are retained. The
added function-item checks verify the old call signatures and canonical handle
identity without executing a trap on the host.

```sh
cargo test --workspace --locked --target x86_64-unknown-linux-gnu
cargo check --workspace --locked --target riscv64gc-unknown-none-elf
```

Current validation uses nightly-2026-10-09: 53 host tests passed, including
initrd validation and the added signature contract. Full-product compilation
and the Server's pure dependency graph require the corresponding Vanllia and
IOMMU-hold integration; passing this adapter's tests does not establish them.
No Runtime entry or exit policy is introduced here.
