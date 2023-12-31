#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;
extern crate cfg_if;
extern crate enum_map;
extern crate nanorand;
#[cfg(feature = "serde")]
extern crate serde;
#[cfg(feature = "serde")]
extern crate serde_big_array;
#[cfg(feature = "std")]
extern crate std;
#[cfg_attr(feature = "xo-chip", macro_use)]
extern crate tracing;

pub mod audio;
mod common;
pub mod cpu;
pub mod display;
mod font;
mod instruction;
pub mod register;

pub use common::{Chip8Mode, Error};
pub use cpu::CPU;
pub use register::Register;
