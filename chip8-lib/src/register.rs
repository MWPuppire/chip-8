use core::fmt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RegisterOutOfRange;
impl fmt::Display for RegisterOutOfRange {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt("CHIP-8 register index out of range (must be 0x0-0xF)", f)
    }
}
#[cfg(feature = "std")]
impl std::error::Error for RegisterOutOfRange {}

#[derive(enum_map::Enum, Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
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
    pub fn by_name(name: &str) -> Option<Register> {
        if name == "v0" || name == "V0" {
            Some(Register::V0)
        } else if name == "v1" || name == "V1" {
            Some(Register::V1)
        } else if name == "v2" || name == "V2" {
            Some(Register::V2)
        } else if name == "v3" || name == "V3" {
            Some(Register::V3)
        } else if name == "v4" || name == "V4" {
            Some(Register::V4)
        } else if name == "v5" || name == "V5" {
            Some(Register::V5)
        } else if name == "v6" || name == "V6" {
            Some(Register::V6)
        } else if name == "v7" || name == "V7" {
            Some(Register::V7)
        } else if name == "v8" || name == "V8" {
            Some(Register::V8)
        } else if name == "v9" || name == "V9" {
            Some(Register::V9)
        } else if name == "va" || name == "vA" || name == "Va" || name == "VA" {
            Some(Register::VA)
        } else if name == "vb" || name == "vB" || name == "Vb" || name == "VB" {
            Some(Register::VB)
        } else if name == "vc" || name == "vC" || name == "Vc" || name == "VC" {
            Some(Register::VC)
        } else if name == "vd" || name == "vD" || name == "Vd" || name == "VD" {
            Some(Register::VD)
        } else if name == "ve" || name == "vE" || name == "Ve" || name == "VE" {
            Some(Register::VE)
        } else if name == "vf" || name == "vF" || name == "Vf" || name == "VF" {
            Some(Register::VF)
        } else {
            None
        }
    }
}

impl TryFrom<u8> for Register {
    type Error = RegisterOutOfRange;
    fn try_from(item: u8) -> Result<Self, RegisterOutOfRange> {
        match item {
            0x0 => Ok(Self::V0),
            0x1 => Ok(Self::V1),
            0x2 => Ok(Self::V2),
            0x3 => Ok(Self::V3),
            0x4 => Ok(Self::V4),
            0x5 => Ok(Self::V5),
            0x6 => Ok(Self::V6),
            0x7 => Ok(Self::V7),
            0x8 => Ok(Self::V8),
            0x9 => Ok(Self::V9),
            0xA => Ok(Self::VA),
            0xB => Ok(Self::VB),
            0xC => Ok(Self::VC),
            0xD => Ok(Self::VD),
            0xE => Ok(Self::VE),
            0xF => Ok(Self::VF),
            _ => Err(RegisterOutOfRange),
        }
    }
}

impl From<Register> for u8 {
    #[inline]
    fn from(item: Register) -> Self {
        item as u8
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(
            match self {
                Self::V0 => "V0",
                Self::V1 => "V1",
                Self::V2 => "V2",
                Self::V3 => "V3",
                Self::V4 => "V4",
                Self::V5 => "V5",
                Self::V6 => "V6",
                Self::V7 => "V7",
                Self::V8 => "V8",
                Self::V9 => "V9",
                Self::VA => "VA",
                Self::VB => "VB",
                Self::VC => "VC",
                Self::VD => "VD",
                Self::VE => "VE",
                Self::VF => "VF",
            },
            f,
        )
    }
}
