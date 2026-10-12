use emu_core::CpuError;
use emu_mem::AddressSpace;
use iced_x86::{Instruction, Register};

use crate::{
    Flags, X86State, flags,
    operand::{self, Size, read, write},
};

pub(crate) fn imul(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
) -> anyhow::Result<(), CpuError> {
    let size = operand::size(instruction, 0)?;
    match instruction.op_count() {
        1 => imul_wide(state, memory, instruction, size),
        count => {
            let (a, b) = if count == 3 {
                (
                    read(state, memory, instruction, 1)?,
                    read(state, memory, instruction, 2)?,
                )
            } else {
                (
                    read(state, memory, instruction, 0)?,
                    read(state, memory, instruction, 1)?,
                )
            };
            let (low, _, flags) = flags::imul(a, b, size);
            write(state, memory, instruction, 0, low)?;
            state.rflags.update(Flags::STATUS, flags);
            Ok(())
        }
    }
}

fn imul_wide(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
    size: Size,
) -> Result<(), CpuError> {
    let (accumulator, high_register) = match size {
        Size::Byte => (Register::AL, Register::AH),
        Size::Word => (Register::AX, Register::DX),
        Size::Dword => (Register::EAX, Register::EDX),
        Size::Qword => (Register::RAX, Register::RDX),
    };
    let a = state.read(accumulator);
    let b = read(state, memory, instruction, 0)?;
    let (low, high, flags) = flags::imul(a, b, size);
    state.write(accumulator, low);
    state.write(high_register, high);
    state.rflags.update(Flags::STATUS, flags);
    Ok(())
}
