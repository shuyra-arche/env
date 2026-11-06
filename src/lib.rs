#![no_std]
//! Legacy environment-call compatibility names.
//!
//! Types and codecs are re-exported from the single canonical API owner;
//! existing typed call paths delegate to Kernel Client. New pure consumers
//! should depend directly on API and new effectful consumers on Client.
//! This adapter owns no ABI definitions, transport or execution state.
//! `env-initrd` remains an independent pure API consumer.

pub use kernel_api_env::*;
pub use kernel_client::call::{
    chrono, control, debug, hole, memory, nole, pie, pole, room, tole,
};

pub mod ecall;
pub mod unit;
