pub struct Riscv {
    _config: Config,
    _memory: Vec<u8>,
    gas: Box<u64>,
}

pub enum Error {
    InvalidInstruction,
}

pub struct Config {
    pub syscall: SyscallFn,
    pub context: u64,
    pub max_memory: u32,
}

pub type SyscallFn = fn(args: &[u32], context: u64) -> u32;

impl Riscv {
    pub fn new(config: Config) -> Self {
        let memory = vec![0; config.max_memory as usize];
        Self {
            _config: config,
            _memory: memory,
            gas: Box::new(0),
        }
    }

    pub fn set_riscv_code(&self, _code: &[u8]) -> Result<(), Error> {
        unimplemented!();
    }

    pub fn get_native_code(&self) -> &[u8] {
        unimplemented!();
    }

    pub fn set_native_code(&self, _code: &[u8]) {
        unimplemented!();
    }

    pub fn set_reg(&self, _index: u32, _value: u32) {
        unimplemented!();
    }

    pub fn set_pc(&self, _value: u32) {
        unimplemented!();
    }

    pub fn set_gas(&mut self, gas: u64) {
        *self.gas = gas;
    }

    pub fn get_gas(&self) -> u64 {
        *self.gas
    }

    pub fn run(&self) -> Result<(), Error> {
        unimplemented!();
    }
}
