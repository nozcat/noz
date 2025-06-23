use crate::Config;
use std::rc::Rc;

/// The engine is a single configuration of the RISC-V virtual machine.
pub struct Engine {
    config: Config,
}

impl Engine {
    /// Constructs a new `Engine` with the given configuration.
    pub fn new(config: Config) -> Rc<Self> {
        Rc::new(Self { config })
    }

    /// Returns a reference to the configuration of the engine.
    pub fn config(&self) -> &Config {
        &self.config
    }
}
