use crate::{Error, Chip8Mode};
use crate::Register;
use crate::instruction::Instruction;
use crate::display::Display;
use crate::font;

use core::convert::Infallible;
use core::time::Duration;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
#[cfg(feature = "serde")]
use serde_big_array::BigArray;

const TIMER_SPEED: f64 = 60.0;
const CLOCK_SPEED: f64 = 500.0;

pub const CHIP8_MEM_SIZE: usize = 4096;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use std::vec::Vec;

        #[repr(transparent)]
        #[derive(Clone, Debug, Default)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct CallStack {
            stack: Vec<u16>,
        }

        impl CallStack {
            pub fn new() -> Self {
                Self::default()
            }
            pub fn push(&mut self, addr: u16) {
                self.stack.push(addr);
            }
            pub fn pop(&mut self) -> Option<u16> {
                self.stack.pop()
            }
        }

        impl<'a> IntoIterator for &'a CallStack {
            type Item = &'a u16;
            type IntoIter = std::slice::Iter<'a, u16>;

            fn into_iter(self) -> Self::IntoIter {
                self.stack.iter()
            }
        }
    } else {
        const CALL_STACK_SIZE: usize = 64;

        #[derive(Clone, Debug)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct CallStack {
            #[cfg_attr(feature = "serde", serde(with = "BigArray"))]
            call_stack: [u16; CALL_STACK_SIZE],
            call_stack_idx: usize,
        }

        impl CallStack {
            pub fn new() -> Self {
                CallStack {
                    call_stack: [0; CALL_STACK_SIZE],
                    call_stack_idx: 0,
                }
            }
            pub fn push(&mut self, addr: u16) {
                if self.call_stack_idx == CALL_STACK_SIZE {
                    // TODO raise error instead of no-op if out of space?
                    return;
                }
                self.call_stack[self.call_stack_idx] = addr;
                self.call_stack_idx += 1;
            }
            pub fn pop(&mut self) -> Option<u16> {
                if self.call_stack_idx == 0 {
                    None
                } else {
                    self.call_stack_idx -= 1;
                    Some(self.call_stack[self.call_stack_idx])
                }
            }
        }
        impl Default for CallStack {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<'a> IntoIterator for &'a CallStack {
            type Item = &'a u16;
            type IntoIter = core::slice::Iter<'a, u16>;

            fn into_iter(self) -> Self::IntoIter {
                self.call_stack[0..self.call_stack_idx].iter()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct CPU {
    cycles_pending: f64,
    timers_pending: f64,
    mode: Chip8Mode,

    pub pc: u16,
    pub index: u16,
    pub registers: enum_map::EnumMap<Register, u8>,
    pub memory: [u8; CHIP8_MEM_SIZE],
    pub screen: Display,
    pub call_stack: CallStack,
    pub delay_timer: u8,
    pub sound_timer: u8,

    input: [bool; 16],
    awaiting_key: Option<Register>,

    #[cfg(feature = "cosmac")]
    pub(crate) vblank_wait: bool,

    #[cfg(any(feature = "super-chip", feature = "xo-chip"))]
    pub exited: bool,
    #[cfg(any(feature = "super-chip", feature = "xo-chip"))]
    pub(crate) persistent_registers: enum_map::EnumMap<Register, u8>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SavedState {
    mode: Chip8Mode,
    pc: u16,
    index: u16,
    registers: enum_map::EnumMap<Register, u8>,
    #[cfg_attr(feature = "serde", serde(with = "BigArray"))]
    memory: [u8; CHIP8_MEM_SIZE],
    screen: Display,
    call_stack: CallStack,
    delay_timer: u8,
    sound_timer: u8,
    awaiting_key: Option<Register>,
    #[cfg(any(feature = "super-chip", feature = "xo-chip"))]
    persistent_registers: enum_map::EnumMap<Register, u8>
}

impl CPU {
    pub fn new(mode: Chip8Mode,) -> CPU {
        let mut cpu = CPU {
            cycles_pending: 0.0,
            timers_pending: 0.0,
            mode,
            pc: 0x200,
            memory: [0; CHIP8_MEM_SIZE],
            screen: Display::new(),
            registers: enum_map::enum_map! { _ => 0 },
            call_stack: CallStack::new(),
            delay_timer: 0,
            sound_timer: 0,
            input: [false; 16],
            awaiting_key: None,
            index: 0,

            #[cfg(feature = "cosmac")]
            vblank_wait: false,

            #[cfg(any(feature = "super-chip", feature = "xo-chip"))]
            exited: false,
            #[cfg(any(feature = "super-chip", feature = "xo-chip"))]
            persistent_registers: enum_map::enum_map! { _ => 0 },
        };
        cpu.clear_memory();
        cpu
    }

    fn clear_memory(&mut self) {
        self.memory.fill(0);
        self.memory[0x50..0xA0].copy_from_slice(&font::FONT_SET);
    }

    pub fn step(&mut self) -> Result<u32, Error> {
        #[cfg(any(feature = "super-chip", feature = "xo-chip"))]
        if self.exited {
            return Err(Error::Exited);
        }

        if self.awaiting_key.is_some() {
            return Ok(1);
        }
        let opcode = self.read_memory_word(self.pc)?;
        let inst = Instruction::lookup(opcode);
        if let Some(inst) = inst {
            let cycles = inst.cycles;
            self.pc += 2;
            if let Some(op) = match self.mode {
                #[cfg(feature = "cosmac")]
                Chip8Mode::Cosmac => inst.cosmac,
                #[cfg(feature = "super-chip")]
                Chip8Mode::SuperChip => inst.schip,
                #[cfg(feature = "xo-chip")]
                Chip8Mode::XoChip => inst.xochip,
            } {
                let extra_cycles = op(self, opcode);
                Ok(cycles + extra_cycles)
            } else {
                Err(Error::NotDefined(inst.disassembly))
            }
        } else {
            Err(Error::UnknownOpcode(opcode))
        }
    }

    pub fn emulate_for(&mut self, dur: Duration) -> Result<(), Error> {
        #[cfg(any(feature = "super-chip", feature = "xo-chip"))]
        if self.exited {
            return Err(Error::Exited);
        }

        let dt = dur.as_secs_f64();
        self.timers_pending += dt * TIMER_SPEED;
        let timer_diff = self.timers_pending as u8;
        self.delay_timer = self.delay_timer.saturating_sub(timer_diff);
        self.sound_timer = self.sound_timer.saturating_sub(timer_diff);
        self.timers_pending -= timer_diff as f64;
        #[cfg(feature = "cosmac")]
        if timer_diff > 0 {
            self.vblank_wait = false;
        } else if self.vblank_wait {
            return Ok(());
        }

        if self.awaiting_key.is_some() {
            return Ok(());
        }

        self.cycles_pending += dt * CLOCK_SPEED;
        while self.cycles_pending > 0.0 {
            let cycles_taken = self.step()?;
            self.cycles_pending -= cycles_taken as f64;
        }
        Ok(())
    }

    pub fn emulate_breakpoints(&mut self, dur: Duration, breakpoints: &[u16]) -> Result<(), Error> {
        #[cfg(any(feature = "super-chip", feature = "xo-chip"))]
        if self.exited {
            return Err(Error::Exited);
        }

        let dt = dur.as_secs_f64();
        self.timers_pending += dt * TIMER_SPEED;
        let timer_diff = self.timers_pending as u8;
        self.delay_timer = self.delay_timer.saturating_sub(timer_diff);
        self.sound_timer = self.sound_timer.saturating_sub(timer_diff);
        self.timers_pending -= timer_diff as f64;
        #[cfg(feature = "cosmac")]
        if timer_diff > 0 {
            self.vblank_wait = false;
        } else if self.vblank_wait {
            return Ok(());
        }

        if self.awaiting_key.is_some() {
            return Ok(());
        }

        self.cycles_pending += dt * CLOCK_SPEED;
        while self.cycles_pending > 0.0 {
            let cycles_taken = self.step()?;
            for i in breakpoints {
                if self.pc == *i {
                    self.cycles_pending = 0.0;
                    return Err(Error::Breakpoint(self.pc));
                }
            }
            self.cycles_pending -= cycles_taken as f64;
        }
        Ok(())
    }

    pub fn should_beep(&self) -> bool {
        self.sound_timer > 0
    }

    pub(crate) fn call_routine(&mut self, pos: u16) {
        self.call_stack.push(self.pc);
        self.pc = pos;
    }

    pub(crate) fn return_routine(&mut self) {
        if let Some(addr) = self.call_stack.pop() {
            self.pc = addr;
        }
        // TODO raise error instead of no-op if no return address?
    }

    pub fn read_memory_byte(&self, pos: u16) -> Result<u8, Infallible> {
        Ok(self.memory[pos as usize])
    }

    pub fn read_memory_word(&self, pos: u16) -> Result<u16, Error> {
        if pos == 0xFFFF {
            Err(Error::OutOfBounds)
        } else {
            let byte1 = self.memory[pos as usize];
            let byte2 = self.memory[(pos + 1) as usize];
            Ok((byte1 as u16) << 8 | (byte2 as u16))
        }
    }

    pub fn write_memory_byte(&mut self, pos: u16, byte: u8) -> Result<(), Infallible> {
        self.memory[pos as usize] = byte;
        Ok(())
    }

    pub fn write_memory_word(&mut self, pos: u16, word: u16) -> Result<(), Error> {
        if pos == 0xFFFF {
            Err(Error::OutOfBounds)
        } else {
            self.memory[pos as usize] = (word >> 8) as u8;
            self.memory[(pos + 1) as usize] = (word & 0xFF) as u8;
            Ok(())
        }
    }

    pub fn is_key_down(&self, key: u8) -> bool {
        if key > 0xF {
            // TODO raise error instead of no-op if out-of-bounds?
            false
        } else {
            self.input[key as usize]
        }
    }

    pub(crate) fn await_key(&mut self, post_reg: Register) {
        self.awaiting_key = Some(post_reg);
    }

    pub fn press_key(&mut self, key: u8) {
        if key > 0xF {
            // TODO raise error instead of no-op if out-of-bounds?
        } else {
            self.input[key as usize] = true;
            if let Some(reg) = self.awaiting_key {
                self.registers[reg] = key;
                self.awaiting_key = None;
            }
        }
    }

    pub fn release_key(&mut self, key: u8) {
        if key > 0xF {
            // TODO raise error instead of no-op if out-of-bounds?
        } else {
            self.input[key as usize] = false;
        }
    }

    pub fn load_rom(&mut self, buf: &[u8]) -> Result<(), Error> {
        if buf.len() > 0xE00 {
            return Err(Error::InvalidFile);
        }
        self.clear_memory();
        self.memory[0x200..(buf.len() + 0x200)].copy_from_slice(buf);
        Ok(())
    }

    pub fn disassemble(&self, idx: u16) -> Option<&str> {
        let word = self.read_memory_word(idx);
        if let Ok(word) = word {
            Instruction::lookup(word).map(|inst| inst.disassembly)
        } else {
            None
        }
    }

    pub fn disassemble_next(&self) -> Option<&str> {
        let word = self.read_memory_word(self.pc);
        if let Ok(word) = word {
            Instruction::lookup(word).map(|inst| inst.disassembly)
        } else {
            None
        }
    }

    pub fn save_state(&self) -> Result<SavedState, Error> {
        #[cfg(any(feature = "super-chip", feature = "xo-chip"))]
        if self.exited {
            return Err(Error::Exited);
        }
        Ok(SavedState {
            mode: self.mode,
            pc: self.pc,
            index: self.index,
            registers: self.registers,
            memory: self.memory,
            screen: self.screen.clone(),
            call_stack: self.call_stack.clone(),
            delay_timer: self.delay_timer,
            sound_timer: self.sound_timer,
            awaiting_key: self.awaiting_key,
            #[cfg(any(feature = "super-chip", feature = "xo-chip"))]
            persistent_registers: self.persistent_registers,
        })
    }

    pub fn load_state(&mut self, state: SavedState) {
        self.mode = state.mode;

        cfg_if::cfg_if! {
            if #[cfg(any(feature = "super-chip", feature = "xo-chip"))] {
                self.exited = false;
                self.persistent_registers = state.persistent_registers;
            }
        }
        self.cycles_pending = 0.0;
        self.timers_pending = 0.0;
        cfg_if::cfg_if! {
            if #[cfg(feature = "cosmac")] {
                self.vblank_wait = false;
            }
        }

        self.pc = state.pc;
        self.index = state.index;
        self.registers = state.registers;
        self.memory = state.memory;
        self.screen = state.screen;
        self.call_stack = state.call_stack;
        self.delay_timer = state.delay_timer;
        self.sound_timer = state.sound_timer;
        self.awaiting_key = state.awaiting_key;
    }
}
