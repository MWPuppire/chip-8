use crate::Register;
use crate::CPU;

pub fn inst_bcd(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let value = cpu.registers[reg];
    cpu.write_memory_byte(cpu.index + 0, value / 100).unwrap();
    cpu.write_memory_byte(cpu.index + 1, (value / 10) % 10).unwrap();
    cpu.write_memory_byte(cpu.index + 2, value % 10).unwrap();
    0
}

pub fn inst_random(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let and = (inst & 0xFF) as u8;
    cpu.registers[reg] = rand::random::<u8>() & and;
    0
}

pub fn inst_nop(_: &mut CPU, _: u16) -> u32 { 0 }
