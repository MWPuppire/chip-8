#![no_std]

extern crate enum_map;
extern crate rand;
extern crate cfg_if;
#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "serde")]
extern crate serde;
#[cfg(feature = "serde")]
extern crate serde_big_array;

pub mod cpu;
pub mod common;
pub mod register;
pub mod display;
mod instruction;
mod font;

pub use common::{Error, Chip8Mode};
pub use cpu::CPU;
pub use register::Register;
