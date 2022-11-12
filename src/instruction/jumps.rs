use crate::register::Register;
use crate::cpu::CPU;

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
    let offset = cpu.registers[Register::V0] as u16;
    cpu.pc = base + offset;
    0
}