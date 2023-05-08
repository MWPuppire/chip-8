#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    UnknownOpcode(u16),
    Breakpoint(u16),
    InvalidFile,
    OutOfBounds,
    NoRomLoaded,
    VBlankWait,
    AwaitingKey,
    Exited,
}
