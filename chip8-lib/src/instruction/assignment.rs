use crate::Register;
use crate::CPU;

pub fn inst_set_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let value = (inst & 0xFF) as u8;
    cpu.registers[reg] = value;
    0
}

pub fn inst_set_index(cpu: &mut CPU, inst: u16) -> u32 {
    cpu.index = inst & 0xFFF;
    0
}

pub fn inst_reg_dump(cpu: &mut CPU, inst: u16) -> u32 {
    let max = ((inst >> 8) & 0xF + 1) as u8;
    for i in 0..max {
        let reg = Register::from_index(i).unwrap();
        cpu.write_memory_byte(cpu.index + i as u16, cpu.registers[reg]).unwrap();
    }
    0
}

pub fn inst_reg_load(cpu: &mut CPU, inst: u16) -> u32 {
    let max = ((inst >> 8) & 0xF + 1) as u8;
    for i in 0..max {
        let reg = Register::from_index(i).unwrap();
        cpu.registers[reg] = cpu.read_memory_byte(cpu.index + i as u16).unwrap();
    }
    0
}

pub fn inst_move_register(cpu: &mut CPU, inst: u16) -> u32 {
    let reg_x = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let reg_y = Register::from_index(((inst >> 4) & 0xF) as u8).unwrap();
    cpu.registers[reg_x] = cpu.registers[reg_y];
    0
}

pub fn inst_sprite_addr_index(cpu: &mut CPU, inst: u16) -> u32 {
    let reg = Register::from_index(((inst >> 8) & 0xF) as u8).unwrap();
    let value = cpu.registers[reg] & 0xF;
    cpu.index = crate::font::SPRITE_ADDR[value as usize];
    // TODO raise error instead of no-op if out of bounds?
    0
}
