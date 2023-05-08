#![no_std]

extern crate enum_map;
extern crate rand;
extern crate cfg_if;
#[cfg(feature = "std")]
extern crate std;

pub mod cpu;
pub mod common;
pub mod register;
pub mod display;
pub mod instruction;
mod font;

pub use common::Error;
pub use cpu::CPU;
pub use register::Register;
