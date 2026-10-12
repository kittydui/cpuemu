use emu_core::CpuError;
use emu_mem::AddressSpace;
use iced_x86::Instruction;

use crate::{X86State, exec::alu, flags};

pub(crate) fn add(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    alu(state, memory, instruction, flags::add)
}
