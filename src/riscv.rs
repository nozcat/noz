//! RISC-V 32-bit IM virtual machine.

use libc::{
    MAP_ANON, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE, c_void, mmap, mprotect, munmap,
};
use log::error;
use std::mem;

/// The multiplier for the max native code size over the riscv code size.
const NATIVE_CODE_MULTIPLIER: usize = 4;

/// Virtual machine for RISC-V 32-bit IM.
pub struct Riscv {
    config: Config,
    _memory: Vec<u8>,
    native_code_addr: *mut c_void,
    native_code_size: usize,
    gas: Box<u64>,
}

/// Error type for RISC-V virtual machine operations.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    /// The code is too large.
    InvalidCodeSize,
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
            Error::InvalidCodeSize => write!(f, "invalid code size"),
            Error::InvalidInstruction => write!(f, "invalid or unsupported instruction"),
            Error::MemoryAllocationFailed => write!(f, "memory allocation failed"),
            Error::MemoryProtectionFailed => write!(f, "memory protection failed"),
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
    /// The maximum size of riscv code in bytes.
    pub max_code_size: usize,
}

impl Config {
    pub fn max_native_code_size(&self) -> usize {
        self.max_code_size * NATIVE_CODE_MULTIPLIER
    }
}

impl Riscv {
    /// Constructs a new `Riscv` VM with the given configuration.
    ///
    /// # Errors
    ///
    /// - `Error::MemoryAllocationFailed` if the memory allocation fails.
    pub fn new(config: Config) -> Result<Self, Error> {
        #[cfg(not(target_arch = "aarch64"))]
        compile_error!("This code only supports aarch64 targets.");

        let memory = vec![0; config.max_memory as usize];
        let native_code_addr: *mut c_void;

        unsafe {
            // 1. Allocate memory.
            // We ask for memory that is readable and writable.
            // On Apple Silicon, we need MAP_JIT to be able to make it executable later.
            #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
            let flags = MAP_ANON | MAP_PRIVATE | libc::MAP_JIT;
            #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
            let flags = MAP_ANON | MAP_PRIVATE;

            let native_code_size = config.max_native_code_size();

            native_code_addr = mmap(
                std::ptr::null_mut(),
                native_code_size,
                PROT_READ | PROT_WRITE,
                flags,
                -1,
                0,
            );

            if native_code_addr == libc::MAP_FAILED {
                return Err(Error::MemoryAllocationFailed);
            }
        }

        Ok(Self {
            config,
            _memory: memory,
            native_code_addr,
            native_code_size: 0,
            gas: Box::new(0),
        })
    }

    /// Loads the RISC-V executable code into the VM.
    ///
    /// This function should be called before `run`. The provided code will be
    /// loaded into the VM's memory.
    ///
    /// # Errors
    ///
    /// - `Error::InvalidInstruction` if the code is invalid.
    pub fn set_riscv_code(&mut self, _code: &[u8]) -> Result<(), Error> {
        unimplemented!();
    }

    /// Loads pre-compiled native code into the VM.
    ///
    /// # Errors
    ///
    /// - `Error::InvalidCodeSize` if the code is too large.
    /// - `Error::MemoryProtectionFailed` if the memory protection fails.
    pub fn set_native_code(&mut self, code: &[u8]) -> Result<(), Error> {
        if code.len() > self.config.max_native_code_size() {
            return Err(Error::InvalidCodeSize);
        }

        unsafe {
            // Change memory permissions to writable.
            let result = mprotect(
                self.native_code_addr,
                self.config.max_native_code_size(),
                PROT_READ | PROT_WRITE,
            );
            if result != 0 {
                return Err(Error::MemoryProtectionFailed);
            }

            std::ptr::copy_nonoverlapping(
                code.as_ptr(),
                self.native_code_addr as *mut u8,
                code.len(),
            );

            // Change memory permissions to read-only and executable.
            let result = mprotect(
                self.native_code_addr,
                self.config.max_native_code_size(),
                PROT_READ | PROT_EXEC,
            );
            if result != 0 {
                return Err(Error::MemoryProtectionFailed);
            }
        }

        self.native_code_size = code.len();

        Ok(())
    }

    /// Returns a slice to the native (JIT-compiled) code.
    pub fn native_code(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.native_code_addr as *mut u8, self.native_code_size)
        }
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
    pub fn gas(&self) -> u64 {
        *self.gas
    }

    /// Returns a mutable slice of the entire VM memory.
    pub fn memory(&mut self) -> &mut [u8] {
        &mut self._memory
    }

    /// Executes the loaded RISC-V function.
    ///
    /// Execution starts from the given program counter `pc` with a single 32-bit
    /// argument `arg`, and continues until it completes, an error occurs, or
    /// gas runs out.
    ///
    /// # Arguments
    ///
    /// * `pc` - The program counter to start execution from.
    /// * `arg` - A single 32-bit argument.
    ///
    /// # Returns
    ///
    /// Returns a `u32` return value upon successful completion.
    ///
    /// # Errors
    ///
    /// - `Error::OutOfGas` if gas runs out.
    pub fn call(&mut self, pc: u32, arg: u32) -> Result<u32, Error> {
        unsafe {
            let native_fn_addr = (self.native_code_addr as *mut u8).add(pc as usize);

            let func: extern "C" fn(u32) -> u32 = mem::transmute(native_fn_addr);

            Ok(func(arg))
        }
    }
}

impl Drop for Riscv {
    fn drop(&mut self) {
        unsafe {
            let result = munmap(self.native_code_addr, self.config.max_native_code_size());
            if result != 0 {
                error!("munmap failed");
            }
        }
    }
}
