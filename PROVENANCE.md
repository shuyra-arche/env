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
