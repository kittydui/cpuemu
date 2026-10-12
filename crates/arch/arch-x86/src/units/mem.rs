use emu_core::CpuError;
use emu_mem::AddressSpace;
use emu_shared::{Address, Endian, Value};

pub(crate) fn read<T: Value>(memory: &AddressSpace, address: u64) -> Result<T, CpuError> {
    memory
        .read_value(Address(address), Endian::Little)
        .map_err(CpuError::Fault)
}

pub(crate) fn write<T: Value>(
    memory: &mut AddressSpace,
    address: u64,
    value: T,
) -> Result<(), CpuError> {
    memory
        .write_value(Address(address), value, Endian::Little)
        .map_err(CpuError::Fault)
}
