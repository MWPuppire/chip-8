use crate::Register;
use crate::CPU;

#[cfg(feature = "cosmac")]
pub fn inst_oreq_register_cosmac(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let reg_y = (((inst >> 4) & 0xF) as u8).try_into().unwrap();
    cpu.registers[reg_x] |= cpu.registers[reg_y];
    cpu.registers[Register::VF] = 0;
    0
}

#[cfg(any(feature = "super-chip", feature = "xo-chip"))]
pub fn inst_oreq_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let reg_y = (((inst >> 4) & 0xF) as u8).try_into().unwrap();
    cpu.registers[reg_x] |= cpu.registers[reg_y];
    0
}

#[cfg(feature = "cosmac")]
pub fn inst_andeq_register_cosmac(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let reg_y = (((inst >> 4) & 0xF) as u8).try_into().unwrap();
    cpu.registers[reg_x] &= cpu.registers[reg_y];
    cpu.registers[Register::VF] = 0;
    0
}

#[cfg(any(feature = "super-chip", feature = "xo-chip"))]
pub fn inst_andeq_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let reg_y = (((inst >> 4) & 0xF) as u8).try_into().unwrap();
    cpu.registers[reg_x] &= cpu.registers[reg_y];
    0
}

#[cfg(feature = "cosmac")]
pub fn inst_xoreq_register_cosmac(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let reg_y = (((inst >> 4) & 0xF) as u8).try_into().unwrap();
    cpu.registers[reg_x] ^= cpu.registers[reg_y];
    cpu.registers[Register::VF] = 0;
    0
}

#[cfg(any(feature = "super-chip", feature = "xo-chip"))]
pub fn inst_xoreq_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let reg_y = (((inst >> 4) & 0xF) as u8).try_into().unwrap();
    cpu.registers[reg_x] ^= cpu.registers[reg_y];
    0
}

#[cfg(any(feature = "cosmac", feature = "xo-chip"))]
pub fn inst_shift_right(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let in_reg = (((inst >> 4) & 0xF) as u8).try_into().unwrap();
    let value = cpu.registers[in_reg];
    cpu.registers[reg] = value >> 1;
    cpu.registers[Register::VF] = value & 1;
    0
}

#[cfg(feature = "super-chip")]
pub fn inst_shift_right_schip(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let value = cpu.registers[reg];
    cpu.registers[reg] = value >> 1;
    cpu.registers[Register::VF] = value & 1;
    0
}

#[cfg(any(feature = "cosmac", feature = "xo-chip"))]
pub fn inst_shift_left(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let in_reg = (((inst >> 4) & 0xF) as u8).try_into().unwrap();
    let value = cpu.registers[in_reg];
    cpu.registers[reg] = value << 1;
    cpu.registers[Register::VF] = value >> 7;
    0
}

#[cfg(feature = "super-chip")]
pub fn inst_shift_left_schip(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let value = cpu.registers[reg];
    cpu.registers[reg] = value << 1;
    cpu.registers[Register::VF] = value >> 7;
    0
}
