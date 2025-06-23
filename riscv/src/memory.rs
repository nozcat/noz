use crate::engine::Engine;
use std::rc::Rc;

/// The memory of an instance.
pub struct Memory {
    pub(crate) engine: Rc<Engine>,
    _memory: Vec<u8>,
}

impl Memory {
    /// Constructs a new `Memory` with the given engine.
    pub fn new(engine: Rc<Engine>) -> Box<Self> {
        let memory = vec![0; engine.config().max_instance_memory as usize];

        Box::new(Self {
            engine,
            _memory: memory,
        })
    }
}
