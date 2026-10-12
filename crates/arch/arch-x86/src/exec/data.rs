use emu_core::CpuError;
use emu_mem::AddressSpace;
use iced_x86::Instruction;

use crate::{
    X86State,
    operand::{effective_address, read, write},
};

pub(crate) fn mov(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> anyhow::Result<(), CpuError> {
    let value = read(state, memory, instruction, 1)?;
    write(state, memory, instruction, 0, value)
}

pub(crate) fn lea(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> anyhow::Result<(), CpuError> {
    let ea = effective_address(state, instruction, 1)?;
    write(state, memory, instruction, 0, ea)
}
