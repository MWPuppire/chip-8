use crate::register::Register;
use crate::cpu::CPU;

pub fn inst_pluseq_immediate(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let value = (inst & 0xFF) as u8;
    cpu.registers[reg] = cpu.registers[reg].wrapping_add(value);
    0
}

pub fn inst_pluseq_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let reg_y = Register::from_index(((inst >> 4) & 0xF) as u8).unwrap();
    let sum = cpu.registers[reg_x].overflowing_add(cpu.registers[reg_y]);
    cpu.registers[Register::VF] = if sum.1 { 1 } else { 0 };
    cpu.registers[reg_x] = sum.0;
    0
}

pub fn inst_minuseq_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let reg_y = Register::from_index(((inst >> 4) & 0xF) as u8).unwrap();
    let diff = cpu.registers[reg_x].overflowing_sub(cpu.registers[reg_y]);
    cpu.registers[Register::VF] = if diff.1 { 1 } else { 0 };
    cpu.registers[reg_x] = diff.0;
    0
}

pub fn inst_subtract_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let reg_y = Register::from_index(((inst >> 4) & 0xF) as u8).unwrap();
    let diff = cpu.registers[reg_y].overflowing_sub(cpu.registers[reg_x]);
    cpu.registers[Register::VF] = if diff.1 { 1 } else { 0 };
    cpu.registers[reg_x] = diff.0;
    0
}

pub fn inst_add_to_index(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    cpu.index = cpu.index.wrapping_add(cpu.registers[reg] as u16);
    cpu.registers[Register::VF] = if cpu.index > 0xFFF { 1 } else { 0 };
    0
}
