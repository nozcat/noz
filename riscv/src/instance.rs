use crate::{error::Error, memory::Memory, module::Module};
use std::{mem, rc::Rc};

/// An instance is a single instance of a module capable of executing code.
pub struct Instance {
    module: Box<Module>,
    memory: Box<Memory>,
}

impl Instance {
    /// Constructs a new `Instance` with the given module and memory.
    pub fn new(module: Box<Module>, memory: Box<Memory>) -> Result<Self, Error> {
        if !Rc::ptr_eq(&module.engine, &memory.engine) {
            return Err(Error::InvalidEngine);
        }

        Ok(Self { module, memory })
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
            let native_fn_addr = (self.module.native_code_addr as *mut u8).add(pc as usize);

            let func: extern "C" fn(u32) -> u32 = mem::transmute(native_fn_addr);

            Ok(func(arg))
        }
    }

    /// Decomposes the instance back into its module and memory.
    pub fn decompose(self) -> (Box<Module>, Box<Memory>) {
        (self.module, self.memory)
    }
}
