#![no_std]
//! `env` 兼容适配器（facade）：**只转出，不定义**。
//!
//! 权威定义住在 Kernel 的 `kernel-api-env` 包里；本仓整份源码只有下面那一行
//! `pub use kernel_api_env::*;`——不写调用表、不写线类型、不写 codec、不写权限位、
//! 不写 marks，也没有任何运行时定义。
//!
//! `env::{Permission, MemoryCall, TaskId, Mark}` 与 `kernel_api_env`、
//! `kernel_api::env` 指向同一份 ABI 类型；`env::wire::{Field, Span}` 则转出
//! `kernel_api_wire` 拥有的字节 codec trait。判据在 `tests/identity.rs`：
//! 双向赋值、`TypeId` 相等、泛型约束互传。
//!
//! 依赖方向只有一条：`env` → `kernel-api-env`（→ `kernel-api-wire` /
//! `kernel-api-macros`）。本仓不依赖 `kernel-api` 伞包，也不依赖任何旧定义仓
//! （旧 `env` / `wire` / `mold`）：一个构建图里出现两份 `env` 或两份 `wire` 的定义
//! 会破类型同一性，故新旧 owner **不得并存**。
//!
//! `kernel-api` 只供 `tests/identity.rs`；`kernel-api-wire` 供同一性测试与
//! `tests/wire_codes.rs`。两者都是 dev-dependency，不增加运行时直接依赖。

pub use kernel_api_env::*;
