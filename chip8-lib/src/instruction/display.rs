use crate::Register;
use crate::CPU;
use crate::display;

#[cfg(feature = "cosmac")]
pub fn inst_draw_cosmac(cpu: &mut CPU, inst: u16) -> u32 {
    cpu.vblank_wait = true;
    let reg_x = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let reg_y = (((inst >> 4) & 0xF) as u8).try_into().unwrap();
    let x = cpu.registers[reg_x] % display::LOWRES_SCREEN_WIDTH as u8;
    let y = cpu.registers[reg_y] % display::LOWRES_SCREEN_HEIGHT as u8;
    let n = (inst & 0xF) as u8;
    let mut flag = false;
    for (i, idx) in (0..n).zip(cpu.index..) {
        let byte = cpu.read_memory_byte(idx).unwrap();
        for bit in 0..8 {
            if (byte << bit) & 128 == 128 {
                flag |= cpu.screen.write_to_screen(x + bit, y + i);
            }
        }
    }
    cpu.registers[Register::VF] = if flag { 1 } else { 0 };
    0
}
#[cfg(feature = "super-chip")]
pub fn inst_draw_schip(cpu: &mut CPU, inst: u16) -> u32 {
    let dimensions = if cpu.screen.high_res {
        display::HIGHRES_SCREEN_DIMENSIONS
    } else {
        display::LOWRES_SCREEN_DIMENSIONS
    };
    let reg_x = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let reg_y = (((inst >> 4) & 0xF) as u8).try_into().unwrap();
    let x = cpu.registers[reg_x] % dimensions.0 as u8;
    let y = cpu.registers[reg_y] % dimensions.1 as u8;
    let n = (inst & 0xF) as u8;
    let mut flag = false;
    for (i, idx) in (0..n).zip(cpu.index..) {
        let byte = cpu.read_memory_byte(idx).unwrap();
        for bit in 0..8 {
            if (byte << bit) & 128 == 128 {
                flag |= cpu.screen.write_to_screen(x + bit, y + i);
            }
        }
    }
    cpu.registers[Register::VF] = if flag { 1 } else { 0 };
    0
}
#[cfg(feature = "xo-chip")]
pub fn inst_draw_xochip(cpu: &mut CPU, inst: u16) -> u32 {
    let dimensions = if cpu.screen.high_res {
        display::HIGHRES_SCREEN_DIMENSIONS
    } else {
        display::LOWRES_SCREEN_DIMENSIONS
    };
    let reg_x = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let reg_y = (((inst >> 4) & 0xF) as u8).try_into().unwrap();
    let x = cpu.registers[reg_x] % dimensions.0 as u8;
    let y = cpu.registers[reg_y] % dimensions.1 as u8;
    let n = (inst & 0xF) as u8;
    let mut flag = false;
    for (i, idx) in (0..n).zip(cpu.index..) {
        let byte = cpu.read_memory_byte(idx).unwrap();
        for bit in 0..8 {
            if (byte << bit) & 128 == 128 {
                flag |= cpu.screen.write_to_screen(
                    (x + bit) % dimensions.0 as u8,
                    (y + i) % dimensions.1 as u8
                );
            }
        }
    }
    cpu.registers[Register::VF] = if flag { 1 } else { 0 };
    0
}

pub fn inst_clear(cpu: &mut CPU, _: u16) -> u32 {
    cpu.screen.clear();
    0
}

cfg_if::cfg_if! {
    if #[cfg(any(feature = "super-chip", feature = "xo-chip"))] {
        pub fn inst_low_res(cpu: &mut CPU, _: u16) -> u32 {
            cpu.screen.high_res = false;
            0
        }
        pub fn inst_high_res(cpu: &mut CPU, _: u16) -> u32 {
            cpu.screen.high_res = true;
            0
        }
    }
}
