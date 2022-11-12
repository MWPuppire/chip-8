use crate::register::Register;
use crate::cpu::CPU;
use crate::display;

pub fn inst_draw(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let reg_y = Register::from_index(((inst >> 4) & 0xF) as u8).unwrap();
    let x = cpu.registers[reg_x];
    let y = cpu.registers[reg_y];
    let n = (inst & 0xF) as u8;
    let mut flag = false;
    for (i, idx) in (0..n).zip(cpu.index..) {
        let byte = cpu.read_memory_byte(idx).unwrap();
        for bit in 0..8 {
            if (byte << bit) & 128 == 128 {
                flag |= cpu.screen.write_pixel((x + bit) % display::SCREEN_WIDTH,
                    (y + i) % display::SCREEN_HEIGHT);
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
