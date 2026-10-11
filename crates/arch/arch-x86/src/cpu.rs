use emu_core::{Cpu, CpuError, Register, RegisterId};
use emu_mem::AddressSpace;
use emu_shared::{Address, Endian};

use crate::{Flags, REGISTERS, RFLAGS, RIP, X86State, decode, exec};

#[derive(Debug, Clone)]
pub struct X86Cpu {
    pub state: X86State,
}

impl X86Cpu {
    pub fn new() -> Self {
        Self {
            state: X86State {
                gpr: [0; 16],
                rip: 0,
                rflags: Flags::RESET,
            },
        }
    }
}

impl Default for X86Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu for X86Cpu {
    fn registers(&self) -> &'static [Register] {
        &REGISTERS
    }

    fn read_register(&self, id: RegisterId, out: &mut [u8]) {
        let value = match id {
            RIP => self.state.rip,
            RFLAGS => self.state.rflags.bits(),
            RegisterId(n @ 0..16) => self.state.gpr[n as usize],
            _ => panic!("unknown x86 register {id:?}"),
        };
        out.copy_from_slice(&value.to_le_bytes());
    }

    fn write_register(&mut self, id: RegisterId, value: &[u8]) {
        let value = u64::from_le_bytes(value.try_into().expect("x86 registers are 8 bytes"));
        match id {
            RIP => self.state.rip = value,
            RFLAGS => self.state.rflags = Flags::from_bits_retain(value),
            RegisterId(n @ 0..16) => self.state.gpr[n as usize] = value,
            _ => panic!("unknown x86 register {id:?}"),
        }
    }

    fn pc(&self) -> Address {
        Address(self.state.rip)
    }

    fn set_pc(&mut self, pc: Address) {
        self.state.rip = pc.0;
    }

    fn endian(&self) -> Endian {
        Endian::Little
    }

    fn step(&mut self, memory: &mut AddressSpace) -> Result<(), CpuError> {
        let instruction = decode::decode(memory, self.state.rip)?;
        let mut next = self.state;
        exec::execute(&mut next, memory, &instruction)?;
        self.state = next;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use emu_core::Machine;
    use emu_shared::{Access, Fault, FaultKind, Permissions};

    use super::*;
    use crate::RAX;

    const RX: Permissions = Permissions::READ.union(Permissions::EXECUTE);

    fn machine(code: &[u8], at: u64) -> Machine<X86Cpu> {
        let mut machine = Machine::new(X86Cpu::new());
        machine.memory.map(Address(0x1000), 0x1000, RX).unwrap();
        machine.memory.host_write(Address(at), code).unwrap();
        machine.cpu.set_pc(Address(at));
        machine
    }

    fn fetch_fault(address: u64, kind: FaultKind) -> CpuError {
        CpuError::Fault(Fault {
            address: Address(address),
            access: Access::Fetch,
            kind,
        })
    }

    #[test]
    fn reset_rflags() {
        let mut out = [0; 8];
        X86Cpu::new().read_register(RFLAGS, &mut out);
        assert_eq!(u64::from_le_bytes(out), 0x2);
    }

    #[test]
    fn register_round_trip() {
        let mut cpu = X86Cpu::new();
        cpu.write_register(RAX, &0x1122_3344_5566_7788u64.to_le_bytes());
        let mut out = [0; 8];
        cpu.read_register(RAX, &mut out);
        assert_eq!(u64::from_le_bytes(out), 0x1122_3344_5566_7788);
    }

    #[test]
    fn nop_advances_rip() {
        // nop; nop dword ptr [rax+rax]
        let mut machine = machine(&[0x90, 0x0F, 0x1F, 0x44, 0x00, 0x00], 0x1000);
        machine.step().unwrap();
        assert_eq!(machine.cpu.pc(), Address(0x1001));
        machine.step().unwrap();
        assert_eq!(machine.cpu.pc(), Address(0x1006));
    }

    #[test]
    fn fetch_from_unmapped_faults() {
        let mut machine = machine(&[], 0x1000);
        machine.cpu.set_pc(Address(0x5000));
        assert_eq!(
            machine.step(),
            Err(fetch_fault(0x5000, FaultKind::Unmapped))
        );
        assert_eq!(machine.cpu.pc(), Address(0x5000));
    }

    #[test]
    fn instruction_crossing_into_unmapped_page_faults_there() {
        // First two bytes of a 5-byte nop, at the end of the page.
        let mut machine = machine(&[0x0F, 0x1F], 0x1FFE);
        assert_eq!(
            machine.step(),
            Err(fetch_fault(0x2000, FaultKind::Unmapped))
        );
        assert_eq!(machine.cpu.pc(), Address(0x1FFE));
    }

    #[test]
    fn invalid_instruction() {
        // push es is invalid in 64-bit mode.
        let mut machine = machine(&[0x06], 0x1000);
        assert_eq!(
            machine.step(),
            Err(CpuError::IllegalInstruction(Address(0x1000)))
        );
    }

    #[test]
    fn unimplemented_instruction() {
        // cpuid
        let mut machine = machine(&[0x0F, 0xA2], 0x1000);
        assert_eq!(
            machine.step(),
            Err(CpuError::Unimplemented(Address(0x1000)))
        );
    }

    #[test]
    fn hlt_stops_without_moving_rip() {
        let mut machine = machine(&[0xF4], 0x1000);
        assert_eq!(machine.step(), Err(CpuError::Halt));
        assert_eq!(machine.cpu.pc(), Address(0x1000));
    }
}
