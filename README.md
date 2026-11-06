# env

Shuyra 旧环境调用路径的 `no_std` 兼容库。纯 ABI 定义来自 `kernel-api-env`，
实际调用委托 `kernel-client`；新代码分别直接使用 Kernel API 和 Client。

- `src/lib.rs`、`src/ecall.rs`、`src/unit.rs`：兼容重导出与 Task/Team 便捷函数。
- `initrd/`：独立 `env-initrd` 包，提供 bundle 编写与借用读取。
- `tests/`：ABI、类型身份、字节编解码与兼容签名检查。

本仓不持有调用表、汇编传输或任务运行状态。兼容入口保持原有 raw 参数形态；
API 枚举只负责载荷，不能通过旧 `.call()` 方法执行系统调用。
`env-initrd` 只依赖纯 API，bundle 格式及验证要求见 [initrd 协议](docs/initrd.md)。

## 工具链与维护

使用 `rust-toolchain.toml` 固定的 `nightly-2026-09-04`，首次运行需要 Rustup
下载工具链及 RV64 目标。依赖版本以 Cargo 清单和锁文件为准。
下列命令从仓库根目录执行；宿主测试不代表真实内核运行验证。

```sh
cargo test --workspace --locked --target x86_64-unknown-linux-gnu
cargo check --workspace --locked --target riscv64gc-unknown-none-elf
```

宿主签名检查不执行陷入；产品编译与启动验证由 Vanllia 的对应组合承担。

## 贡献与来源

维护与 PR 规则见 [Shuyra 贡献规范](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md)。
来源、作者与许可情况见 [PROVENANCE.md](PROVENANCE.md)。
