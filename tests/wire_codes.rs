//! `env::WireCodes` 经 facade 的派生面：码表 ＋ `fail_to_code` / `code_to_fail`。
//!
//! 派生生成的正文里 `OK` 是**调用处提供的**名字（derive 不认 crate 路径），故这里
//! 按权威口径把它从 `kernel_api_wire` 引进来。

use kernel_api_wire::OK;

#[derive(Clone, Copy, Debug, PartialEq, Eq, env::WireCodes)]
#[wire(also(RETIRED = 3))]
pub enum TinyFail {
    #[code(1)]
    Unknown,
    #[code(2)]
    Taken,
}

#[test]
fn wire_codes_derive_keeps_the_domain_code_table_through_the_facade() {
    assert_eq!(UNKNOWN, 1);
    assert_eq!(TAKEN, 2);
    assert_eq!(RETIRED, 3);

    assert_eq!(fail_to_code(None), OK);
    assert_eq!(fail_to_code(Some(TinyFail::Unknown)), UNKNOWN);
    assert_eq!(fail_to_code(Some(TinyFail::Taken)), TAKEN);

    assert_eq!(code_to_fail(UNKNOWN), Some(TinyFail::Unknown));
    assert_eq!(code_to_fail(TAKEN), Some(TinyFail::Taken));
    assert_eq!(code_to_fail(OK), None);
    // `also` 那一格只有码、没有变体，且没写 `fallback` ⇒ 反向落表外。
    assert_eq!(code_to_fail(RETIRED), None);
}
