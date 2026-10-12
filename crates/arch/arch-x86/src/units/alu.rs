use emu_core::CpuError;
use emu_mem::AddressSpace;
use iced_x86::Instruction;

use crate::{
    Flags, X86State,
    operand::{self, Location, Size},
};

type AluResult = (Location, Size, u64, Flags);

fn calculate(
    state: &X86State,
    memory: &AddressSpace,
    instruction: &Instruction,
    op: impl Fn(u64, u64, Size) -> (u64, Flags),
) -> Result<AluResult, CpuError> {
    let size = operand::size(instruction, 0)?;
    let destination = operand::locate(state, instruction, 0)?;
    let a = operand::load(state, memory, destination, size)?;
    let b = operand::read(state, memory, instruction, 1)?;
    let (result, flags) = op(a, b, size);
    Ok((destination, size, result, flags))
}

// writes back the result of the operation.
pub(crate) fn alu(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
    op: impl Fn(u64, u64, Size) -> (u64, Flags),
) -> Result<(), CpuError> {
    let (destination, size, result, flags) = calculate(state, memory, instruction, op)?;
    operand::store(state, memory, destination, size, result)?;
    state.rflags.update(Flags::STATUS, flags);
    Ok(())
}

// doesn't write back the result.
pub(crate) fn compare(
    state: &mut X86State,
    memory: &AddressSpace,
    instruction: &Instruction,
    op: impl Fn(u64, u64, Size) -> (u64, Flags),
) -> Result<(), CpuError> {
    let (_, _, _, flags) = calculate(state, memory, instruction, op)?;
    state.rflags.update(Flags::STATUS, flags);
    Ok(())
}

// handles single operand instructions
pub(crate) fn unary(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
    written: Flags,
    op: impl Fn(u64, Size) -> (u64, Flags),
) -> Result<(), CpuError> {
    let size = operand::size(instruction, 0)?;
    let destination = operand::locate(state, instruction, 0)?;
    let value = operand::load(state, memory, destination, size)?;
    let (result, flags) = op(value, size);
    operand::store(state, memory, destination, size, result)?;
    state.rflags.update(written, flags);
    Ok(())
}
