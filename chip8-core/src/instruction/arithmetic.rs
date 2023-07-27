use crate::Register;
use crate::CPU;

pub(super) fn inst_pluseq_immediate(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let value = (inst & 0xFF) as u8;
    cpu.registers[reg] = cpu.registers[reg].wrapping_add(value);
    0
}

pub(super) fn inst_pluseq_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let reg_y = (((inst >> 4) & 0xF) as u8).try_into().unwrap();
    let sum = cpu.registers[reg_x].overflowing_add(cpu.registers[reg_y]);
    cpu.registers[Register::VF] = if sum.1 { 1 } else { 0 };
    cpu.registers[reg_x] = sum.0;
    0
}

pub(super) fn inst_minuseq_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let reg_y = (((inst >> 4) & 0xF) as u8).try_into().unwrap();
    let diff = cpu.registers[reg_x].overflowing_sub(cpu.registers[reg_y]);
    cpu.registers[Register::VF] = if diff.1 { 0 } else { 1 };
    cpu.registers[reg_x] = diff.0;
    0
}

pub(super) fn inst_subtract_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let reg_y = (((inst >> 4) & 0xF) as u8).try_into().unwrap();
    let diff = cpu.registers[reg_y].overflowing_sub(cpu.registers[reg_x]);
    cpu.registers[Register::VF] = if diff.1 { 0 } else { 1 };
    cpu.registers[reg_x] = diff.0;
    0
}

pub(super) fn inst_add_to_index(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let new_index = cpu.index + cpu.registers[reg] as u16;
    cpu.registers[Register::VF] = (new_index > 0xFFF) as u8;
    cpu.index = new_index & 0xFFF;
    0
}
