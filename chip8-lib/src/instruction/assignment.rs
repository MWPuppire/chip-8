use crate::CPU;

pub fn inst_set_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let value = (inst & 0xFF) as u8;
    cpu.registers[reg] = value;
    0
}

pub fn inst_set_index(cpu: &mut CPU, inst: u16) -> u32 {
    cpu.index = inst & 0xFFF;
    0
}

#[cfg(any(feature = "cosmac", feature = "xo-chip"))]
pub fn inst_reg_dump(cpu: &mut CPU, inst: u16) -> u32 {
    let max = (((inst >> 8) & 0xF) + 1) as u8;
    for i in 0..max {
        let reg = i.try_into().unwrap();
        cpu.write_memory_byte(cpu.index + i as u16, cpu.registers[reg]).unwrap();
    }
    cpu.index = (cpu.index + max as u16) & 0xFFF;
    0
}

#[cfg(feature = "super-chip")]
pub fn inst_reg_dump_schip(cpu: &mut CPU, inst: u16) -> u32 {
    let max = (((inst >> 8) & 0xF) + 1) as u8;
    for i in 0..max {
        let reg = i.try_into().unwrap();
        cpu.write_memory_byte(cpu.index + i as u16, cpu.registers[reg]).unwrap();
    }
    0
}

#[cfg(any(feature = "cosmac", feature = "xo-chip"))]
pub fn inst_reg_load(cpu: &mut CPU, inst: u16) -> u32 {
    let max = (((inst >> 8) & 0xF) + 1) as u8;
    for i in 0..max {
        let reg = i.try_into().unwrap();
        cpu.registers[reg] = cpu.read_memory_byte(cpu.index + i as u16).unwrap();
    }
    cpu.index = (cpu.index + max as u16) & 0xFFF;
    0
}

#[cfg(feature = "super-chip")]
pub fn inst_reg_load_schip(cpu: &mut CPU, inst: u16) -> u32 {
    let max = (((inst >> 8) & 0xF) + 1) as u8;
    for i in 0..max {
        let reg = i.try_into().unwrap();
        cpu.registers[reg] = cpu.read_memory_byte(cpu.index + i as u16).unwrap();
    }
    0
}

pub fn inst_move_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let reg_y = (((inst >> 4) & 0xF) as u8).try_into().unwrap();
    cpu.registers[reg_x] = cpu.registers[reg_y];
    0
}

pub fn inst_sprite_addr_index(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = (((inst >> 8) & 0xF) as u8).try_into().unwrap();
    let value = cpu.registers[reg] & 0xF;
    cpu.index = crate::font::SPRITE_ADDR[value as usize];
    0
}
