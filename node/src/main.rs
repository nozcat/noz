use log::info;
use riscv::{Config, Engine, Instance, Memory, Module};

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let syscall = |args: &[u32], context: u64| {
        info!("syscall: {:?}, {:?}", args, context);
        0
    };

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

    info!("output: {}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
