use crate::audio::Audio;
use crate::display::Display;
use crate::font;
use crate::instruction::Instruction;
use crate::Register;
use crate::{Chip8Mode, Error};

use core::time::Duration;

use nanorand::{Rng, SeedableRng, WyRand};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_big_array::BigArray;

const TIMER_SPEED: f64 = 60.0;
const CLOCK_SPEED: f64 = 500.0;

#[cfg(not(feature = "xo-chip"))]
pub const CHIP8_MEM_SIZE: usize = 0x1000;
#[cfg(feature = "xo-chip")]
pub const CHIP8_MEM_SIZE: usize = 0x10000;

cfg_if::cfg_if! {
    if #[cfg(feature = "alloc")] {
        use alloc::vec::Vec;

        #[repr(transparent)]
        #[derive(Clone, Debug, Default)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct CallStack {
            stack: Vec<u16>,
        }

        impl CallStack {
            #[inline]
            pub fn new() -> Self {
                Self::default()
            }
            #[inline]
            pub fn push(&mut self, addr: u16) {
                self.stack.push(addr);
            }
            #[inline]
            pub fn pop(&mut self) -> Option<u16> {
                self.stack.pop()
            }
            #[inline]
            pub fn iter(&self) -> core::slice::Iter<u16> {
                self.stack.iter()
            }
        }

        impl<'a> IntoIterator for &'a CallStack {
            type Item = &'a u16;
            type IntoIter = core::slice::Iter<'a, u16>;

            #[inline]
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
            #[inline]
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
            #[inline]
            pub fn iter(&self) -> core::slice::Iter<u16> {
                self.call_stack[0..self.call_stack_idx].iter()
            }
        }
        impl Default for CallStack {
            #[inline]
            fn default() -> Self {
                Self::new()
            }
        }

        impl<'a> IntoIterator for &'a CallStack {
            type Item = &'a u16;
            type IntoIter = core::slice::Iter<'a, u16>;

            #[inline]
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
    pub mode: Chip8Mode,

    pub pc: u16,
    pub index: u16,
    pub registers: enum_map::EnumMap<Register, u8>,
    pub memory: [u8; CHIP8_MEM_SIZE],
    pub screen: Display,
    pub call_stack: CallStack,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub audio: Audio,

    input: [bool; 16],
    awaiting_key: Option<Register>,
    random_state: WyRand,

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
    seed: [u8; 8],
    #[cfg(any(feature = "super-chip", feature = "xo-chip"))]
    persistent_registers: enum_map::EnumMap<Register, u8>,
}

impl CPU {
    pub fn new(mode: Chip8Mode) -> CPU {
        let mut cpu = CPU {
            cycles_pending: 0.0,
            timers_pending: 0.0,
            mode,

            pc: 0x200,
            index: 0,
            registers: enum_map::enum_map! { _ => 0 },
            memory: [0; CHIP8_MEM_SIZE],
            screen: Display::new(),
            call_stack: CallStack::new(),
            delay_timer: 0,
            sound_timer: 0,
            audio: Audio::new(),

            input: [false; 16],
            awaiting_key: None,
            random_state: WyRand::new(),

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

    #[cfg(not(any(feature = "super-chip", feature = "xo-chip")))]
    #[inline]
    fn clear_memory(&mut self) {
        self.memory.fill(0);
        self.memory[0x50..0xA0].copy_from_slice(&font::FONT_SET);
    }
    #[cfg(any(feature = "super-chip", feature = "xo-chip"))]
    #[inline]
    fn clear_memory(&mut self) {
        self.memory.fill(0);
        self.memory[0x50..0xA0].copy_from_slice(&font::FONT_SET);
        self.memory[0xA0..0x140].copy_from_slice(&font::BIG_FONT_SET);
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
            let op: Option<crate::instruction::OpcodeExecute> = match self.mode {
                #[cfg(feature = "cosmac")]
                Chip8Mode::Cosmac => inst.cosmac,
                #[cfg(feature = "super-chip")]
                Chip8Mode::SuperChip => inst.schip,
                #[cfg(feature = "xo-chip")]
                Chip8Mode::XoChip => inst.xochip,
            };
            // Code will only be unreachable if none of the mode features are
            // enabled; in the interest of having only the `compile_error!`
            // saying that one of them needs to be enabled, disable this warning
            // if none of them are enabled.
            #[cfg_attr(
                not(any(feature = "cosmac", feature = "super-chip", feature = "xo-chip")),
                allow(unreachable_code)
            )]
            if let Some(op) = op {
                let extra_cycles = op(self, opcode);
                Ok(cycles + extra_cycles)
            } else {
                Err(Error::NotDefined(inst.disassembly, self.mode))
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

            // checks for early exit
            #[cfg(feature = "cosmac")]
            if self.vblank_wait {
                return Ok(());
            }
            if self.awaiting_key.is_some() {
                return Ok(());
            }
        }
        Ok(())
    }

    pub fn emulate_for_until(&mut self, dur: Duration, halt: impl Fn(&CPU) -> bool) -> Result<(), Error> {
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

            // checks for early exit
            if halt(self) {
                return Err(Error::EarlyExitRequested);
            }
            #[cfg(feature = "cosmac")]
            if self.vblank_wait {
                return Ok(());
            }
            if self.awaiting_key.is_some() {
                return Ok(());
            }
        }
        Ok(())
    }

    #[inline]
    pub fn read_beep_samples_to(&mut self, dur: Duration, buf: &mut [f32]) -> usize {
        if self.sound_timer > 0 {
            self.audio.read_samples_to(dur, buf)
        } else {
            0
        }
    }

    #[cfg(feature = "std")]
    #[inline]
    pub fn get_beep_samples(&mut self, dur: Duration) -> Option<Vec<f32>> {
        if self.sound_timer > 0 {
            Some(self.audio.get_samples(dur))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn call_routine(&mut self, pos: u16) {
        self.call_stack.push(self.pc);
        self.pc = pos;
    }

    #[inline]
    pub(crate) fn return_routine(&mut self) {
        if let Some(addr) = self.call_stack.pop() {
            self.pc = addr;
        }
        // TODO raise error instead of no-op if no return address?
    }

    #[inline]
    pub fn read_memory_byte(&self, pos: u16) -> Result<u8, Error> {
        let pos = pos as usize;
        if pos >= CHIP8_MEM_SIZE {
            Err(Error::OutOfBounds)
        } else {
            Ok(self.memory[pos])
        }
    }

    #[inline]
    pub fn read_memory_word(&self, pos: u16) -> Result<u16, Error> {
        let pos = pos as usize;
        if pos >= (CHIP8_MEM_SIZE - 1) {
            Err(Error::OutOfBounds)
        } else {
            let byte1 = self.memory[pos];
            let byte2 = self.memory[pos + 1];
            Ok((byte1 as u16) << 8 | (byte2 as u16))
        }
    }

    #[inline]
    pub fn write_memory_byte(&mut self, pos: u16, byte: u8) -> Result<(), Error> {
        let pos = pos as usize;
        if pos >= CHIP8_MEM_SIZE {
            Err(Error::OutOfBounds)
        } else {
            self.memory[pos] = byte;
            Ok(())
        }
    }

    #[inline]
    pub fn write_memory_word(&mut self, pos: u16, word: u16) -> Result<(), Error> {
        let pos = pos as usize;
        if pos >= (CHIP8_MEM_SIZE - 1) {
            Err(Error::OutOfBounds)
        } else {
            self.memory[pos] = (word >> 8) as u8;
            self.memory[pos + 1] = (word & 0xFF) as u8;
            Ok(())
        }
    }

    #[inline]
    pub fn is_key_down(&self, key: u8) -> bool {
        if key > 0xF {
            // TODO raise error instead of no-op if out-of-bounds?
            false
        } else {
            self.input[key as usize]
        }
    }

    #[inline]
    pub(crate) fn await_key(&mut self, post_reg: Register) {
        self.awaiting_key = Some(post_reg);
    }

    #[inline]
    pub fn press_key(&mut self, key: u8) {
        if key > 0xF {
            // TODO raise error instead of no-op if out-of-bounds?
        } else {
            self.input[key as usize] = true;
        }
    }

    #[inline]
    pub fn release_key(&mut self, key: u8) {
        if key > 0xF {
            // TODO raise error instead of no-op if out-of-bounds?
        } else {
            self.input[key as usize] = false;
            if let Some(reg) = self.awaiting_key {
                self.registers[reg] = key;
                self.awaiting_key = None;
            }
        }
    }

    pub fn load_rom(&mut self, buf: &[u8]) -> Result<(), Error> {
        if buf.len() > (CHIP8_MEM_SIZE - 0x200) {
            return Err(Error::InvalidFile);
        }
        #[cfg(feature = "xo-chip")]
        if buf.len() > 0x1000 && self.mode != Chip8Mode::XoChip {
            info!("Attempted to load a large ROM outside XO-CHIP mode; switching modes");
            let _ = core::mem::replace(self, Self::new(Chip8Mode::XoChip));
            self.memory[0x200..(buf.len() + 0x200)].copy_from_slice(buf);
            return Ok(());
        }
        let _ = core::mem::replace(self, Self::new(self.mode));
        self.memory[0x200..(buf.len() + 0x200)].copy_from_slice(buf);
        Ok(())
    }

    pub fn hotswap(&mut self, buf: &[u8]) -> Result<(), Error> {
        if buf.len() > (CHIP8_MEM_SIZE - 0x200) {
            return Err(Error::InvalidFile);
        }
        self.memory[0x200..(buf.len() + 0x200)].copy_from_slice(buf);
        Ok(())
    }

    #[inline]
    pub fn disassemble(&self, idx: u16) -> Option<&str> {
        let word = self.read_memory_word(idx);
        if let Ok(word) = word {
            Instruction::lookup(word).map(|inst| inst.disassembly)
        } else {
            None
        }
    }

    #[inline]
    pub fn disassemble_next(&self) -> Option<&str> {
        self.disassemble(self.pc)
    }

    pub fn save_state(&mut self) -> Result<SavedState, Error> {
        #[cfg(any(feature = "super-chip", feature = "xo-chip"))]
        if self.exited {
            return Err(Error::Exited);
        }
        let seed = self.random_state.rand();
        // for consistency of random state in saved states
        self.random_state.reseed(seed);
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
            seed,
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
        self.random_state.reseed(state.seed);
    }

    #[inline]
    pub fn random(&mut self) -> u8 {
        self.random_state.generate()
    }

    #[inline]
    pub fn reseed(&mut self, seed: u64) {
        self.random_state = WyRand::new_seed(seed);
    }
}

impl Default for CPU {
    #[inline]
    fn default() -> Self {
        CPU::new(Chip8Mode::default())
    }
}
