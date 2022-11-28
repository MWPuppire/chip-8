use crate::common::Error;
use crate::register::Register;
use crate::display::Display;
use crate::instruction::Instruction;
use crate::font;

const TIMER_SPEED: f64 = 60.0;
const CLOCK_SPEED: f64 = 500.0;

pub struct CPU {
    cycles_pending: f64,
    timers_pending: f64,

    pub pc: u16,
    pub index: u16,
    pub registers: enum_map::EnumMap<Register, u8>,
    pub memory: [u8; 4096],
    pub screen: Display,
    call_stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,

    input: [bool; 16],
    awaiting_key: Option<Register>,
}

impl CPU {
    pub fn new() -> CPU {
        let mut cpu = CPU {
            cycles_pending: 0.0,
            timers_pending: 0.0,
            pc: 0x200,
            memory: [0; 4096],
            screen: Display::new(),
            registers: enum_map::enum_map! {
                Register::V0 => 0,
                Register::V1 => 0,
                Register::V2 => 0,
                Register::V3 => 0,
                Register::V4 => 0,
                Register::V5 => 0,
                Register::V6 => 0,
                Register::V7 => 0,
                Register::V8 => 0,
                Register::V9 => 0,
                Register::VA => 0,
                Register::VB => 0,
                Register::VC => 0,
                Register::VD => 0,
                Register::VE => 0,
                Register::VF => 0,
            },
            call_stack: vec!(),
            delay_timer: 0,
            sound_timer: 0,
            input: [false; 16],
            awaiting_key: None,
            index: 0,
        };
        cpu.clear_memory();
        cpu
    }

    fn clear_memory(&mut self) {
        self.memory.fill(0);
        self.memory[0x50..0xA0].copy_from_slice(&font::FONT_SET);
    }

    pub fn step(&mut self) -> Result<u32, Error> {
        let opcode = self.read_memory_word(self.pc)?;
        let inst = Instruction::lookup(opcode);
        if let Some(inst) = inst {
            let cycles = inst.cycles;
            self.pc += 2;
            let extra_cycles = (inst.execute)(self, opcode);
            Ok(cycles + extra_cycles)
        } else {
            println!("Unknown opcode {:#06x} at memory {:#06x}", opcode, self.pc);
            Err(Error::UnknownOpcode)
        }
    }

    pub fn emulate(&mut self, dt: f64) -> Result<(), Error> {
        self.timers_pending += dt * TIMER_SPEED;
        let timer_diff = self.timers_pending as u8;
        self.delay_timer = self.delay_timer.checked_sub(timer_diff).unwrap_or(0);
        self.sound_timer = self.sound_timer.checked_sub(timer_diff).unwrap_or(0);
        self.timers_pending -= timer_diff as f64;

        if let Some(_) = self.awaiting_key {
            return Ok(());
        }

        self.cycles_pending += dt * CLOCK_SPEED;
        while self.cycles_pending > 0.0 {
            let cycles_taken = self.step()?;
            self.cycles_pending -= cycles_taken as f64;
        }
        Ok(())
    }
    pub fn emulate_until(&mut self, dt: f64, breakpoints: &[u16]) -> Result<(), Error> {
        self.timers_pending += dt * TIMER_SPEED;
        let timer_diff = self.timers_pending as u8;
        self.delay_timer = self.delay_timer.checked_sub(timer_diff).unwrap_or(0);
        self.sound_timer = self.sound_timer.checked_sub(timer_diff).unwrap_or(0);
        self.timers_pending -= timer_diff as f64;

        if let Some(_) = self.awaiting_key {
            return Ok(());
        }

        self.cycles_pending += dt * CLOCK_SPEED;
        while self.cycles_pending > 0.0 {
            let cycles_taken = self.step()?;
            for i in breakpoints {
                if self.pc == *i {
                    self.cycles_pending = 0.0;
                    return Err(Error::Breakpoint);
                }
            }
            self.cycles_pending -= cycles_taken as f64;
        }
        Ok(())
    }

    pub fn should_beep(&self) -> bool {
        self.sound_timer > 0
    }

    pub fn call_routine(&mut self, pos: u16) {
        self.call_stack.push(self.pc);
        self.pc = pos;
    }

    pub fn return_routine(&mut self) {
        if let Some(pos) = self.call_stack.pop() {
            self.pc = pos;
        }
        // TO-DO raise error instead of no-op if no return address?
    }

    pub fn read_memory_byte(&self, pos: u16) -> Result<u8, Error> {
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

    pub fn write_memory_byte(&mut self, pos: u16, byte: u8) -> Result<(), Error> {
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
            // TO-DO raise error instead of no-op if out-of-bounds?
            false
        } else {
            self.input[key as usize]
        }
    }

    pub fn await_key(&mut self, post_reg: Register) {
        self.awaiting_key = Some(post_reg);
    }

    pub fn press_key(&mut self, key: u8) {
        if key > 0xF {
            // TO-DO raise error instead of no-op if out-of-bounds?
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
            // TO-DO raise error instead of no-op if out-of-bounds?
        } else {
            self.input[key as usize] = false;
        }
    }

    pub fn load_rom(&mut self, buf: Vec<u8>) -> Result<(), Error> {
        if buf.len() > 0xE00 {
            return Err(Error::InvalidFile);
        }
        self.clear_memory();
        for i in 0..buf.len() {
            self.memory[i + 0x200] = buf[i];
        }
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
}
