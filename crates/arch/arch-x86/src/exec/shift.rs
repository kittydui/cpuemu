use emu_core::CpuError;
use emu_mem::AddressSpace;
use iced_x86::Instruction;

use crate::{
    Flags, X86State, flags,
    operand::{self, Size},
};

fn shift(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
    op: impl Fn(u64, u32, Size) -> (u64, Flags),
) -> Result<(), CpuError> {
    let size = operand::size(instruction, 0)?;
    let destination = operand::locate(state, instruction, 0)?;
    let value = operand::load(state, memory, destination, size)?;
    let mask = if size == Size::Qword { 0x3F } else { 0x1F };
    let count = operand::read(state, memory, instruction, 1)? as u32 & mask;
    if count == 0 {
        return operand::store(state, memory, destination, size, value);
    }
    let (result, flags) = op(value, count, size);
    operand::store(state, memory, destination, size, result)?;
    state.rflags.update(Flags::STATUS, flags);
    Ok(())
}

pub(crate) fn shl(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    shift(state, memory, instruction, flags::shl)
}

pub(crate) fn shr(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    shift(state, memory, instruction, flags::shr)
}

pub(crate) fn sar(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    shift(state, memory, instruction, flags::sar)
}
