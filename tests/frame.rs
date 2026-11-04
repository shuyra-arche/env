//! `env::Frame` 经 facade 的派生面：**默认 `::env::wire` 路径**。
//!
//! 这条判据只吃 facade 上的名字：`Frame` 派生默认生成 `::env::wire::{Span, total}`
//! （`codec` 缺省值），故它同时证两件事——(1) 历史派生名 `env::Frame` 仍由 glob
//! facade 转出；(2) `::env::wire` 在这个 crate 名下解析到 `kernel_api_env::wire`，
//! 也就是 `kernel_api_wire` 的字节 codec。

#[derive(Clone, Copy, Debug, PartialEq, Eq, env::Frame)]
pub struct Ping {
    pub who: u32,
    pub tag: u64,
}

#[test]
fn frame_derive_uses_the_default_env_wire_path() {
    // `LEN` 来自 `::env::wire::total`：4 + 8，没有字面偏移。
    assert_eq!(Ping::LEN, 12);
    assert_eq!(<Ping as env::wire::Span>::MAX, Some(Ping::LEN));

    let ping = Ping {
        who: 0x1234_5678,
        tag: 0x0102_0304_0506_0708,
    };
    let mut bytes = [0u8; Ping::LEN];
    let end = <Ping as env::wire::Span>::store_at(&ping, &mut bytes, 0).expect("store_at");
    assert_eq!(end, Ping::LEN);
    assert_eq!(&bytes[..4], &0x1234_5678u32.to_le_bytes());
    assert_eq!(&bytes[4..], &0x0102_0304_0506_0708u64.to_le_bytes());

    let (back, read) = <Ping as env::wire::Span>::fetch_at(&bytes, 0).expect("fetch_at");
    assert_eq!((back, read), (ping, Ping::LEN));

    // 短一字节就读不懂——同一份 `Span::fetch_at` 的口径。
    assert!(<Ping as env::wire::Span>::fetch_at(&bytes[..Ping::LEN - 1], 0).is_none());
}
