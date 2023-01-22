#![feature(iter_collect_into)]

extern crate enum_map;
extern crate rand;

pub mod cpu;
pub mod common;
pub mod register;
pub mod display;
pub mod instruction;
mod font;

pub use common::Error;
pub use cpu::CPU;
pub use instruction::Instruction;
pub use register::Register;
