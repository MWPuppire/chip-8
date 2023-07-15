// The imports are all flagged unused if no CHIP-8 feature is enabled; in the
// interest of keeping a more useful error message, that warning is disabled.
#![cfg_attr(
    not(any(feature = "cosmac", feature = "super-chip", feature = "xo-chip")),
    allow(unused_imports)
)]

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

pub(crate) type OpcodeExecute = fn(&mut CPU, u16) -> u32;

pub(crate) struct Instruction {
    #[cfg(feature = "cosmac")]
    pub(crate) cosmac: Option<OpcodeExecute>,
    #[cfg(feature = "super-chip")]
    pub(crate) schip: Option<OpcodeExecute>,
    #[cfg(feature = "xo-chip")]
    pub(crate) xochip: Option<OpcodeExecute>,
    pub(crate) cycles: u32,
    pub(crate) disassembly: &'static str,
}

#[allow(dead_code)]
fn inst_todo(_: &mut CPU, code: u16) -> u32 {
    let op = Instruction::disassemble(code).unwrap();
    todo!("{}", op);
}

macro_rules! make_instruction {
    ($cosmac:expr, $schip:expr, $xochip:expr, $cycles:literal, $disassembly:literal $(,)?) => {
        Some(Instruction {
            #[cfg(feature = "cosmac")]
            cosmac: $cosmac,
            #[cfg(feature = "super-chip")]
            schip: $schip,
            #[cfg(feature = "xo-chip")]
            xochip: $xochip,
            cycles: $cycles,
            disassembly: $disassembly,
        })
    };
}

impl Instruction {
    pub(crate) fn lookup(opcode: u16) -> Option<Instruction> {
        match (
            (opcode >> 12) & 0xF,
            (opcode >> 8) & 0xF,
            (opcode >> 4) & 0xF,
            opcode & 0xF,
        ) {
            (0x0, 0x0, 0x0, 0x0) => make_instruction!(
                Some(inst_nop),
                Some(inst_nop),
                Some(inst_nop),
                1,
                "(void) 0;",
            ),
            (0x0, 0x0, 0xC, _) => {
                make_instruction!(None, Some(inst_todo), Some(inst_todo), 1, "scroll_down(N);",)
            }
            (0x0, 0x0, 0xD, _) => {
                make_instruction!(None, None, Some(inst_todo), 1, "scroll_up(N);",)
            }
            (0x0, 0x0, 0xE, 0x0) => make_instruction!(
                Some(inst_clear),
                Some(inst_clear),
                Some(inst_clear),
                1,
                "display_clear();",
            ),
            (0x0, 0x0, 0xE, 0xE) => make_instruction!(
                Some(inst_return),
                Some(inst_return),
                Some(inst_return),
                1,
                "return;",
            ),
            (0x0, 0x0, 0xF, 0xB) => {
                make_instruction!(None, Some(inst_todo), Some(inst_todo), 1, "scroll_right();",)
            }
            (0x0, 0x0, 0xF, 0xC) => {
                make_instruction!(None, Some(inst_todo), Some(inst_todo), 1, "scroll_left();",)
            }
            (0x0, 0x0, 0xF, 0xD) => {
                make_instruction!(None, Some(inst_exit), Some(inst_exit), 1, "exit();",)
            }
            (0x0, 0x0, 0xF, 0xE) => make_instruction!(
                None,
                Some(inst_low_res),
                Some(inst_low_res),
                1,
                "low_res();",
            ),
            (0x0, 0x0, 0xF, 0xF) => make_instruction!(
                None,
                Some(inst_high_res),
                Some(inst_high_res),
                1,
                "high_res();",
            ),
            (0x1, _, _, _) => make_instruction!(
                Some(inst_goto),
                Some(inst_goto),
                Some(inst_goto),
                1,
                "goto NNN;",
            ),
            (0x2, _, _, _) => make_instruction!(
                Some(inst_call),
                Some(inst_call),
                Some(inst_call),
                1,
                "*(0xNNN)();",
            ),
            (0x3, _, _, _) => make_instruction!(
                Some(inst_if_equal),
                Some(inst_if_equal),
                Some(inst_if_equal),
                1,
                "if (Vx == NN) goto next;",
            ),
            (0x4, _, _, _) => make_instruction!(
                Some(inst_if_inequal),
                Some(inst_if_inequal),
                Some(inst_if_inequal),
                1,
                "if (Vx != NN) goto next;",
            ),
            (0x5, _, _, 0x0) => make_instruction!(
                Some(inst_if_equal_register),
                Some(inst_if_equal_register),
                Some(inst_if_equal_register),
                1,
                "if (Vx == Vy) goto next;",
            ),
            (0x5, _, _, 0x2) => {
                make_instruction!(
                    None,
                    None,
                    Some(inst_reg_dump_xy),
                    1,
                    "reg_dump(Vx, Vy, &I);",
                )
            }
            (0x5, _, _, 0x3) => {
                make_instruction!(
                    None,
                    None,
                    Some(inst_reg_load_xy),
                    1,
                    "reg_load(Vx, Vy, &I);",
                )
            }
            (0x6, _, _, _) => make_instruction!(
                Some(inst_set_register),
                Some(inst_set_register),
                Some(inst_set_register),
                1,
                "Vx = NN;",
            ),
            (0x7, _, _, _) => make_instruction!(
                Some(inst_pluseq_immediate),
                Some(inst_pluseq_immediate),
                Some(inst_pluseq_immediate),
                1,
                "Vx += NN;",
            ),
            (0x8, _, _, 0x0) => make_instruction!(
                Some(inst_move_register),
                Some(inst_move_register),
                Some(inst_move_register),
                1,
                "Vx = Vy;",
            ),
            (0x8, _, _, 0x1) => make_instruction!(
                Some(inst_oreq_register_cosmac),
                Some(inst_oreq_register),
                Some(inst_oreq_register),
                1,
                "Vx |= Vy;",
            ),
            (0x8, _, _, 0x2) => make_instruction!(
                Some(inst_andeq_register_cosmac),
                Some(inst_andeq_register),
                Some(inst_andeq_register),
                1,
                "Vx &= Vy;",
            ),
            (0x8, _, _, 0x3) => make_instruction!(
                Some(inst_xoreq_register_cosmac),
                Some(inst_xoreq_register),
                Some(inst_xoreq_register),
                1,
                "Vx ^= Vy;",
            ),
            (0x8, _, _, 0x4) => make_instruction!(
                Some(inst_pluseq_register),
                Some(inst_pluseq_register),
                Some(inst_pluseq_register),
                1,
                "Vx += Vy;",
            ),
            (0x8, _, _, 0x5) => make_instruction!(
                Some(inst_minuseq_register),
                Some(inst_minuseq_register),
                Some(inst_minuseq_register),
                1,
                "Vx -= Vy;",
            ),
            (0x8, _, _, 0x6) => make_instruction!(
                Some(inst_shift_right),
                Some(inst_shift_right_schip),
                Some(inst_shift_right),
                1,
                "Vx = Vy >> 1;",
            ),
            (0x8, _, _, 0x7) => make_instruction!(
                Some(inst_subtract_register),
                Some(inst_subtract_register),
                Some(inst_subtract_register),
                1,
                "Vx = Vy - Vx;",
            ),
            (0x8, _, _, 0xE) => make_instruction!(
                Some(inst_shift_left),
                Some(inst_shift_left_schip),
                Some(inst_shift_left),
                1,
                "Vx = Vy << 1;",
            ),
            (0x9, _, _, _) => make_instruction!(
                Some(inst_if_inequal_register),
                Some(inst_if_inequal_register),
                Some(inst_if_inequal_register),
                1,
                "if (Vx != Vy) goto next;",
            ),
            (0xA, _, _, _) => make_instruction!(
                Some(inst_set_index),
                Some(inst_set_index),
                Some(inst_set_index),
                1,
                "I = NNN;",
            ),
            (0xB, _, _, _) => make_instruction!(
                Some(inst_jump_v0),
                Some(inst_jump_v0_schip),
                Some(inst_jump_v0),
                1,
                "PC = V0 + NNN;",
            ),
            (0xC, _, _, _) => make_instruction!(
                Some(inst_random),
                Some(inst_random),
                Some(inst_random),
                1,
                "Vx = rand() & NN;",
            ),
            (0xD, _, _, _) => make_instruction!(
                Some(inst_draw_cosmac),
                Some(inst_draw_schip),
                Some(inst_draw_xochip),
                1,
                "draw(Vx, Vy, N);",
            ),
            (0xE, _, 0x9, 0xE) => make_instruction!(
                Some(inst_key_equal),
                Some(inst_key_equal),
                Some(inst_key_equal),
                1,
                "if (key() == Vx) goto next;",
            ),
            (0xE, _, 0xA, 0x1) => make_instruction!(
                Some(inst_key_inequal),
                Some(inst_key_inequal),
                Some(inst_key_inequal),
                1,
                "if (key() != Vx) goto next;",
            ),
            (0xF, 0x0, 0x0, 0x0) => make_instruction!(
                None,
                None,
                Some(inst_set_long_index),
                1,
                "I = read_and_skip_next_word();",
            ),
            (0xF, _, 0x0, 0x1) => {
                make_instruction!(None, None, Some(inst_todo), 1, "set_drawing_plane(N);",)
            }
            (0xF, _, 0x0, 0x2) => {
                make_instruction!(
                    None,
                    None,
                    Some(inst_set_audio_buffer),
                    1,
                    "load_audio_pattern(I);",
                )
            }
            (0xF, _, 0x0, 0x7) => make_instruction!(
                Some(inst_get_delay),
                Some(inst_get_delay),
                Some(inst_get_delay),
                1,
                "Vx = get_delay();",
            ),
            (0xF, _, 0x0, 0xA) => make_instruction!(
                Some(inst_get_key),
                Some(inst_get_key),
                Some(inst_get_key),
                1,
                "Vx = get_key();",
            ),
            (0xF, _, 0x1, 0x5) => make_instruction!(
                Some(inst_set_delay),
                Some(inst_set_delay),
                Some(inst_set_delay),
                1,
                "delay_timer(Vx);",
            ),
            (0xF, _, 0x1, 0x8) => make_instruction!(
                Some(inst_set_sound),
                Some(inst_set_sound),
                Some(inst_set_sound),
                1,
                "sound_timer(Vx);",
            ),
            (0xF, _, 0x1, 0xE) => make_instruction!(
                Some(inst_add_to_index),
                Some(inst_add_to_index),
                Some(inst_add_to_index),
                1,
                "I += Vx;",
            ),
            (0xF, _, 0x2, 0x9) => make_instruction!(
                Some(inst_sprite_addr_index),
                Some(inst_sprite_addr_index),
                Some(inst_sprite_addr_index),
                1,
                "I = sprite_addr[Vx];",
            ),
            (0xF, _, 0x3, 0x0) => make_instruction!(
                None,
                Some(inst_todo),
                Some(inst_todo),
                1,
                "I = digit_addr[Vx];",
            ),
            (0xF, _, 0x3, 0x3) => make_instruction!(
                Some(inst_bcd),
                Some(inst_bcd),
                Some(inst_bcd),
                1,
                "set_bcd(I, Vx);",
            ),
            (0xF, _, 0x3, 0xA) => {
                make_instruction!(
                    None,
                    None,
                    Some(inst_set_audio_pitch),
                    1,
                    "set_audio_hertz(Vx);",
                )
            }
            (0xF, _, 0x5, 0x5) => make_instruction!(
                Some(inst_reg_dump),
                Some(inst_reg_dump_schip),
                Some(inst_reg_dump),
                1,
                "reg_dump(V0, Vx, &I);",
            ),
            (0xF, _, 0x6, 0x5) => make_instruction!(
                Some(inst_reg_load),
                Some(inst_reg_load_schip),
                Some(inst_reg_load),
                1,
                "reg_load(V0, Vx, &I);",
            ),
            (0xF, _, 0x7, 0x5) => make_instruction!(
                None,
                Some(inst_todo),
                Some(inst_todo),
                1,
                "persist_dump(Vx);",
            ),
            (0xF, _, 0x8, 0x5) => make_instruction!(
                None,
                Some(inst_todo),
                Some(inst_todo),
                1,
                "persist_load(Vx);",
            ),
            _ => None,
        }
    }
    pub(crate) fn disassemble(opcode: u16) -> Option<&'static str> {
        Self::lookup(opcode).map(|x| x.disassembly)
    }
}
