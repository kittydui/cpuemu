use emu_core::{Register, RegisterId};

use crate::Flags;

pub const RAX: RegisterId = RegisterId(0);
pub const RCX: RegisterId = RegisterId(1);
pub const RDX: RegisterId = RegisterId(2);
pub const RBX: RegisterId = RegisterId(3);
pub const RSP: RegisterId = RegisterId(4);
pub const RBP: RegisterId = RegisterId(5);
pub const RSI: RegisterId = RegisterId(6);
pub const RDI: RegisterId = RegisterId(7);
pub const R8: RegisterId = RegisterId(8);
pub const R9: RegisterId = RegisterId(9);
pub const R10: RegisterId = RegisterId(10);
pub const R11: RegisterId = RegisterId(11);
pub const R12: RegisterId = RegisterId(12);
pub const R13: RegisterId = RegisterId(13);
pub const R14: RegisterId = RegisterId(14);
pub const R15: RegisterId = RegisterId(15);
pub const RIP: RegisterId = RegisterId(16);
pub const RFLAGS: RegisterId = RegisterId(17);

pub static REGISTERS: [Register; 18] = [
    Register {
        id: RAX,
        name: "rax",
        size: 8,
    },
    Register {
        id: RCX,
        name: "rcx",
        size: 8,
    },
    Register {
        id: RDX,
        name: "rdx",
        size: 8,
    },
    Register {
        id: RBX,
        name: "rbx",
        size: 8,
    },
    Register {
        id: RSP,
        name: "rsp",
        size: 8,
    },
    Register {
        id: RBP,
        name: "rbp",
        size: 8,
    },
    Register {
        id: RSI,
        name: "rsi",
        size: 8,
    },
    Register {
        id: RDI,
        name: "rdi",
        size: 8,
    },
    Register {
        id: R8,
        name: "r8",
        size: 8,
    },
    Register {
        id: R9,
        name: "r9",
        size: 8,
    },
    Register {
        id: R10,
        name: "r10",
        size: 8,
    },
    Register {
        id: R11,
        name: "r11",
        size: 8,
    },
    Register {
        id: R12,
        name: "r12",
        size: 8,
    },
    Register {
        id: R13,
        name: "r13",
        size: 8,
    },
    Register {
        id: R14,
        name: "r14",
        size: 8,
    },
    Register {
        id: R15,
        name: "r15",
        size: 8,
    },
    Register {
        id: RIP,
        name: "rip",
        size: 8,
    },
    Register {
        id: RFLAGS,
        name: "rflags",
        size: 8,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct X86State {
    pub gpr: [u64; 16],
    pub rip: u64,
    pub rflags: Flags,
}
