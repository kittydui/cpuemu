use emu_mem::AddressSpace;
use emu_shared::{Address, Endian, Fault};

use crate::{Register, RegisterId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuError {
    Fault(Fault),
    IllegalInstruction(Address),
    Unimplemented(Address),
    Halt,
}

pub trait Cpu {
    fn registers(&self) -> &'static [Register];
    fn read_register(&self, id: RegisterId, out: &mut [u8]);
    fn write_register(&mut self, id: RegisterId, value: &[u8]);
    fn pc(&self) -> Address;
    fn set_pc(&mut self, pc: Address);
    fn endian(&self) -> Endian;
    fn step(&mut self, memory: &mut AddressSpace) -> anyhow::Result<(), CpuError>;
}
