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

use crate::X86State;

pub(crate) fn execute(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    state.rip = instruction.next_ip();
    match instruction.mnemonic() {
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
