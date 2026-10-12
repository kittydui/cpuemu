use emu_core::CpuError;
use emu_mem::AddressSpace;
use iced_x86::Instruction;

use crate::{
    X86State,
    operand::{read, write},
};

pub(crate) fn nop(
    _state: &mut X86State,
    _memory: &mut AddressSpace,
    _instruction: &Instruction,
) -> anyhow::Result<(), CpuError> {
    Ok(())
}

pub(crate) fn mov(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> anyhow::Result<(), CpuError> {
    let value = read(state, memory, instruction, 1)?;
    write(state, memory, instruction, 0, value)
}
