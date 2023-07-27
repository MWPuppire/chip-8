use core::fmt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    UnknownOpcode(u16),
    Breakpoint(u16),
    InvalidFile,
    OutOfBounds,
    NoRomLoaded,
    Exited,
    NotDefined(&'static str, Chip8Mode),
    EarlyExitRequested,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnknownOpcode(op) => write!(f, "unknown opcode {:04x}", op),
            Self::Breakpoint(at) => write!(f, "reached breakpoint at {:04x}", at),
            Self::InvalidFile => write!(f, "supplied file is not a valid ROM"),
            Self::OutOfBounds => write!(f, "attempted an out-of-bounds memory access"),
            Self::NoRomLoaded => write!(f, "no ROM is loaded to execute from"),
            Self::Exited => write!(f, "program has exited"),
            Self::NotDefined(op, mode) => write!(f, "`{}` isn't defined for {}", op, mode),
            Self::EarlyExitRequested => write!(f, "emulator paused execution"),
        }
    }
}
#[cfg(feature = "std")]
impl std::error::Error for Error {}

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

impl fmt::Display for Chip8Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            #[cfg(feature = "cosmac")]
            Self::Cosmac => write!(f, "Cosmac"),
            #[cfg(feature = "super-chip")]
            Self::SuperChip => write!(f, "Super-CHIP"),
            #[cfg(feature = "xo-chip")]
            Self::XoChip => write!(f, "XO-CHIP"),
        }
    }
}

#[cfg(any(feature = "cosmac", feature = "super-chip", feature = "xo-chip"))]
impl Default for Chip8Mode {
    #[inline]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chip8ModeParseError {
    InvalidMode,
    NotEnabled(&'static str),
}
impl fmt::Display for Chip8ModeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidMode => write!(f, "unknown CHIP-8 mode"),
            Self::NotEnabled(flag) => write!(f, "mode not enabled; recompile with feature flag {}", flag),
        }
    }
}
#[cfg(feature = "std")]
impl std::error::Error for Chip8ModeParseError {}

impl core::str::FromStr for Chip8Mode {
    type Err = Chip8ModeParseError;
    fn from_str(s: &str) -> Result<Self, Chip8ModeParseError> {
        if s.eq_ignore_ascii_case("cosmac") {
            #[cfg(feature = "cosmac")]
            return Ok(Chip8Mode::Cosmac);
            #[cfg(not(feature = "cosmac"))]
            return Err(Chip8ModeParseError::NotEnabled("cosmac"));
        } else if s.eq_ignore_ascii_case("super-chip") {
            #[cfg(feature = "super-chip")]
            return Ok(Chip8Mode::SuperChip);
            #[cfg(not(feature = "super-chip"))]
            return Err(Chip8ModeParseError::NotEnabled("super-chip"));
        } else if s.eq_ignore_ascii_case("xo-chip") {
            #[cfg(feature = "xo-chip")]
            return Ok(Chip8Mode::XoChip);
            #[cfg(not(feature = "xo-chip"))]
            return Err(Chip8ModeParseError::NotEnabled("xo-chip"));
        } else {
            Err(Chip8ModeParseError::InvalidMode)
        }
    }
}
