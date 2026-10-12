use emu_core::CpuError;
use emu_mem::AddressSpace;
use emu_shared::Address;
use iced_x86::{Instruction, OpKind, Register};

use crate::X86State;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Size {
    Byte,
    Word,
    Dword,
    Qword,
}

impl Size {
    pub(crate) const fn mask(self) -> u64 {
        match self {
            Size::Byte => 0xFF,
            Size::Word => 0xFFFF,
            Size::Dword => 0xFFFF_FFFF,
            Size::Qword => u64::MAX,
        }
    }

    pub(crate) const fn sign_bit(self) -> u64 {
        (self.mask() >> 1) + 1
    }

    pub(crate) const fn from_bytes(bytes: usize) -> Option<Self> {
        match bytes {
            1 => Some(Size::Byte),
            2 => Some(Size::Word),
            4 => Some(Size::Dword),
            8 => Some(Size::Qword),
            _ => None,
        }
    }

    pub(crate) const fn bytes(self) -> usize {
        match self {
            Size::Byte => 1,
            Size::Word => 2,
            Size::Dword => 4,
            Size::Qword => 8,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Location {
    Register { index: usize, high: bool },
    Memory(u64),
}

pub(crate) fn locate(
    state: &X86State,
    instruction: &Instruction,
    operand: u32,
) -> Result<Location, CpuError> {
    match instruction.op_kind(operand) {
        OpKind::Register => {
            let (index, _, high) =
                gpr(instruction.op_register(operand)).ok_or_else(|| unimplemented(instruction))?;
            Ok(Location::Register { index, high })
        }
        OpKind::Memory => Ok(Location::Memory(effective_address(
            state,
            instruction,
            operand,
        )?)),
        _ => Err(unimplemented(instruction)),
    }
}

pub(crate) fn load(
    state: &X86State,
    memory: &AddressSpace,
    location: Location,
    size: Size,
) -> Result<u64, CpuError> {
    match location {
        Location::Register { index, high } => Ok(extract(state.gpr[index], size, high)),
        Location::Memory(address) => {
            let mut bytes = [0; 8];
            memory
                .read(Address(address), &mut bytes[..size.bytes()])
                .map_err(CpuError::Fault)?;
            Ok(u64::from_le_bytes(bytes))
        }
    }
}

pub(crate) fn store(
    state: &mut X86State,
    memory: &mut AddressSpace,
    location: Location,
    size: Size,
    value: u64,
) -> Result<(), CpuError> {
    match location {
        Location::Register { index, high } => {
            state.gpr[index] = merge(state.gpr[index], value, size, high);
            Ok(())
        }
        Location::Memory(address) => memory
            .write(Address(address), &value.to_le_bytes()[..size.bytes()])
            .map_err(CpuError::Fault),
    }
}

fn extract(value: u64, size: Size, high: bool) -> u64 {
    if high {
        (value >> 8) & 0xFF
    } else {
        value & size.mask()
    }
}

fn merge(old: u64, value: u64, size: Size, high: bool) -> u64 {
    match (size, high) {
        (_, true) => (old & !0xFF00) | ((value & 0xFF) << 8),
        (Size::Dword, _) => value & 0xFFFF_FFFF,
        (Size::Qword, _) => value,
        _ => (old & !size.mask()) | (value & size.mask()),
    }
}

pub(crate) fn read(
    state: &X86State,
    memory: &AddressSpace,
    instruction: &Instruction,
    operand: u32,
) -> Result<u64, CpuError> {
    match instruction.op_kind(operand) {
        OpKind::Register | OpKind::Memory => {
            let size = size(instruction, operand)?;
            let location = locate(state, instruction, operand)?;
            load(state, memory, location, size)
        }
        OpKind::NearBranch64 => Ok(instruction.near_branch64()),
        OpKind::Immediate8
        | OpKind::Immediate16
        | OpKind::Immediate32
        | OpKind::Immediate64
        | OpKind::Immediate8to16
        | OpKind::Immediate8to32
        | OpKind::Immediate8to64
        | OpKind::Immediate32to64 => Ok(instruction.immediate(operand)),
        _ => Err(unimplemented(instruction)),
    }
}

pub(crate) fn write(
    state: &mut X86State,
    memory: &mut AddressSpace,
    instruction: &Instruction,
    operand: u32,
    value: u64,
) -> Result<(), CpuError> {
    let size = size(instruction, operand)?;
    let location = locate(state, instruction, operand)?;
    store(state, memory, location, size, value)
}

pub(crate) fn size(instruction: &Instruction, operand: u32) -> anyhow::Result<Size, CpuError> {
    let bytes = match instruction.op_kind(operand) {
        OpKind::Register => instruction.op_register(operand).size(),
        OpKind::Memory => instruction.memory_size().size(),
        OpKind::Immediate8 => 1,
        OpKind::Immediate16 | OpKind::Immediate8to16 => 2,
        OpKind::Immediate32 | OpKind::Immediate8to32 => 4,
        OpKind::Immediate64 | OpKind::Immediate8to64 | OpKind::Immediate32to64 => 8,
        _ => 0,
    };
    Size::from_bytes(bytes).ok_or_else(|| unimplemented(instruction))
}

pub(crate) fn effective_address(
    state: &X86State,
    instruction: &Instruction,
    operand: u32,
) -> Result<u64, CpuError> {
    instruction
        .virtual_address(operand, 0, |register, _, _| match register {
            // In 64-bit mode the ES, CS, SS and DS bases are always 0.
            Register::ES | Register::CS | Register::SS | Register::DS => Some(0),
            // FS/GS bases aren't modelled yet, so these become Unimplemented.
            Register::FS | Register::GS => None,
            register => {
                gpr(register).map(|(index, size, high)| extract(state.gpr[index], size, high))
            }
        })
        .ok_or_else(|| unimplemented(instruction))
}

pub(crate) fn read_register(
    state: &X86State,
    instruction: &Instruction,
    register: Register,
) -> Result<u64, CpuError> {
    let (index, size, high) = gpr(register).ok_or_else(|| unimplemented(instruction))?;
    Ok(extract(state.gpr[index], size, high))
}

pub(crate) fn write_register(
    state: &mut X86State,
    instruction: &Instruction,
    register: Register,
    value: u64,
) -> Result<(), CpuError> {
    let (index, size, high) = gpr(register).ok_or_else(|| unimplemented(instruction))?;
    state.gpr[index] = merge(state.gpr[index], value, size, high);
    Ok(())
}

fn gpr(register: Register) -> Option<(usize, Size, bool)> {
    if !register.is_gpr() {
        return None;
    }
    let high = matches!(
        register,
        Register::AH | Register::CH | Register::DH | Register::BH
    );
    let size = Size::from_bytes(register.size())?;
    Some((register.full_register().number(), size, high))
}

fn unimplemented(instruction: &Instruction) -> CpuError {
    CpuError::Unimplemented(Address(instruction.ip()))
}

#[cfg(test)]
mod tests {
    use emu_shared::Permissions;
    use iced_x86::{Decoder, DecoderOptions};

    use super::*;
    use crate::{Flags, RAX, RBX, RCX};

    fn decode(bytes: &[u8]) -> Instruction {
        Decoder::with_ip(64, bytes, 0x1000, DecoderOptions::NONE).decode()
    }

    fn state() -> X86State {
        X86State {
            gpr: [0; 16],
            rip: 0x1000,
            rflags: Flags::RESET,
        }
    }

    #[test]
    fn dword_write_zero_extends() {
        let mut state = state();
        state.gpr[RAX.0 as usize] = u64::MAX;
        let instruction = decode(&[0x90]);
        write_register(&mut state, &instruction, Register::EAX, 1).unwrap();
        assert_eq!(state.gpr[RAX.0 as usize], 1);
    }

    #[test]
    fn narrow_writes_keep_other_bits() {
        let mut state = state();
        let instruction = decode(&[0x90]);
        state.gpr[RAX.0 as usize] = 0x1122_3344_5566_7788;
        write_register(&mut state, &instruction, Register::AX, 0xAAAA).unwrap();
        assert_eq!(state.gpr[RAX.0 as usize], 0x1122_3344_5566_AAAA);
        write_register(&mut state, &instruction, Register::AL, 0xBB).unwrap();
        assert_eq!(state.gpr[RAX.0 as usize], 0x1122_3344_5566_AABB);
        write_register(&mut state, &instruction, Register::AH, 0xCC).unwrap();
        assert_eq!(state.gpr[RAX.0 as usize], 0x1122_3344_5566_CCBB);
    }

    #[test]
    fn read_high_byte() {
        let mut state = state();
        state.gpr[RAX.0 as usize] = 0x7788;
        let instruction = decode(&[0x90]);
        assert_eq!(read_register(&state, &instruction, Register::AH), Ok(0x77));
    }

    #[test]
    fn base_index_scale_displacement() {
        // mov rax, [rbx+rcx*4+0x10]
        let instruction = decode(&[0x48, 0x8B, 0x44, 0x8B, 0x10]);
        let mut state = state();
        state.gpr[RBX.0 as usize] = 0x1000;
        state.gpr[RCX.0 as usize] = 2;
        assert_eq!(effective_address(&state, &instruction, 1), Ok(0x1018));
    }

    #[test]
    fn rip_relative() {
        // mov rax, [rip+0x100], 7 bytes long, so the address is 0x1007 + 0x100
        let instruction = decode(&[0x48, 0x8B, 0x05, 0x00, 0x01, 0x00, 0x00]);
        assert_eq!(effective_address(&state(), &instruction, 1), Ok(0x1107));
    }

    #[test]
    fn fs_is_unimplemented() {
        // mov rax, fs:[0x28]
        let instruction = decode(&[0x64, 0x48, 0x8B, 0x04, 0x25, 0x28, 0x00, 0x00, 0x00]);
        assert_eq!(
            effective_address(&state(), &instruction, 1),
            Err(CpuError::Unimplemented(Address(0x1000)))
        );
    }

    #[test]
    fn immediate_is_sign_extended() {
        // mov rax, -1 (32-bit immediate)
        let instruction = decode(&[0x48, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF]);
        let memory = AddressSpace::new();
        assert_eq!(read(&state(), &memory, &instruction, 1), Ok(u64::MAX));
    }

    #[test]
    fn memory_operand_round_trip() {
        // mov [rbx], rax
        let instruction = decode(&[0x48, 0x89, 0x03]);
        let mut memory = AddressSpace::new();
        memory
            .map(
                Address(0x2000),
                0x1000,
                Permissions::READ | Permissions::WRITE,
            )
            .unwrap();
        let mut state = state();
        state.gpr[RBX.0 as usize] = 0x2000;
        write(
            &mut state,
            &mut memory,
            &instruction,
            0,
            0x1122_3344_5566_7788,
        )
        .unwrap();
        assert_eq!(
            read(&state, &memory, &instruction, 0),
            Ok(0x1122_3344_5566_7788)
        );
    }
}
