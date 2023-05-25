use core::fmt;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    UnknownOpcode(u16),
    Breakpoint(u16),
    InvalidFile,
    OutOfBounds,
    NoRomLoaded,
    Exited,
    NotDefined(&'static str),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownOpcode(op) => write!(f, "unknown opcode {:04x}", op),
            Self::Breakpoint(at) => write!(f, "reached breakpoint at {:04x}", at),
            Self::InvalidFile => write!(f, "supplied file is not a valid ROM"),
            Self::OutOfBounds => write!(f, "attempted an out-of-bounds memory access"),
            Self::NoRomLoaded => write!(f, "no ROM is loaded to execute from"),
            Self::Exited => write!(f, "program has exited"),
            Self::NotDefined(op) => write!(f, "instruction {} isn't defined for this mode", op),
        }
    }
}
#[cfg(feature = "std")]
impl std::error::Error for Error { }

cfg_if::cfg_if! {
    if #[cfg(all(feature = "cosmac", feature = "super-chip", feature = "xo-chip"))] {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub enum Chip8Mode {
            Cosmac = 0,
            SuperChip = 1,
            XoChip = 2,
        }
    } else if #[cfg(all(feature = "cosmac", feature = "super-chip"))] {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub enum Chip8Mode {
            Cosmac = 0,
            SuperChip = 1,
        }
    } else if #[cfg(all(feature = "cosmac", feature = "xo-chip"))] {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub enum Chip8Mode {
            Cosmac = 0,
            XoChip = 2,
        }
    } else if #[cfg(all(feature = "super-chip", feature = "xo-chip"))] {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub enum Chip8Mode {
            SuperChip = 1,
            XoChip = 2,
        }
    } else if #[cfg(feature = "cosmac")] {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub enum Chip8Mode {
            Cosmac = 0,
        }
    } else if #[cfg(feature = "super-chip")] {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub enum Chip8Mode {
            SuperChip = 1,
        }
    } else if #[cfg(feature = "xo-chip")] {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub enum Chip8Mode {
            XoChip = 2,
        }
    } else {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub enum Chip8Mode { }
        compile_error!("Must enable one of the interpreter features for `chip8-lib`: `cosmac`, `super-chip`, `xo-chip`");
    }
}

impl Default for Chip8Mode {
    fn default() -> Self {
        cfg_if::cfg_if! {
            // arbitrary ordering of Super-Chip versus XO-CHIP; Cosmac as
            // default, if enabled, makes sense though
            if #[cfg(feature = "cosmac")] {
                Self::Cosmac
            } else if #[cfg(feature = "super-chip")] {
                Self::SuperChip
            } else if #[cfg(feature = "xo-chip")] {
                Self::XoChip
            }
        }
    }
}
