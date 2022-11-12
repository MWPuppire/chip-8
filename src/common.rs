#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    UnknownOpcode,
    Breakpoint,
    InvalidFile,
    OutOfBounds,
    WindowFailure,
    AudioFailure,
    NoRomLoaded,
}
