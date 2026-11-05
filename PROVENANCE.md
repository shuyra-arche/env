# Provenance

## Source

- Repository: <https://github.com/anreonyr/sqware>
- Commit: `fb4be8eb5cf607fccef3aa4d696f1a24701edb34`
- Branch at extraction: `codex/shuyra-env-extraction`
- Extracted path: `crates/env`

The source commit has 71 tracked files under `crates/env`: 62 Rust files under
`src/`, 8 Rust test files under `tests/`, and `Cargo.toml`. All 62 `src/*.rs`
files and all 8 original `tests/*.rs` files are byte-for-byte copies of that
commit. The ABI, marks, authorization, and codec sources were not edited.

The standalone root `Cargo.toml` is derived from the upstream manifest. It
replaces inherited workspace fields with explicit `version = "0.1.0"` and
`edition = "2024"`, declares `[workspace]` with `resolver = "3"`, preserves
`[lib] test = false`, `bench = false`, and `doctest = false`, and keeps the
upstream `bitflags`, `erra`, `mold`, and `wire` requirements. `mold` remains at
rev `f52501f4c91b686eded8b4bcd743b4e96084b7f9`; `wire` remains at rev
`0ff77dc4573fa2620d2fe995a18c01949d9c7e57`. The only added dependency is the
test-only `syn = "2"` with `full` and `visit`, resolved to the existing
`2.0.119`. `rust-toolchain.toml` is copied unchanged from the source root.

## File mapping

| Upstream | Here |
| --- | --- |
| `crates/env/src/**/*.rs` (62 files) | `src/**/*.rs` |
| `crates/env/tests/manifest.rs` | `tests/manifest.rs` |
| `crates/env/tests/memory.rs` | `tests/memory.rs` |
| `crates/env/tests/nole.rs` | `tests/nole.rs` |
| `crates/env/tests/resource_domains.rs` | `tests/resource_domains.rs` |
| `crates/env/tests/resources.rs` | `tests/resources.rs` |
| `crates/env/tests/signals.rs` | `tests/signals.rs` |
| `crates/env/tests/tole.rs` | `tests/tole.rs` |
| `crates/env/tests/unit.rs` | `tests/unit.rs` |
| `crates/env/Cargo.toml` | `Cargo.toml` (standalone rewrite) |
| source-root `rust-toolchain.toml` | `rust-toolchain.toml` |
| source-root `Cargo.lock` | `Cargo.lock` (pruned copy, see below) |

The 8 original test files contain 40 tests:

| Test file | Tests |
| --- | ---: |
| `manifest.rs` | 4 |
| `memory.rs` | 15 |
| `nole.rs` | 1 |
| `resource_domains.rs` | 4 |
| `resources.rs` | 4 |
| `signals.rs` | 3 |
| `tole.rs` | 3 |
| `unit.rs` | 6 |

## Shape test

`tests/shape.rs` is new. Its `References` helper copies lines 598-617 of
`programs/tests/system-shape/lib.rs` at the source commit exactly in content,
de-indented from the source module wrapper. It includes the `visit_path`
implementation and the `visit_item_use` handling of
`UseTree::Path`, `Group`, `Name`, `Rename`, and `Glob`. It does not copy the
recursive directory `references` helper from that file.

The test `environment_marks_have_no_domain_dependencies` reads this
repository's `src/marks.rs`, parses it with `syn::parse_file`, visits it with
`References`, and applies the original four `starts_with` predicates unchanged:

```text
crate::system
crate::driver
crate::service
protocol::
```

The assertion is the environment half of the source test
`execution_mechanism_and_environment_marks_have_no_domain_dependencies` at
`programs/tests/system-shape/lib.rs:675`. That merged source test is not removed
at this checkpoint; removing its env half and renaming its remaining schedule
half belong to the consumer stage after this provider is published. This
extraction therefore does not claim that the source test has already been fully
migrated.

## Lock file

The upstream commit does not contain an independent lock file inside
`crates/env`. Its only lock file is the source-root `Cargo.lock`, which is
tracked in commit `fb4be8e` and matched the source worktree byte for byte when
this snapshot was taken. The candidate `Cargo.lock` was copied from that
source-root file and then pruned with an actual offline `cargo metadata` run to
the standalone `env` package graph. It is generated candidate evidence, not an
original `crates/env` lock and not claimed to have existed in the source commit.

The pruned lock has 9 packages and keeps the accepted registry versions already
present in the source-root lock: `bitflags 2.13.1`, `erra 0.2.0`,
`proc-macro2 1.0.107`, `quote 1.0.47`, `syn 2.0.119`, and
`unicode-ident 1.0.24`. `mold` and `wire` retain the exact Git revisions above.
No dependency version was upgraded for this extraction. The candidate
`.gitignore` ignores only `/target`, so this generated lock is intended to be
tracked.

## Attribution

The source commit author and committer is:

```text
anreonyr <anreonyr@gmail.com>
```

Author identities observed in the real source Git history include:

```text
Serial Device <anreonyr@gmail.com>
anreonyr <88045926+anreonyr@users.noreply.github.com>
anreonyr <anreonyr@gmail.com>
```

The original repository history remains at the source URL above. This
extraction is a snapshot without a rewritten or synthesized history.

## Licensing

The source project declares no license. At commit
`fb4be8eb5cf607fccef3aa4d696f1a24701edb34`, no tracked `LICENSE`, `COPYING`, or
`NOTICE` file matched the project. No license text or license metadata is added
by this extraction; do not assume a license that the source does not grant.

## Adapter stage: re-export-only facade over canonical Kernel

Everything above this section describes the **extraction stage** and is kept as
history. It is not a description of the current tree: the 62-file source count,
the `tests/shape.rs` section, and the extraction-stage `Cargo.lock` description
above no longer describe this tree (the source files are gone and the lock was
regenerated, see below).

This stage turns the standalone extraction into a pure compatibility adapter.
The authoritative definitions now live in the Kernel repository
<https://github.com/shuyra-arche/kernel.git> at full commit
`a9986c82127cf25183bf5c30e7d42e4a9376b734`. The environment ABI is owned by
`api/env` (package `kernel-api-env`) and re-exported by the umbrella `api`
(`kernel-api`, as `kernel_api::env`). `api/wire` (package `kernel-api-wire`)
owns only the byte codec (`Field` / `Span` / tail helpers / `OK`) and is
depended upon by `api/env`; it owns no environment definition and no handle such
as `TaskId`.

- `src/lib.rs` is now the entire source tree: `#![no_std]` plus
  `pub use kernel_api_env::*;`, with a doc comment stating the dependency
  direction. All 61 other `src/**/*.rs` files of the extraction stage — the ABI
  call tables, `wire`, `ledger`, `marks`, `ecall`, permission and wait
  vocabulary — were deleted. This adapter owns no runtime definitions.
- `Cargo.toml` keeps the standalone fields (`name = "env"`,
  `version = "0.1.0"`, `edition = "2024"`, `[workspace] resolver = "3"`, and
  `[lib] test = false`, `bench = false`, `doctest = false`). The runtime
  `bitflags`, `erra`, `mold`, and `wire` requirements were replaced by a single
  `kernel-api-env` git dependency pinned to the full Kernel SHA above; the
  `syn` dev-dependency was removed. `kernel-api` and `kernel-api-wire`, both
  pinned to the same full SHA, are dev-only: `kernel-api` is used by
  `tests/identity.rs`; `kernel-api-wire` is used there and by
  `tests/wire_codes.rs`. They add no direct runtime edge. The old `mold`
  rev `f52501f4c91b686eded8b4bcd743b4e96084b7f9` and `wire` rev
  `0ff77dc4573fa2620d2fe995a18c01949d9c7e57` are no longer referenced here.
- The eight original ABI test files (`manifest`, `memory`, `nole`,
  `resource_domains`, `resources`, `signals`, `tole`, `unit`; 40 tests) are
  byte-for-byte unchanged. They reach everything through `env::...` paths, which
  the facade resolves to the owner types.
- `tests/shape.rs` was removed. Its guard is now **substantively equal**, not
  byte-identical, to the one in the canonical Kernel checkout at
  `kernel/api/env/tests/shape.rs`: the same `References` visitor body (including
  the `visit_item_use` `Path` / `Group` / `Name` / `Rename` / `Glob` handling),
  the same four `starts_with` predicates (`crate::system`, `crate::driver`,
  `crate::service`, `protocol::`), and the same "no match" assertion over that
  package's `src/marks.rs`. The canonical file additionally carries
  `use kernel_api_env as env;` and scopes its imports inside the test function,
  so the two files are not byte-identical. Root verified the guard semantics and
  ran it there. This adapter keeps no empty, skipped, or fallback shape check.
- `tests/identity.rs` is new. It proves, in both directions and through
  `core::any::TypeId`, that facade `env::{Permission, MemoryCall, TaskId}` are
  the canonical `kernel_api_env` types and the `kernel_api::env` umbrella types;
  that `env::wire::{Field, Span}` transfer generic bounds with
  `kernel_api_wire::{Field, Span}`; and that `TaskId` / `Mark` store and fetch
  through the canonical `Wire` trait and the facade. `TaskId` is a
  `kernel-api-env` handle, not a `kernel-api-wire` item, so the test never names
  `kernel_api_wire::TaskId`.
- `tests/frame.rs` and `tests/wire_codes.rs` are new and add no dependency: the
  first derives `env::Frame` on a struct and asserts real `Span` store/fetch
  bytes through the derive's default `::env::wire` path; the second derives
  `env::WireCodes` on an enum, imports `kernel_api_wire::OK` (the derive takes
  `OK` from its call site), and asserts the code table plus
  `fail_to_code` / `code_to_fail`. Together they prove the historical `env`
  derive re-exports survive the glob facade.
- `Cargo.lock` was regenerated by Root for this candidate graph and is committed
  with it: the only sources are the public Kernel repository at the full SHA
  above (no local paths), no original `mold` or `wire` source remains in the
  graph, and the previously accepted registry versions (`bitflags 2.13.1`,
  `erra 0.2.0`, `proc-macro2 1.0.107`, `quote 1.0.47`, `syn 2.0.119`,
  `unicode-ident 1.0.24`) are preserved. This worker ran no Cargo command and
  generated no lock.
- Root validation: 47 locked host tests passed (40 in the eight original ABI
  files, 5 in `tests/identity.rs`, and one each in `tests/frame.rs` and
  `tests/wire_codes.rs`), across 11 targets; the locked RISC-V `--lib` build passed;
  `cargo metadata` shows
  the four Kernel API packages (`kernel-api`, `kernel-api-env`,
  `kernel-api-wire`, `kernel-api-macros`) at the exact full SHA above and no old
  owner; the eight original ABI test files were byte-compared and are unchanged.
- Final validation uses the unchanged declared `nightly-2026-09-04`, installed
  in an isolated task toolchain directory: `rustc 1.100.0-nightly
  (a69a63265 2026-09-03)`. The earlier first run used the environment-supplied
  `nightly-2026-10-09`; the declared-toolchain evidence gap is now closed.
- The owner packages are public in the Kernel repository at that full SHA and
  are consumed as git dependencies there; no crates.io release was attempted,
  so no registry publishing gate is pending.

Old owner definitions — `crates/env` in the source repository above, the
original `wire` repository, and the original `mold` repository — are superseded
by the canonical Kernel packages. They must not coexist with them in one build
graph: two nominal `env` or `wire` definitions break type identity.
