use crate::CPU;

#[cfg(feature = "xo-chip")]
#[inline]
fn skip_instruction(cpu: &mut CPU) {
    cpu.pc += 2;
    // skip the double-word instruction in XO-CHIP
    if cpu.mode == crate::Chip8Mode::XoChip && cpu.read_memory_word(cpu.pc) == Ok(0xF000) {
        cpu.pc += 2;
    }
}
#[cfg(not(feature = "xo-chip"))]
#[inline]
fn skip_instruction(cpu: &mut CPU) {
    cpu.pc += 2;
}

pub(super) fn inst_if_equal(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let value = (inst & 0xFF) as u8;
    if cpu.registers[reg] == value {
        skip_instruction(cpu);
    }
    0
}

pub(super) fn inst_if_inequal(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let value = (inst & 0xFF) as u8;
    if cpu.registers[reg] != value {
        skip_instruction(cpu);
    }
    0
}

pub(super) fn inst_if_equal_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let reg_y = (((inst >> 4) & 0xF) as u8).try_into().unwrap();
    if cpu.registers[reg_x] == cpu.registers[reg_y] {
        skip_instruction(cpu);
    }
    0
}

pub(super) fn inst_if_inequal_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let reg_y = (((inst >> 4) & 0xF) as u8).try_into().unwrap();
    if cpu.registers[reg_x] != cpu.registers[reg_y] {
        skip_instruction(cpu);
    }
    0
}
