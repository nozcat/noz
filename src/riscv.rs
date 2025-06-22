//! RISC-V 32-bit IM virtual machine.

/// Virtual machine for RISC-V 32-bit IM.
pub struct Riscv {
    _config: Config,
    _memory: Vec<u8>,
    gas: Box<u64>,
}

/// Error type for RISC-V virtual machine operations.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    /// The VM encountered an instruction that is not valid or not supported.
    InvalidInstruction,
    /// The VM ran out of gas.
    OutOfGas,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidInstruction => write!(f, "invalid or unsupported instruction"),
            Error::OutOfGas => write!(f, "out of gas"),
        }
    }
}

impl std::error::Error for Error {}

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
}

impl Riscv {
    /// Constructs a new `Riscv` VM with the given configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use noz::riscv::{Config, Riscv};
    ///
    /// fn my_syscall_handler(_args: &[u32], _context: u64) -> u32 {
    ///     0
    /// }
    ///
    /// let config = Config {
    ///     syscall: my_syscall_handler,
    ///     context: 0,
    ///     max_memory: 1024 * 1024, // 1 MiB
    /// };
    ///
    /// let vm = Riscv::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        let memory = vec![0; config.max_memory as usize];
        Self {
            _config: config,
            _memory: memory,
            gas: Box::new(0),
        }
    }

    /// Loads the RISC-V executable code into the VM.
    ///
    /// This function should be called before `run`. The provided code will be
    /// loaded into the VM's memory.
    ///
    /// # Errors
    ///
    /// Returns an error if the code is invalid or cannot be loaded.
    pub fn set_riscv_code(&mut self, _code: &[u8]) -> Result<(), Error> {
        unimplemented!();
    }

    /// Returns a slice to the native (JIT-compiled) code.
    pub fn get_native_code(&self) -> &[u8] {
        unimplemented!();
    }

    /// Loads pre-compiled native code into the VM.
    pub fn set_native_code(&mut self, _code: &[u8]) {
        unimplemented!();
    }

    /// Sets the value of a specific general-purpose register.
    ///
    /// # Arguments
    ///
    /// * `_index` - The index of the register (0-31).
    /// * `_value` - The value to write to the register.
    ///
    /// # Panics
    ///
    /// Panics if `_index` is greater than 31.
    pub fn set_reg(&mut self, _index: u32, _value: u32) {
        unimplemented!();
    }

    /// Sets the program counter (PC) to a specific address.
    ///
    /// # Arguments
    ///
    /// * `_value` - The new value for the program counter.
    pub fn set_pc(&mut self, _value: u32) {
        unimplemented!();
    }

    /// Sets the amount of gas for the VM's execution.
    ///
    /// Gas is a mechanism to limit the computational resources a VM can use.
    /// Each instruction consumes a certain amount of gas. If the gas runs out,
    /// execution will stop.
    ///
    /// # Arguments
    ///
    /// * `gas` - The total amount of gas available for execution.
    pub fn set_gas(&mut self, gas: u64) {
        *self.gas = gas;
    }

    /// Returns the remaining amount of gas.
    pub fn get_gas(&self) -> u64 {
        *self.gas
    }

    /// Executes the loaded RISC-V code.
    ///
    /// Execution starts from the current program counter and continues until
    /// it completes, an error occurs, or gas runs out.
    ///
    /// # Errors
    ///
    /// Returns `Error::OutOfGas` if gas runs out.
    pub fn run(&mut self) -> Result<(), Error> {
        unimplemented!();
    }
}
