use emu_core::CpuError;
use emu_mem::AddressSpace;
use iced_x86::Instruction;

use crate::{
    X86State, flags,
    units::alu::{alu, compare},
};

pub(crate) fn add(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    alu(state, memory, instruction, flags::add)
}

pub(crate) fn sub(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    alu(state, memory, instruction, flags::sub)
}

pub(crate) fn cmp(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    compare(state, memory, instruction, flags::sub)
}
