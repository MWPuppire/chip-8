#[derive(enum_map::Enum, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
}

impl Register {
    pub fn from_index(idx: u8) -> Option<Register> {
        match idx {
            0x0 => Some(Register::V0),
            0x1 => Some(Register::V1),
            0x2 => Some(Register::V2),
            0x3 => Some(Register::V3),
            0x4 => Some(Register::V4),
            0x5 => Some(Register::V5),
            0x6 => Some(Register::V6),
            0x7 => Some(Register::V7),
            0x8 => Some(Register::V8),
            0x9 => Some(Register::V9),
            0xA => Some(Register::VA),
            0xB => Some(Register::VB),
            0xC => Some(Register::VC),
            0xD => Some(Register::VD),
            0xE => Some(Register::VE),
            0xF => Some(Register::VF),
            _ => None
        }
    }
}
