use emu_core::CpuError;
use emu_mem::AddressSpace;
use iced_x86::Instruction;

use crate::{
    Flags, X86State, flags,
    units::alu::{alu, compare, unary},
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

pub(crate) fn adc(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    let carry = state.rflags.contains(Flags::CF);
    alu(state, memory, instruction, |a, b, size| {
        flags::adc(a, b, carry, size)
    })
}

pub(crate) fn sbb(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    let borrow = state.rflags.contains(Flags::CF);
    alu(state, memory, instruction, |a, b, size| {
        flags::sbb(a, b, borrow, size)
    })
}

pub(crate) fn cmp(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    compare(state, memory, instruction, flags::sub)
}

pub(crate) fn inc(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    unary(
        state,
        memory,
        instruction,
        Flags::STATUS - Flags::CF,
        |a, size| flags::add(a, 1, size),
    )
}

pub(crate) fn dec(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    unary(
        state,
        memory,
        instruction,
        Flags::STATUS - Flags::CF,
        |a, size| flags::sub(a, 1, size),
    )
}

pub(crate) fn neg(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> Result<(), CpuError> {
    unary(state, memory, instruction, Flags::STATUS, |a, size| {
        flags::sub(0, a, size)
    })
}
