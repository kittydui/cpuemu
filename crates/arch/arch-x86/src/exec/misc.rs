use emu_core::CpuError;
use emu_mem::AddressSpace;
use iced_x86::Instruction;

use crate::X86State;

// NOP: does nothing. Also covers the multi-byte `nop r/m` forms used for padding: their
// memory operand is never accessed, so they can't fault.
pub(crate) fn nop(
    _state: &mut X86State,
    _memory: &mut AddressSpace,
    _instruction: &Instruction,
) -> Result<(), CpuError> {
    Ok(())
}

// HLT: stops the CPU. execute has already moved rip past the hlt, and step keeps that for
// Halt, so resuming continues with the next instruction.
pub(crate) fn hlt(
    _state: &mut X86State,
    _memory: &mut AddressSpace,
    _instruction: &Instruction,
) -> Result<(), CpuError> {
    Err(CpuError::Halt)
}
