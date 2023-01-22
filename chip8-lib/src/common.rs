#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    UnknownOpcode,
    Breakpoint,
    InvalidFile,
    OutOfBounds,
    NoRomLoaded,
    // variants not used by chip8-lib,
    // but included for use in enclosing libraries
    WindowFailure,
    AudioFailure,
}
