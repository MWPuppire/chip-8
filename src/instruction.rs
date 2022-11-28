use crate::cpu::CPU;

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

pub struct Instruction {
    pub execute: fn(&mut CPU, u16) -> u32,
    pub cycles: u32,
    pub disassembly: &'static str,
}

impl Instruction {
    pub fn lookup(opcode: u16) -> Option<Instruction> {
        match opcode & 0xF000 {
            0x0000 => {
                if opcode == 0x0000 {
                    Some(Instruction {
                        execute: inst_nop,
                        cycles: 1,
                        disassembly: "(void) 0;",
                    })
                } else if (opcode & 0x00F0) == 0x00B0 {
                    Some(Instruction {
                        execute: inst_nop, // TO-DO implement instruction
                        cycles: 1,
                        disassembly: "scroll_up(N);",
                    })
                } else if (opcode & 0x00F0) == 0x00C0 {
                    Some(Instruction {
                        execute: inst_nop, // TO-DO implement instruction
                        cycles: 1,
                        disassembly: "scroll_down(N);"
                    })
                } else if opcode == 0x00E0 {
                    Some(Instruction {
                        execute: inst_clear,
                        cycles: 1,
                        disassembly: "display_clear();",
                    })
                } else if opcode == 0x00EE {
                    Some(Instruction {
                        execute: inst_return,
                        cycles: 1,
                        disassembly: "return;",
                    })
                } else if opcode == 0x00FB {
                    Some(Instruction {
                        execute: inst_nop, // TO-DO implement instruction
                        cycles: 1,
                        disassembly: "scroll_right();",
                    })
                } else if opcode == 0x00FC {
                    Some(Instruction {
                        execute: inst_nop, // TO-DO implement instruction
                        cycles: 1,
                        disassembly: "scroll_left();",
                    })
                } else if opcode == 0x00FD {
                    Some(Instruction {
                        execute: inst_nop, // TO-DO implement instruction
                        cycles: 1,
                        disassembly: "exit();",
                    })
                } else if opcode == 0x00FE {
                    Some(Instruction {
                        execute: inst_nop, // TO-DO implement instruction
                        cycles: 1,
                        disassembly: "low_res();",
                    })
                } else if opcode == 0x00FF {
                    Some(Instruction {
                        execute: inst_nop, // TO-DO implement instruction
                        cycles: 1,
                        disassembly: "high_res();",
                    })
                } else {
                    None
                }
            },
            0x1000 => Some(Instruction {
                execute: inst_goto,
                cycles: 1,
                disassembly: "goto NNN;",
            }),
            0x2000 => Some(Instruction {
                execute: inst_call,
                cycles: 1,
                disassembly: "*(0xNNN)();",
            }),
            0x3000 => Some(Instruction {
                execute: inst_if_equal,
                cycles: 1,
                disassembly: "if (Vx == NN) goto next;",
            }),
            0x4000 => Some(Instruction {
                execute: inst_if_inequal,
                cycles: 1,
                disassembly: "if (Vx != NN) goto next;",
            }),
            0x5000 => Some(Instruction {
                execute: inst_if_equal_register,
                cycles: 1,
                disassembly: "if (Vx == Vy) goto next;",
            }),
            0x6000 => Some(Instruction {
                execute: inst_set_register,
                cycles: 1,
                disassembly: "Vx = NN;",
            }),
            0x7000 => Some(Instruction {
                execute: inst_pluseq_immediate,
                cycles: 1,
                disassembly: "Vx += NN;",
            }),
            0x8000 => match opcode & 0xF {
                0x0 => Some(Instruction {
                    execute: inst_move_register,
                    cycles: 1,
                    disassembly: "Vx = Vy;",
                }),
                0x1 => Some(Instruction {
                    execute: inst_oreq_register,
                    cycles: 1,
                    disassembly: "Vx |= Vy;",
                }),
                0x2 => Some(Instruction {
                    execute: inst_andeq_register,
                    cycles: 1,
                    disassembly: "Vx &= Vy;",
                }),
                0x3 => Some(Instruction {
                    execute: inst_xoreq_register,
                    cycles: 1,
                    disassembly: "Vx ^= Vy;",
                }),
                0x4 => Some(Instruction {
                    execute: inst_pluseq_register,
                    cycles: 1,
                    disassembly: "Vx += Vy;",
                }),
                0x5 => Some(Instruction {
                    execute: inst_minuseq_register,
                    cycles: 1,
                    disassembly: "Vx -= Vy;",
                }),
                0x6 => Some(Instruction {
                    execute: inst_shift_right,
                    cycles: 1,
                    disassembly: "Vx = Vy >> 1;",
                }),
                0x7 => Some(Instruction {
                    execute: inst_subtract_register,
                    cycles: 1,
                    disassembly: "Vx = Vy - Vx;",
                }),
                0xE => Some(Instruction {
                    execute: inst_shift_left,
                    cycles: 1,
                    disassembly: "Vx = Vy << 1;",
                }),
                _ => None,
            },
            0x9000 => Some(Instruction {
                execute: inst_if_inequal_register,
                cycles: 1,
                disassembly: "if (Vx != Vy) goto next;",
            }),
            0xA000 => Some(Instruction {
                execute: inst_set_index,
                cycles: 1,
                disassembly: "I = NNN;",
            }),
            0xB000 => Some(Instruction {
                execute: inst_jump_v0,
                cycles: 1,
                disassembly: "PC = V0 + NNN;",
            }),
            0xC000 => Some(Instruction {
                execute: inst_random,
                cycles: 1,
                disassembly: "Vx = rand() & NN;",
            }),
            0xD000 => Some(Instruction {
                execute: inst_draw,
                cycles: 1,
                disassembly: "draw(Vx, Vy, N);",
            }),
            0xE000 => {
                if opcode & 0xFF == 0x9E {
                    Some(Instruction {
                        execute: inst_key_equal,
                        cycles: 1,
                        disassembly: "if (key() == Vx) goto next;",
                    })
                } else if opcode & 0xFF == 0xA1 {
                    Some(Instruction {
                        execute: inst_key_inequal,
                        cycles: 1,
                        disassembly: "if (key() != Vx) goto next;",
                    })
                } else {
                    None
                }
            },
            0xF000 => match opcode & 0xFF {
                0x07 => Some(Instruction {
                    execute: inst_get_delay,
                    cycles: 1,
                    disassembly: "Vx = get_delay();",
                }),
                0x0A => Some(Instruction {
                    execute: inst_get_key,
                    cycles: 1,
                    disassembly: "Vx = get_key();",
                }),
                0x15 => Some(Instruction {
                    execute: inst_set_delay,
                    cycles: 1,
                    disassembly: "delay_timer(Vx);",
                }),
                0x18 => Some(Instruction {
                    execute: inst_set_sound,
                    cycles: 1,
                    disassembly: "sound_timer(Vx);",
                }),
                0x1E => Some(Instruction {
                    execute: inst_add_to_index,
                    cycles: 1,
                    disassembly: "I += Vx;",
                }),
                0x29 => Some(Instruction {
                    execute: inst_sprite_addr_index,
                    cycles: 1,
                    disassembly: "I = sprite_addr[Vx];",
                }),
                0x30 => Some(Instruction {
                    execute: inst_nop, // TO-DO implement instruction
                    cycles: 1,
                    disassembly: "I = digit_addr[Vx];",
                }),
                0x33 => Some(Instruction {
                    execute: inst_bcd,
                    cycles: 1,
                    disassembly: "set_bcd(I, Vx);",
                }),
                0x55 => Some(Instruction {
                    execute: inst_reg_dump,
                    cycles: 1,
                    disassembly: "reg_dump(Vx, &I);",
                }),
                0x65 => Some(Instruction {
                    execute: inst_reg_load,
                    cycles: 1,
                    disassembly: "reg_load(Vx, &I);",
                }),
                0x75 => Some(Instruction {
                    execute: inst_nop, // TO-DO implement instruction
                    cycles: 1,
                    disassembly: "persist_dump(Vx);",
                }),
                0x85 => Some(Instruction {
                    execute: inst_nop, // TO-DO implement instruction
                    cycles: 1,
                    disassembly: "persist_load(Vx);",
                }),
                _ => None
            },
            _ => None
        }
    }
}
