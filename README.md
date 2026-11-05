# env

`env` 是 shuyra `env` 面的**兼容适配器（facade）**：该包**一个定义都不写**——
不写调用表、不写线类型、不写 codec、不写权限位、不写 marks、不写名字与账本条目。
该包源码只有 `pub use kernel_api_env::*;`，把 Kernel 的权威包 `kernel-api-env`
原样转出。

两类名字要分开看，两条判据都在 `tests/identity.rs`：

- **nominal 类型**：`env::Permission`、`env::MemoryCall`、`env::TaskId`、`env::Mark`
  **就是** `api/env`（包 `kernel-api-env`）定义的那几枚类型，经伞包 `api`
  （`kernel_api::env`）与 facade 原样转出——三条路径上是**同一枚 nominal 类型**
  （双向赋值 + `TypeId` 相等）。
- **共享的 wire trait**：`env::wire::{Field, Span}` 是 `api/wire`（包
  `kernel-api-wire`）定义的字节 codec trait，由 `api/env` 转出——`env::wire::Field`
  与 `kernel_api_wire::Field` 是**同一条 trait**（泛型约束双向互传）。

口径与方向：`api/wire` 只拥有**字节 codec**（`Field` / `Span` / 尾部助手 / `OK`），
**不拥有** `TaskId` 或任何句柄；句柄（`TaskId` / `Mark` / `PieToken` …）是 `api/env`
的定义。依赖只有 `api/env` → `api/wire` 一条边，反向没有。

`TaskId` / `Mark` 另有存/取判据：经权威 `api/env` 的 `Wire` 存、经 facade 的
`env::Wire` 取（同一条 trait，故两条路互喂）。

## 依赖方向

```text
env(facade) --runtime--> kernel-api-env -> kernel-api-wire, kernel-api-macros, bitflags, erra
env(facade) --dev only--> kernel-api(umbrella), kernel-api-wire   # identity / wire_codes 测试
```

运行时只有一条边，且指向 Kernel 权威包。`kernel-api` 与 `kernel-api-wire` 只作
dev-dependency，不进运行时图。

三个依赖都固定到 Kernel 仓 <https://github.com/shuyra-arche/kernel.git> 的**同一个
完整提交** `a9986c82127cf25183bf5c30e7d42e4a9376b734`（full SHA，不用短 SHA、不用
分支名）。旧定义仓（`sqware` 的 `crates/env`、旧 `wire`、旧 `mold`）**一条边都没
有**：一个构建图里出现两份 `env` 或两份 `wire` 的定义会破类型同一性，故新旧
owner **不得并存**、也不得用 `[patch]` 之类的手段把两边缝在一起。

工作区的独立 `env-initrd` 包拥有 `env_initrd::ledger::bundle`：Unit 二进制声明、
共享镜像和必要资源。`env` facade 的导出保持原样；胶囊读取器由
`kernel-api-env::ledger::capsule` 直接转出，不恢复旧 `src/ledger` 或复制 ABI。
initrd 前八字节保留内核的 offset/length 合同，不记录 Deployment 名称或启动清单。
读侧完整验证后才返回借用视图。

## 独立构建与测试

本目录是独立 Cargo 工作区。工具链由 `rust-toolchain.toml` 固定为
`nightly-2026-09-04`。Root 已隔离安装此版本并按锁复验，见下节"验证证据"。

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

`Cargo.lock` 已由 Root 按本清单的候选图重新生成并随候选提交：来源只有公开 Kernel
仓的固定 full SHA（不是本地路径），没有任何旧 `mold` / `wire` 源，并保留抽取期已
验收的 registry 版本（`bitflags 2.13.1`、`erra 0.2.0`、`proc-macro2 1.0.107`、
`quote 1.0.47`、`syn 2.0.119`、`unicode-ident 1.0.24`）。故上面带 `--locked` 的
命令按这份锁运行；失配时由重新生成来解，不要靠手改锁或改依赖版本绕过。

## 测试

8 个原有 ABI 测试文件逐字节保留（`manifest` 4、`memory` 15、`nole` 1、
`resource_domains` 4、`resources` 4、`signals` 3、`tole` 3、`unit` 6，共 40 个），
外加 `tests/identity.rs`（5 个同一性判据）与 facade 派生面的 `tests/frame.rs`、
`tests/wire_codes.rs`（各 1 个）：11 个测试目标、47 个测试，已在声明的工具链上通过。
测试不涉及 QEMU；宿主测试也不证明真实 trap / 系统调用路径。

## 形状判据的归宿

抽取期本仓另有一份 `tests/shape.rs`（1 个测试）：它读本地 `src/marks.rs`，用同一份
`References` visitor 套四条 `starts_with` 谓词（`crate::system` / `crate::driver` /
`crate::service` / `protocol::`）。适配器不再拥有任何源码，故该判据**迁回权威仓**：
`kernel/api/env/tests/shape.rs` 就是同一条判据的原样版本——同一份 visitor 实现
（含 `visit_item_use` 的 `Path`/`Group`/`Name`/`Rename`/`Glob` 处理）、同样四条谓词、
同样"一条都不许命中"的断言，读的是权威包的 `src/marks.rs`——**实质相同，不是逐字节
相同**：权威那一份另加了 `use kernel_api_env as env;` 并把 import 收进测试函数里。
Root 已核对这条判据的语义并在权威包上**跑过**它。

因此本仓删掉了 `tests/shape.rs` 与仅供它使用的 `syn` dev-dependency。本仓**不留**
空壳版本、跳过标记或回退式形状检查——那种检查只会看着仍在、实际不判任何东西。

## 验证证据（Root 跑，不是本 worker）

- 宿主测试：47 个测试通过（8 个原有 ABI 文件 40 个、`tests/identity.rs` 5 个、
  两个 facade derive 路径各 1 个），共 11 个目标，使用 `--locked`。
- RISC-V：声明工具链上的 `cargo build --lib --target riscv64gc-unknown-none-elf --locked` 通过。
- 依赖图：`cargo metadata` 显示四枚 Kernel API 包（`kernel-api`、`kernel-api-env`、
  `kernel-api-wire`、`kernel-api-macros`）都在 exact
  `a9986c82127cf25183bf5c30e7d42e4a9376b734`，且没有旧 owner。
- 原文保真：Root 逐字节比对 8 个原有 ABI 测试文件，未变。
- 锁：Root 重新生成 `Cargo.lock`，保留原有 registry 版本。
- **工具链口径**：最终验证使用仓内声明的 `nightly-2026-09-04`，
  `rustc 1.100.0-nightly (a69a63265 2026-09-03)`。此前的初轮验证使用环境内
  `nightly-2026-10-09`；声明工具链的验证缺口已由本轮安装和复验关闭。

## 发布闸门（必须同轮次同步）

owner 包已经在 Kernel 仓该 full SHA `a9986c82127cf25183bf5c30e7d42e4a9376b734` 上
**公开**——Git 发布已就绪；本阶段不做、也不声称做过 crates.io 发布。故闸门不是
"等 registry 发布"，而是下面三步的提前接受顺序：

1. 接受本适配器，并把外部适配器 `wire`（`api/wire` 的 facade）一起钉到**同一个
   full SHA**。两个适配器必须同轮次、同一 SHA 落地：只搬一个就得到"两份定义
   并存"的半个图。
2. 旧外部 `mold` 暂时原地不动：过程宏包在保留 `entry` 的同时无法再转发这批过程宏。
   外部 `mold` 的拆分（普通 facade + 独立的 `entry` owner）是另一个 PR；在那之前
   **不得声称 mold 替换完成**。
3. 上面两项都落地后，才把**整个消费方闭包**一次性切到该 full SHA。本 PR 不做任何
   生产消费方切换，不改生产锁文件；新旧 owner 混用的部分图必须被拒。
