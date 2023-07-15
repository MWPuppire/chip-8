use crate::CPU;

pub(super) fn inst_bcd(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let value = cpu.registers[reg];
    cpu.write_memory_byte(cpu.index + 0, value / 100).unwrap();
    cpu.write_memory_byte(cpu.index + 1, (value / 10) % 10)
        .unwrap();
    cpu.write_memory_byte(cpu.index + 2, value % 10).unwrap();
    0
}

pub(super) fn inst_random(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let and = (inst & 0xFF) as u8;
    cpu.registers[reg] = cpu.random() & and;
    0
}

pub(super) fn inst_nop(_: &mut CPU, _: u16) -> u32 {
    0
}

#[cfg(any(feature = "super-chip", feature = "xo-chip"))]
pub(super) fn inst_exit(cpu: &mut CPU, _: u16) -> u32 {
    cpu.exited = true;
    0
}

#[cfg(feature = "xo-chip")]
pub(super) fn inst_set_audio_buffer(cpu: &mut CPU, _: u16) -> u32 {
    let buf = &cpu.memory[(cpu.index as usize)..];
    cpu.audio.write_pattern(buf);
    0
}

#[cfg(feature = "xo-chip")]
pub(super) fn inst_set_audio_pitch(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let pitch = cpu.registers[reg] as f32;
    cpu.audio.set_pitch(pitch);
    0
}
