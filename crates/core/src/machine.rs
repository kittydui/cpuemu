use emu_mem::AddressSpace;

use crate::{Cpu, CpuError};

pub struct Machine<C: Cpu> {
    pub cpu: C,
    pub memory: AddressSpace,
}

impl<C: Cpu> Machine<C> {
    pub fn new(cpu: C) -> Self {
        Self {
            cpu,
            memory: AddressSpace::new(),
        }
    }

    pub fn step(&mut self) -> anyhow::Result<(), CpuError> {
        self.cpu.step(&mut self.memory)
    }
}
