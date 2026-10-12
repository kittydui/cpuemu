pub mod arith;
pub mod control;
pub mod conv;
pub mod data;
pub mod flag;
pub mod logic;
pub mod mul;
pub mod shift;
pub mod stack;
pub mod string;

use emu_core::CpuError;
use emu_mem::AddressSpace;
use emu_shared::Address;
use iced_x86::{Instruction, Mnemonic};

use crate::{
    Flags, X86State,
    operand::{self, Location, Size},
};

// ALU helper

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
pub(crate) fn execute(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    state.rip = instruction.next_ip();
    match instruction.mnemonic() {
        // Arithmatic
        Mnemonic::Add => arith::add(state, memory, instruction),

        // Misc
        Mnemonic::Nop => data::nop(state, memory, instruction),
        Mnemonic::Hlt => Err(CpuError::Halt),
        _ => Err(CpuError::Unimplemented(Address(instruction.ip()))),
    }
}

/*
Signature for each mnemonic/insn:
pub(crate) fn name(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError>
*/
