mod config;
mod engine;
mod error;
mod instance;
mod instruction;
mod memory;
mod module;
#[cfg(test)]
mod tests;

pub use config::Config;
pub use engine::Engine;
pub use error::Error;
pub use instance::Instance;
pub use instruction::RiscVInstruction;
pub use memory::Memory;
pub use module::Module;

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let syscall = |_args: &[u32], _context: u64| 0;

        let config = Config {
            syscall,
            max_instance_memory: 1024 * 1024,
            max_code_size: 1024,
        };

        let engine = Engine::new(config);

        // This is ARM64 assembly for a function that takes a u32 and returns it.
        // fn(num: u32) -> u32 { return num; }
        //
        // You can get this from a compiler explorer.
        // sub sp, sp, #16
        // str w0, [sp, #12]
        // ldr w0, [sp, #12]
        // add sp, sp, #16
        // ret
        let mut code = vec![];
        code.extend(0xD10043FF_u32.to_le_bytes());
        code.extend(0xB9000FE0_u32.to_le_bytes());
        code.extend(0xB9400FE0_u32.to_le_bytes());
        code.extend(0x910043FF_u32.to_le_bytes());
        code.extend(0xD65F03C0_u32.to_le_bytes());

        let mut module = Module::new(engine.clone()).unwrap();
        module.set_native_code(&code).unwrap();

        let memory = Memory::new(engine.clone());

        let mut instance = Instance::new(module, memory).unwrap();

        let output = instance.call(0, 42).unwrap();

        assert_eq!(output, 42);
    }
}
*/
