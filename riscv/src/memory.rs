use crate::engine::Engine;

/// The memory of an instance.
pub struct Memory {
    _memory: Vec<u8>,
}

impl Memory {
    /// Constructs a new `Memory` with the given engine.
    pub fn new(engine: &Engine) -> Self {
        Self {
            _memory: vec![0; engine.config().max_instance_memory as usize],
        }
    }
}
