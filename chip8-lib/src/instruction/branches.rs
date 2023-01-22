use crate::Register;
use crate::CPU;

pub fn inst_if_equal(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let value = (inst & 0xFF) as u8;
    if cpu.registers[reg] == value {
        cpu.pc += 2;
    }
    0
}

pub fn inst_if_inequal(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let value = (inst & 0xFF) as u8;
    if cpu.registers[reg] != value {
        cpu.pc += 2;
    }
    0
}

pub fn inst_if_equal_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let reg_y = Register::from_index(((inst >> 4) & 0xF) as u8).unwrap();
    if cpu.registers[reg_x] == cpu.registers[reg_y] {
        cpu.pc += 2;
    }
    0
}

pub fn inst_if_inequal_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let reg_y = Register::from_index(((inst >> 4) & 0xF) as u8).unwrap();
    if cpu.registers[reg_x] != cpu.registers[reg_y] {
        cpu.pc += 2;
    }
    0
}
