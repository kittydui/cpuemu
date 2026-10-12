use emu_core::CpuError;
use emu_mem::AddressSpace;
use iced_x86::Instruction;

use crate::{X86State, flags, operand::read};

pub(crate) fn jmp(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> anyhow::Result<(), CpuError> {
    // jmp only writes to rip, nothing else, so we just do Ok(())
    state.rip = read(state, memory, instruction, 0)?;
    Ok(())
}

pub(crate) fn jcc(
    state: &mut X86State,
    _memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    // We only write to rip when the Jump Condition is true.
    if flags::jc(state.rflags, instruction.condition_code()) {
        state.rip = instruction.near_branch_target();
    }
    Ok(())
}
