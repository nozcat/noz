use crate::{memory::Memory, module::Module};

/// An instance is a single instance of a module capable of executing code.
pub struct Instance {
    module: Module,
    memory: Memory,
}

impl Instance {
    /// Constructs a new `Instance` with the given module and memory.
    pub fn new(module: Module, memory: Memory) -> Self {
        Self { module, memory }
    }

    /// Decomposes the instance back into its module and memory.
    pub fn decompose(self) -> (Module, Memory) {
        (self.module, self.memory)
    }
}
