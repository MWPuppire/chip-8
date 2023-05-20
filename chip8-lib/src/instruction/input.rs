use crate::CPU;

pub fn inst_key_equal(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    if cpu.is_key_down(cpu.registers[reg]) {
        cpu.pc += 2;
    }
    0
}

pub fn inst_key_inequal(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    if !cpu.is_key_down(cpu.registers[reg]) {
        cpu.pc += 2;
    }
    0
}

pub fn inst_get_key(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    cpu.await_key(reg);
    0
}
