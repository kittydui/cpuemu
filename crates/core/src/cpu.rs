use emu_mem::AddressSpace;
use emu_shared::{Address, Endian, Fault, Value};

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

// Typed register access for any CPU, e.g. `cpu.register::<u64>(RAX)`. It's a separate
// trait because generic methods on `Cpu` would stop it being usable as `dyn Cpu`.
pub trait CpuExt: Cpu {
    // Returns the register's low `T::SIZE` bytes, zero-extended if `T` is wider.
    fn register<T: Value>(&self, id: RegisterId) -> T {
        let size = register_size(self.registers(), id);
        let mut bytes = [0; MAX_REGISTER_SIZE];
        self.read_register(id, &mut bytes[..size]);
        T::from_bytes(&bytes[..T::SIZE], Endian::Little)
    }

    // Writes `value` zero-extended to the register's size, or truncated if it's wider.
    fn set_register<T: Value>(&mut self, id: RegisterId, value: T) {
        let size = register_size(self.registers(), id);
        let mut bytes = [0; MAX_REGISTER_SIZE];
        value.to_bytes(Endian::Little, &mut bytes[..T::SIZE]);
        self.write_register(id, &bytes[..size]);
    }
}

impl<C: Cpu + ?Sized> CpuExt for C {}

// Register values are little-endian bytes, up to a 512-bit vector register.
const MAX_REGISTER_SIZE: usize = 64;

fn register_size(registers: &[Register], id: RegisterId) -> usize {
    registers
        .iter()
        .find(|register| register.id == id)
        .unwrap_or_else(|| panic!("unknown register {id:?}"))
        .size
}
