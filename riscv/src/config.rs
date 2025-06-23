/// The multiplier for the max native code size over the riscv code size.
const NATIVE_CODE_MULTIPLIER: usize = 4;

/// Configuration for the RISC-V virtual machine.
pub struct Config {
    /// A function pointer to a syscall handler.
    ///
    /// The VM will call this function when an `ecall` instruction is executed.
    /// The first argument is a slice of `u32` values from registers `a0-a7`.
    /// The second argument is a user-defined context value.
    /// The function should return a `u32` value to be placed in register `a0`.
    pub syscall: fn(args: &[u32], context: u64) -> u32,
    /// A user-defined value passed to the syscall handler.
    pub context: u64,
    /// The maximum amount of memory available to the VM, in bytes.
    pub max_memory: u32,
    /// The maximum size of riscv code in bytes.
    pub max_code_size: usize,
}

impl Config {
    pub fn max_native_code_size(&self) -> usize {
        self.max_code_size * NATIVE_CODE_MULTIPLIER
    }
}
