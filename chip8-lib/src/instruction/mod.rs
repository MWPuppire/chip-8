use crate::CPU;

mod arithmetic;
use arithmetic::*;
mod assignment;
use assignment::*;
mod bitwise;
use bitwise::*;
mod branches;
use branches::*;
mod display;
use display::*;
mod input;
use input::*;
mod jumps;
use jumps::*;
mod misc;
use misc::*;
mod timers;
use timers::*;

pub(crate) struct Instruction {
    pub execute: fn(&mut CPU, u16) -> u32,
    pub cycles: u32,
    pub disassembly: &'static str,
}

impl Instruction {
    pub fn lookup(opcode: u16) -> Option<Instruction> {
        match ((opcode >> 12) & 0xF, (opcode >> 8) & 0xF, (opcode >> 4) & 0xF, opcode & 0xF) {
            (0x0, 0x0, 0x0, 0x0) => Some(Instruction {
                execute: inst_nop,
                cycles: 1,
                disassembly: "(void) 0;",
            }),
            (0x0, 0x0, 0xB,   _) => None, // TODO scroll_up(N);
            (0x0, 0x0, 0xC,   _) => None, // TODO scroll_down(N);
            (0x0, 0x0, 0xE, 0x0) => Some(Instruction {
                execute: inst_clear,
                cycles: 1,
                disassembly: "display_clear();",
            }),
            (0x0, 0x0, 0xE, 0xE) => Some(Instruction {
                execute: inst_return,
                cycles: 1,
                disassembly: "return;",
            }),
            (0x0, 0x0, 0xF, 0xB) => None, // TODO scroll_right();
            (0x0, 0x0, 0xF, 0xC) => None, // TODO scroll_left();
            (0x0, 0x0, 0xF, 0xD) => None, // TODO exit();
            (0x0, 0x0, 0xF, 0xE) => None, // TODO low_res();
            (0x0, 0x0, 0xF, 0xF) => None, // TODO high_res();
            (0x1,   _,   _,   _) => Some(Instruction {
                execute: inst_goto,
                cycles: 1,
                disassembly: "goto NNN;",
            }),
            (0x2,   _,   _,   _) => Some(Instruction {
                execute: inst_call,
                cycles: 1,
                disassembly: "*(0xNNN)();",
            }),
            (0x3,   _,   _,   _) => Some(Instruction {
                execute: inst_if_equal,
                cycles: 1,
                disassembly: "if (Vx == NN) goto next;",
            }),
            (0x4,   _,   _,   _) => Some(Instruction {
                execute: inst_if_inequal,
                cycles: 1,
                disassembly: "if (Vx != NN) goto next;",
            }),
            (0x5,   _,   _,   _) => Some(Instruction {
                execute: inst_if_equal_register,
                cycles: 1,
                disassembly: "if (Vx == Vy) goto next;",
            }),
            (0x6,   _,   _,   _) => Some(Instruction {
                execute: inst_set_register,
                cycles: 1,
                disassembly: "Vx = NN;",
            }),
            (0x7,   _,   _,   _) => Some(Instruction {
                execute: inst_pluseq_immediate,
                cycles: 1,
                disassembly: "Vx += NN;",
            }),
            (0x8,   _,   _, 0x0) => Some(Instruction {
                execute: inst_move_register,
                cycles: 1,
                disassembly: "Vx = Vy;",
            }),
            (0x8,   _,   _, 0x1) => Some(Instruction {
                execute: inst_oreq_register,
                cycles: 1,
                disassembly: "Vx |= Vy;",
            }),
            (0x8,   _,   _, 0x2) => Some(Instruction {
                execute: inst_andeq_register,
                cycles: 1,
                disassembly: "Vx &= Vy;",
            }),
            (0x8,   _,   _, 0x3) => Some(Instruction {
                execute: inst_xoreq_register,
                cycles: 1,
                disassembly: "Vx ^= Vy;",
            }),
            (0x8,   _,   _, 0x4) => Some(Instruction {
                execute: inst_pluseq_register,
                cycles: 1,
                disassembly: "Vx += Vy;",
            }),
            (0x8,   _,   _, 0x5) => Some(Instruction {
                execute: inst_minuseq_register,
                cycles: 1,
                disassembly: "Vx -= Vy;",
            }),
            (0x8,   _,   _, 0x6) => Some(Instruction {
                execute: inst_shift_right,
                cycles: 1,
                disassembly: "Vx = Vy >> 1;",
            }),
            (0x8,   _,   _, 0x7) => Some(Instruction {
                execute: inst_subtract_register,
                cycles: 1,
                disassembly: "Vx = Vy - Vx;",
            }),
            (0x8,   _,   _, 0xE) => Some(Instruction {
                execute: inst_shift_left,
                cycles: 1,
                disassembly: "Vx = Vy << 1;",
            }),
            (0x9,   _,   _,   _) => Some(Instruction {
                execute: inst_if_inequal_register,
                cycles: 1,
                disassembly: "if (Vx != Vy) goto next;",
            }),
            (0xA,   _,   _,   _) => Some(Instruction {
                execute: inst_set_index,
                cycles: 1,
                disassembly: "I = NNN;",
            }),
            (0xB,   _,   _,   _) => Some(Instruction {
                execute: inst_jump_v0,
                cycles: 1,
                disassembly: "PC = V0 + NNN;",
            }),
            (0xC,   _,   _,   _) => Some(Instruction {
                execute: inst_random,
                cycles: 1,
                disassembly: "Vx = rand() & NN;",
            }),
            (0xD,   _,   _,   _) => Some(Instruction {
                execute: inst_draw,
                cycles: 1,
                disassembly: "draw(Vx, Vy, N);",
            }),
            (0xE,   _, 0x9, 0xE) => Some(Instruction {
                execute: inst_key_equal,
                cycles: 1,
                disassembly: "if (key() == Vx) goto next;",
            }),
            (0xE,   _, 0xA, 0x1) => Some(Instruction {
                execute: inst_key_inequal,
                cycles: 1,
                disassembly: "if (key() != Vx) goto next;",
            }),
            (0xF,   _, 0x0, 0x7) => Some(Instruction {
                execute: inst_get_delay,
                cycles: 1,
                disassembly: "Vx = get_delay();",
            }),
            (0xF,   _, 0x0, 0xA) => Some(Instruction {
                execute: inst_get_key,
                cycles: 1,
                disassembly: "Vx = get_key();",
            }),
            (0xF,   _, 0x1, 0x5) => Some(Instruction {
                execute: inst_set_delay,
                cycles: 1,
                disassembly: "delay_timer(Vx);",
            }),
            (0xF,   _, 0x1, 0x8) => Some(Instruction {
                execute: inst_set_sound,
                cycles: 1,
                disassembly: "sound_timer(Vx);",
            }),
            (0xF,   _, 0x1, 0xE) => Some(Instruction {
                execute: inst_add_to_index,
                cycles: 1,
                disassembly: "I += Vx;",
            }),
            (0xF,   _, 0x2, 0x9) => Some(Instruction {
                execute: inst_sprite_addr_index,
                cycles: 1,
                disassembly: "I = sprite_addr[Vx];",
            }),
            (0xF,   _, 0x3, 0x0) => None, // TODO I = digit_addr[Vx];
            (0xF,   _, 0x3, 0x3) => Some(Instruction {
                execute: inst_bcd,
                cycles: 1,
                disassembly: "set_bcd(I, Vx);",
            }),
            (0xF,   _, 0x5, 0x5) => Some(Instruction {
                execute: inst_reg_dump,
                cycles: 1,
                disassembly: "reg_dump(Vx, &I);",
            }),
            (0xF,   _, 0x6, 0x5) => Some(Instruction {
                execute: inst_reg_load,
                cycles: 1,
                disassembly: "reg_load(Vx, &I);",
            }),
            (0xF,   _, 0x7, 0x5) => None, // TODO persist_dump(Vx);
            (0xF,   _, 0x8, 0x5) => None, // TODO persist_load(Vx);
            _ => None
        }
    }
}
