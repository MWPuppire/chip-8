use crate::CPU;

pub(super) fn inst_goto(cpu: &mut CPU, inst: u16) -> u32 {
    cpu.pc = inst & 0xFFF;
    0
}

pub(super) fn inst_call(cpu: &mut CPU, inst: u16) -> u32 {
    cpu.call_routine(inst & 0xFFF);
    0
}

pub(super) fn inst_return(cpu: &mut CPU, _: u16) -> u32 {
    cpu.return_routine();
    0
}

#[cfg(any(feature = "cosmac", feature = "xo-chip"))]
pub(super) fn inst_jump_v0(cpu: &mut CPU, inst: u16) -> u32 {
    let base = inst & 0xFFF;
    cpu.pc = base + cpu.registers[crate::Register::V0] as u16;
    0
}

#[cfg(feature = "super-chip")]
pub(super) fn inst_jump_v0_schip(cpu: &mut CPU, inst: u16) -> u32 {
    let base = inst & 0xFFF;
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    cpu.pc = base + cpu.registers[reg] as u16;
    0
}
