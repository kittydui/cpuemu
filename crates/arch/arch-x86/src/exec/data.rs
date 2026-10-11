use emu_core::CpuError;
use emu_mem::AddressSpace;
use iced_x86::Instruction;

use crate::X86State;

pub(crate) fn nop(
    _state: &mut X86State,
    _memory: &mut AddressSpace,
    _instruction: &Instruction,
) -> anyhow::Result<(), CpuError> {
    Ok(())
}
