use core::fmt;

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
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
        pub enum Chip8Mode {
            #[default]
            Cosmac = 0,
            SuperChip = 1,
            XoChip = 2,
        }
    } else if #[cfg(all(feature = "cosmac", feature = "super-chip"))] {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
        pub enum Chip8Mode {
            #[default]
            Cosmac = 0,
            SuperChip = 1,
        }
    } else if #[cfg(all(feature = "cosmac", feature = "xo-chip"))] {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
        pub enum Chip8Mode {
            #[default]
            Cosmac = 0,
            XoChip = 2,
        }
    } else if #[cfg(all(feature = "super-chip", feature = "xo-chip"))] {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
        pub enum Chip8Mode {
            #[default]
            SuperChip = 1,
            XoChip = 2,
        }
    } else if #[cfg(feature = "cosmac")] {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
        pub enum Chip8Mode {
            #[default]
            Cosmac = 0,
        }
    } else if #[cfg(feature = "super-chip")] {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
        pub enum Chip8Mode {
            #[default]
            SuperChip = 1,
        }
    } else if #[cfg(feature = "xo-chip")] {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
        pub enum Chip8Mode {
            #[default]
            XoChip = 2,
        }
    } else {
        compile_error!("Must enable one of the interpreter features for `chip8-lib`: `cosmac`, `super-chip`, `xo-chip`");
    }
}
