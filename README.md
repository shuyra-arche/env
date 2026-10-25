# env

`env` 是 shuyra 的 `no_std` 环境调用 ABI：U 态任务与 S 态域任务共用的调用号与载荷
编解码、线类型、发起骨架、门闩权限，以及名字、账本条目和标记冲突检查。它不包含
服务目录协议，也不依赖 `std` 或具体的域实现。

## 独立构建与测试

本目录是独立 Cargo 工作区。工具链由 `rust-toolchain.toml` 固定为
`nightly-2026-09-04`。

RISC-V 库构建：

```sh
cd env
cargo build --target riscv64gc-unknown-none-elf --locked
```

宿主测试：

```sh
cargo test --tests --target x86_64-unknown-linux-gnu --locked
```

依赖已缓存后，同一组命令可离线复验：

```sh
cargo build --target riscv64gc-unknown-none-elf --locked --offline
cargo test --tests --target x86_64-unknown-linux-gnu --locked --offline
```

`[lib]` 的 `test`、`bench`、`doctest` 保持为 `false`；这不跳过集成测试。
原有的 8 个测试文件共 40 个测试，`tests/shape.rs` 另加 1 个结构判据，实际运行
9 个测试目标、41 个测试。测试不涉及 QEMU。

依赖 `mold` 与 `wire` 固定为公开 Git 仓库的已验收修订；`syn` 只作为测试用
`dev-dependency`，供 `tests/shape.rs` 解析 `src/marks.rs` 的路径引用。
