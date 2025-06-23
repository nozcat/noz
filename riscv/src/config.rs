/// The multiplier for the max native code size over the riscv code size.
const NATIVE_CODE_MULTIPLIER: usize = 4;

/// Configuration for a RISC-V engine.
pub struct Config {
    /// A function pointer to a syscall handler.
    ///
    /// The engine will call this function when an `ecall` instruction is executed in an instance.
    /// The first argument is a slice of `u32` values from registers `a0-a7`.
    /// The second argument is a user-defined context value.
    /// The function should return a `u32` value to be placed in register `a0`.
    pub syscall: fn(args: &[u32], context: u64) -> u32,
    /// The maximum amount of memory available to an instance, in bytes.
    pub max_instance_memory: u32,
    /// The maximum size of riscv code in bytes.
    pub max_code_size: usize,
}

impl Config {
    pub fn max_native_code_size(&self) -> usize {
        self.max_code_size * NATIVE_CODE_MULTIPLIER
    }
}
