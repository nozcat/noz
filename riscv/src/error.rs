/// Error type for RISC-V virtual machine operations.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    /// The VM failed to clear the instruction cache.
    ClearCacheFailed,
    /// The code is too large.
    InvalidCodeSize,
    /// The engine of the module and memory are not the same.
    InvalidEngine,
    /// The VM encountered an instruction that is not valid or not supported.
    InvalidInstruction,
    /// The VM failed to allocate memory.
    MemoryAllocationFailed,
    /// The VM failed to change memory permissions.
    MemoryProtectionFailed,
    /// The VM ran out of gas.
    OutOfGas,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ClearCacheFailed => write!(f, "clear cache failed"),
            Error::InvalidCodeSize => write!(f, "invalid code size"),
            Error::InvalidEngine => write!(f, "invalid engine"),
            Error::InvalidInstruction => write!(f, "invalid or unsupported instruction"),
            Error::MemoryAllocationFailed => write!(f, "memory allocation failed"),
            Error::MemoryProtectionFailed => write!(f, "memory protection failed"),
            Error::OutOfGas => write!(f, "out of gas"),
        }
    }
}

impl std::error::Error for Error {}
