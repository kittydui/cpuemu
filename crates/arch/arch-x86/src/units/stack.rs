use emu_core::CpuError;
use emu_mem::AddressSpace;
use emu_shared::Value;

use crate::{X86State, units::mem};

pub(crate) const RSP: usize = crate::RSP.0 as usize;
pub(crate) const RBP: usize = crate::RBP.0 as usize;

// Pushes a value of any size: rsp moves down by `T::SIZE`.
pub(crate) fn push<T: Value>(
    state: &mut X86State,
    memory: &mut AddressSpace,
    value: T,
) -> Result<(), CpuError> {
    let rsp = state.gpr[RSP].wrapping_sub(T::SIZE as u64);
    mem::write(memory, rsp, value)?;
    state.gpr[RSP] = rsp;
    Ok(())
}

// Reads the `T` on top of the stack.
pub(crate) fn pop<T: Value>(state: &mut X86State, memory: &AddressSpace) -> Result<T, CpuError> {
    let value = mem::read(memory, state.gpr[RSP])?;
    state.gpr[RSP] = state.gpr[RSP].wrapping_add(T::SIZE as u64);
    Ok(value)
}
