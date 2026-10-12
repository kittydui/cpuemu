use emu_core::CpuError;
use emu_mem::AddressSpace;
use emu_shared::Address;
use iced_x86::Instruction;

use crate::{
    X86State,
    operand::{self, Size, read, write},
    units::{
        mem,
        stack::{self, RBP, RSP},
    },
};

pub(crate) fn push(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    if operand::size(instruction, 0)? != Size::Qword {
        return Err(CpuError::Unimplemented(Address(instruction.ip())));
    }

    let value = read(state, memory, instruction, 0)?;
    stack::push(state, memory, value)
}

pub(crate) fn pop(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    let rsp = state.gpr[RSP];
    let value: u64 = mem::read(memory, rsp)?;
    state.gpr[RSP] = rsp.wrapping_add(8);
    if let Err(error) = write(state, memory, instruction, 0, value) {
        state.gpr[RSP] = rsp;
        return Err(error);
    }
    Ok(())
}

pub(crate) fn leave(
    state: &mut X86State,
    memory: &mut AddressSpace,
    _instruction: &Instruction,
) -> Result<(), CpuError> {
    let rbp = state.gpr[RBP];
    let saved: u64 = mem::read(memory, rbp)?;
    state.gpr[RSP] = rbp.wrapping_add(8);
    state.gpr[RBP] = saved;
    Ok(())
}
