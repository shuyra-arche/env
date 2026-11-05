// Both packages compile the same format implementation. No ABI dependency or
// second implementation is needed by host and runtime bundle consumers.
#[path = "../../src/ledger/capsule.rs"]
pub mod capsule;
#[path = "../../src/ledger/bundle.rs"]
pub mod bundle;
