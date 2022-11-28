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

    pub fn by_name(name: &str) -> Option<Register> {
        let name = name.to_ascii_lowercase();
        if name == "v0" {
            Some(Register::V0)
        } else if name == "v1" {
            Some(Register::V1)
        } else if name == "v2" {
            Some(Register::V2)
        } else if name == "v3" {
            Some(Register::V3)
        } else if name == "v4" {
            Some(Register::V4)
        } else if name == "v5" {
            Some(Register::V5)
        } else if name == "v6" {
            Some(Register::V6)
        } else if name == "v7" {
            Some(Register::V7)
        } else if name == "v8" {
            Some(Register::V8)
        } else if name == "v9" {
            Some(Register::V9)
        } else if name == "vA" {
            Some(Register::VA)
        } else if name == "vB" {
            Some(Register::VB)
        } else if name == "vC" {
            Some(Register::VC)
        } else if name == "vD" {
            Some(Register::VD)
        } else if name == "vE" {
            Some(Register::VE)
        } else if name == "vF" {
            Some(Register::VF)
        } else {
            None
        }
    }
}
