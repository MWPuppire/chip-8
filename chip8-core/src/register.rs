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
        match name {
            "v0" | "V0" => Some(Register::V0),
            "v1" | "V1" => Some(Register::V1),
            "v2" | "V2" => Some(Register::V2),
            "v3" | "V3" => Some(Register::V3),
            "v4" | "V4" => Some(Register::V4),
            "v5" | "V5" => Some(Register::V5),
            "v6" | "V6" => Some(Register::V6),
            "v7" | "V7" => Some(Register::V7),
            "v8" | "V8" => Some(Register::V8),
            "v9" | "V9" => Some(Register::V9),
            "vA" | "VA" => Some(Register::VA),
            "vB" | "VB" => Some(Register::VB),
            "vC" | "VC" => Some(Register::VC),
            "vD" | "VD" => Some(Register::VD),
            "vE" | "VE" => Some(Register::VE),
            "vF" | "VF" => Some(Register::VF),
            _ => None,
        }
    }
}

impl core::str::FromStr for Register {
    type Err = ();
    #[inline]
    fn from_str(s: &str) -> Result<Self, ()> {
        Self::by_name(s).ok_or(())
    }
}

impl TryFrom<u8> for Register {
    type Error = RegisterOutOfRange;
    #[inline]
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
