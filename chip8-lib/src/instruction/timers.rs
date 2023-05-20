use crate::CPU;

pub fn inst_get_delay(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    cpu.registers[reg] = cpu.delay_timer;
    0
}

pub fn inst_set_delay(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    cpu.delay_timer = cpu.registers[reg];
    0
}

pub fn inst_set_sound(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    cpu.sound_timer = cpu.registers[reg];
    0
}
