use emu_core::CpuError;
use emu_mem::AddressSpace;
use iced_x86::Instruction;

use crate::{
    X86State,
    exec::{alu, compare},
    flags, operand,
};

pub(crate) fn and(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    alu(state, memory, instruction, |a, b, size| {
        let result = a & b & size.mask();
        (result, flags::logic(result, size))
    })
}

pub(crate) fn not(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    // not inverts its operand in place and leaves every flag unchanged.
    let size = operand::size(instruction, 0)?;
    let destination = operand::locate(state, instruction, 0)?;
    let value = operand::load(state, memory, destination, size)?;
    operand::store(state, memory, destination, size, !value & size.mask())
}

pub(crate) fn or(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    alu(state, memory, instruction, |a, b, size| {
        let result = (a | b) & size.mask();
        (result, flags::logic(result, size))
    })
}

pub(crate) fn xor(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    alu(state, memory, instruction, |a, b, size| {
        let result = (a ^ b) & size.mask();
        (result, flags::logic(result, size))
    })
}

pub(crate) fn test(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    compare(state, memory, instruction, |a, b, size| {
        let result = a & b & size.mask();
        (result, flags::logic(result, size))
    })
}
