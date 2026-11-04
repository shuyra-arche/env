//! `env` 适配器的同一性判据：**一枚名字只有一份定义**。
//!
//! 本仓不定义任何类型或 trait。下面每一条跨路径的赋值、`TypeId` 判据与泛型约束，
//! 只比较**本文件点到的路径**：facade `env`、权威 `kernel_api_env`、伞包
//! `kernel_api::env` 这三条指名同一枚 nominal 类型；`env::wire::{Field, Span}` 与
//! `kernel_api_wire::{Field, Span}` 指名同一条共享 trait。路径指错（出现第二枚 nominal
//! 类型或第二条 trait）时，这些签名编不过。
//!
//! **这不是整图去重**：这三条路径之外的旧 `env` / `wire` 包可以留在图里而没人用到、
//! 也不被这些签名触到，本文件发现不了它。"整图里只许有一份 `env` / `wire`"由 Root 的
//! `cargo metadata` 与锁审计单独判，不由这里的赋值与约束判。
//!
//! 口径提醒：`kernel-api-wire` 拥有**字节 codec**（`Field` / `Span` 与尾部助手），
//! 不拥有 `TaskId`；句柄（`TaskId` / `Mark` / `PieToken` …）是 `kernel-api-env` 的
//! 定义，本文件因此**不**引用 `kernel_api_wire::TaskId`（那里没有这个名字）。

use core::any::TypeId;

use env as facade;
use kernel_api::env as umbrella;
use kernel_api_env as canonical;
use kernel_api_wire as wire_owner;

/// 两条路径点名同一枚类型 ⇒ `TypeId` 必须相等（`TypeId` 按 nominal 身份取）。
fn assert_same_type<A: 'static, B: 'static>() {
    assert_eq!(TypeId::of::<A>(), TypeId::of::<B>());
}

#[test]
fn permission_is_the_canonical_permission_on_every_path() {
    let permission = facade::Permission::FETCH | facade::Permission::STORE;
    let canonical_permission: canonical::Permission = permission;
    let back: facade::Permission = canonical_permission;
    assert_eq!(back, permission);
    assert_eq!(back.bits(), 0b0011);

    let umbrella_permission: umbrella::Permission = canonical_permission;
    let canonical_again: canonical::Permission = umbrella_permission;
    assert_eq!(canonical_again, canonical_permission);

    let _: fn(facade::Permission) -> canonical::Permission = core::convert::identity;
    let _: fn(canonical::Permission) -> facade::Permission = core::convert::identity;
    let _: fn(umbrella::Permission) -> canonical::Permission = core::convert::identity;
    let _: fn(canonical::Permission) -> umbrella::Permission = core::convert::identity;
    let _: fn(facade::Permission) -> umbrella::Permission = core::convert::identity;
    let _: fn(umbrella::Permission) -> facade::Permission = core::convert::identity;

    assert_same_type::<facade::Permission, canonical::Permission>();
    assert_same_type::<umbrella::Permission, canonical::Permission>();
}

#[test]
fn memory_call_is_the_canonical_memory_call_on_every_path() {
    let reference = canonical::FlatReference::from_bytes(&7u64.to_le_bytes()).expect("reference");
    let call: facade::MemoryCall = canonical::MemoryCall::Release { reference };
    let canonical_call: canonical::MemoryCall = call;
    let back: facade::MemoryCall = canonical_call;
    assert_eq!(back, canonical::MemoryCall::Release { reference });

    let umbrella_call: umbrella::MemoryCall = canonical_call;
    let canonical_again: canonical::MemoryCall = umbrella_call;
    assert_eq!(canonical_again, canonical_call);

    let _: fn(facade::MemoryCall) -> canonical::MemoryCall = core::convert::identity;
    let _: fn(canonical::MemoryCall) -> facade::MemoryCall = core::convert::identity;
    let _: fn(umbrella::MemoryCall) -> canonical::MemoryCall = core::convert::identity;
    let _: fn(canonical::MemoryCall) -> umbrella::MemoryCall = core::convert::identity;
    let _: fn(facade::MemoryCall) -> umbrella::MemoryCall = core::convert::identity;
    let _: fn(umbrella::MemoryCall) -> facade::MemoryCall = core::convert::identity;

    assert_same_type::<facade::MemoryCall, canonical::MemoryCall>();
    assert_same_type::<umbrella::MemoryCall, canonical::MemoryCall>();
}

#[test]
fn task_id_is_the_canonical_task_id_on_every_path() {
    let task = facade::TaskId::new(7);
    let canonical_task: canonical::TaskId = task;
    let back: facade::TaskId = canonical_task;
    assert_eq!(back, task);
    assert_eq!(back.get(), 7);

    let umbrella_task: umbrella::TaskId = canonical_task;
    let canonical_again: canonical::TaskId = umbrella_task;
    assert_eq!(canonical_again, canonical_task);

    let _: fn(facade::TaskId) -> canonical::TaskId = core::convert::identity;
    let _: fn(canonical::TaskId) -> facade::TaskId = core::convert::identity;
    let _: fn(umbrella::TaskId) -> canonical::TaskId = core::convert::identity;
    let _: fn(canonical::TaskId) -> umbrella::TaskId = core::convert::identity;
    let _: fn(facade::TaskId) -> umbrella::TaskId = core::convert::identity;
    let _: fn(umbrella::TaskId) -> facade::TaskId = core::convert::identity;

    assert_same_type::<facade::TaskId, canonical::TaskId>();
    assert_same_type::<umbrella::TaskId, canonical::TaskId>();
}

/// 约束互传（→）：拿 facade 那条 `Field` 作约束的 `F`，满足 writer 侧同名约束。
fn width_via_owner_trait<F: facade::wire::Field>() -> usize {
    <F as wire_owner::Field>::WIDTH
}

/// 约束互传（←）：拿 writer 侧 `Field` 作约束的 `F`，满足 facade 那条约束。
fn width_via_facade_trait<F: wire_owner::Field>() -> usize {
    <F as facade::wire::Field>::WIDTH
}

/// 同上，换成要游标的 `Span`（`MAX` 是 `Option`：名字那一格不报上界）。
fn max_via_owner_trait<S: wire_owner::Span>() -> Option<usize> {
    <S as facade::wire::Span>::MAX
}

fn max_via_facade_trait<S: facade::wire::Span>() -> Option<usize> {
    <S as wire_owner::Span>::MAX
}

#[test]
fn byte_codec_traits_are_the_kernel_api_wire_traits_in_both_directions() {
    assert_eq!(width_via_owner_trait::<u32>(), 4);
    assert_eq!(width_via_facade_trait::<u64>(), 8);
    assert_eq!(max_via_owner_trait::<[u8; 8]>(), Some(8));
    assert_eq!(max_via_facade_trait::<u32>(), Some(4));
    // 名字那一格不报上界：同一个 impl 经两条路读出来都是 `None`。
    assert_eq!(max_via_owner_trait::<String>(), None);
    assert_eq!(max_via_facade_trait::<String>(), None);

    // 还是同一份 impl：经 facade 写、经 writer 侧读。
    let mut bytes = [0u8; 4];
    <u32 as facade::wire::Field>::store(&7, &mut bytes);
    assert_eq!(<u32 as wire_owner::Field>::fetch(&bytes), Some(7));
}

/// 经**权威** trait 存（`kernel_api_env::Wire`）。
fn store_via_canonical<T: canonical::Wire>(value: &T) -> ([usize; 6], usize) {
    let mut words = [0usize; 6];
    let mut at = 0;
    canonical::Wire::pack(value, &mut words, &mut at);
    (words, at)
}

/// 经**facade** 的 `env::Wire` 取（同一枚 trait，故两条路可互喂）。
fn fetch_via_facade<T: facade::Wire>(words: &[usize; 6]) -> (T, usize) {
    let mut at = 0;
    let value = <T as facade::Wire>::unpack(words, &mut at).expect("decode");
    (value, at)
}

#[test]
fn task_id_and_mark_store_and_fetch_through_the_canonical_trait_and_the_facade() {
    let task = facade::TaskId::new(7);
    let (words, at) = store_via_canonical(&task);
    assert_eq!((words[0], at), (7, 1));
    let (back, read): (canonical::TaskId, usize) = fetch_via_facade(&words);
    assert_eq!((back, read), (task, 1));

    // 记号那一格：挑一个 32 位量，`usize` 那一格在任何宿主宽度上都装得下。
    let mark = canonical::Mark::new(7);
    let (words, at) = store_via_canonical(&mark);
    assert_eq!((words[0] as u64, at), (7, 1));
    let (back, read): (facade::Mark, usize) = fetch_via_facade(&words);
    assert_eq!((back, read), (mark, 1));

    // 两条路径上是同一枚 const 构造函数：同一个名字算出同一个数。
    assert_eq!(facade::Mark::of("env"), canonical::Mark::of("env"));
}
