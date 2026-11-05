#![no_std]

extern crate alloc;

/// The initrd capsule format specifies 4 KiB pages independently of ABI crates.
pub const PAGE_SIZE: usize = 4096;

pub mod ledger;
