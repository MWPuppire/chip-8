use crate::Register;
use crate::CPU;

pub fn inst_goto(cpu: &mut CPU, inst: u16) -> u32 {
    cpu.pc = inst & 0xFFF;
    0
}

pub fn inst_call(cpu: &mut CPU, inst: u16) -> u32 {
    cpu.call_routine(inst & 0xFFF);
    0
}

pub fn inst_return(cpu: &mut CPU, _: u16) -> u32 {
    cpu.return_routine();
    0
}

pub fn inst_jump_v0(cpu: &mut CPU, inst: u16) -> u32 {
    let base = inst & 0xFFF;
    let reg = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    cpu.pc = base + cpu.registers[reg] as u16;
    0
}
