use crate::register::Register;
use crate::cpu::CPU;

pub fn inst_oreq_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let reg_y = Register::from_index(((inst >> 4) & 0xF) as u8).unwrap();
    cpu.registers[reg_x] = cpu.registers[reg_x] | cpu.registers[reg_y];
    cpu.registers[Register::VF] = 0;
    0
}

pub fn inst_andeq_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let reg_y = Register::from_index(((inst >> 4) & 0xF) as u8).unwrap();
    cpu.registers[reg_x] = cpu.registers[reg_x] & cpu.registers[reg_y];
    cpu.registers[Register::VF] = 0;
    0
}

pub fn inst_xoreq_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let reg_y = Register::from_index(((inst >> 4) & 0xF) as u8).unwrap();
    cpu.registers[reg_x] = cpu.registers[reg_x] ^ cpu.registers[reg_y];
    cpu.registers[Register::VF] = 0;
    0
}

pub fn inst_shift_right(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let reg_y = Register::from_index(((inst >> 4) & 0xF) as u8).unwrap();
    cpu.registers[reg_x] = cpu.registers[reg_y] >> 1;
    cpu.registers[Register::VF] = cpu.registers[reg_y] & 1;
    0
}

pub fn inst_shift_left(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let reg_y = Register::from_index(((inst >> 4) & 0xF) as u8).unwrap();
    cpu.registers[reg_x] = cpu.registers[reg_y] << 1;
    cpu.registers[Register::VF] = cpu.registers[reg_y] >> 7;
    0
}
