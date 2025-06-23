use crate::{engine::Engine, error::Error};
use clear_cache::clear_cache;
use libc::{
    MAP_ANON, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE, c_void, mmap, mprotect, munmap,
};
use log::error;
use std::rc::Rc;

/// A module is a RISC-V program that can be executed in an instance.
pub struct Module {
    pub(crate) engine: Rc<Engine>,
    pub(crate) native_code_addr: *mut c_void,
    pub(crate) native_code_size: usize,
    _memory_ptr: Box<*mut u8>,
    _gas: Box<u32>,
}

impl Module {
    /// Constructs a new `Module` with the given engine.
    ///
    /// # Errors
    ///
    /// - `Error::MemoryAllocationFailed` if the memory allocation fails.
    pub fn new(engine: Rc<Engine>) -> Result<Box<Self>, Error> {
        #[cfg(not(target_arch = "aarch64"))]
        compile_error!("This code only supports aarch64 targets.");

        let native_code_addr: *mut c_void;

        unsafe {
            // 1. Allocate memory.
            // We ask for memory that is readable and writable.
            // On Apple Silicon, we need MAP_JIT to be able to make it executable later.
            #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
            let flags = MAP_ANON | MAP_PRIVATE | libc::MAP_JIT;
            #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
            let flags = MAP_ANON | MAP_PRIVATE;

            let native_code_size = engine.config().max_native_code_size();

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

        Ok(Box::new(Self {
            engine,
            native_code_addr,
            native_code_size: 0,
            _memory_ptr: Box::new(std::ptr::null_mut()),
            _gas: Box::new(0),
        }))
    }

    /// Loads RISC-V executable code into the module.
    ///
    /// # Errors
    ///
    /// - `Error::InvalidInstruction` if the code is invalid.
    pub fn set_riscv_code(&mut self, _code: &[u8]) -> Result<(), Error> {
        unimplemented!();
    }

    /// Loads pre-compiled native code into the module.
    ///
    /// # Errors
    ///
    /// - `Error::InvalidCodeSize` if the code is too large.
    /// - `Error::MemoryProtectionFailed` if the memory protection fails.
    pub fn set_native_code(&mut self, code: &[u8]) -> Result<(), Error> {
        if code.len() > self.engine.config().max_native_code_size() {
            return Err(Error::InvalidCodeSize);
        }

        unsafe {
            // Change memory permissions to writable.
            let result = mprotect(
                self.native_code_addr,
                self.engine.config().max_native_code_size(),
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
                self.engine.config().max_native_code_size(),
                PROT_READ | PROT_EXEC,
            );
            if result != 0 {
                return Err(Error::MemoryProtectionFailed);
            }

            // Clear the instruction cache.
            let result = clear_cache(self.native_code_addr, self.native_code_addr.add(code.len()));
            if !result {
                return Err(Error::ClearCacheFailed);
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
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            let result = munmap(
                self.native_code_addr,
                self.engine.config().max_native_code_size(),
            );
            if result != 0 {
                error!("munmap failed");
            }
        }
    }
}
