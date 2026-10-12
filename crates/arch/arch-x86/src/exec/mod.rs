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

// interpreter loop
pub(crate) fn execute(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    state.rip = instruction.next_ip();
    match instruction.mnemonic() {
        // Arithmatic
        Mnemonic::Add => arith::add(state, memory, instruction),
        Mnemonic::Sub => arith::sub(state, memory, instruction),
        Mnemonic::Cmp => arith::cmp(state, memory, instruction),

        // Logic
        Mnemonic::And => logic::and(state, memory, instruction),
        Mnemonic::Or => logic::or(state, memory, instruction),
        Mnemonic::Xor => logic::xor(state, memory, instruction),
        Mnemonic::Test => logic::test(state, memory, instruction),
        Mnemonic::Not => logic::not(state, memory, instruction),

        // Control
        Mnemonic::Jmp => control::jmp(state, memory, instruction),
        Mnemonic::Call => control::call(state, memory, instruction),
        Mnemonic::Ret => control::ret(state, memory, instruction),

        // Stack
        Mnemonic::Push => stack::push(state, memory, instruction),
        Mnemonic::Pop => stack::pop(state, memory, instruction),
        Mnemonic::Leave => stack::leave(state, memory, instruction),

        // Data
        Mnemonic::Mov => data::mov(state, memory, instruction),

        // Misc
        Mnemonic::Nop => data::nop(state, memory, instruction),
        Mnemonic::Hlt => Err(CpuError::Halt),

        // we just check `is_jcc_short_or_near` and it handles all 16 conditional jmps
        _ if instruction.is_jcc_short_or_near() => control::jcc(state, memory, instruction),
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
