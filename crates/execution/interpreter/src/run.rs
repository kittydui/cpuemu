use emu_core::{Cpu, CpuError, Machine};
use emu_shared::Address;

use crate::Breakpoints;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stop {
    Cpu(CpuError),
    Breakpoint(Address),
    InstructionLimit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Exit {
    pub stop: Stop,
    pub executed: u64,
}

pub fn run<C: Cpu>(
    machine: &mut Machine<C>,
    breakpoints: &mut Breakpoints,
    limit: Option<u64>,
) -> Exit {
    let resume = breakpoints.resume.take();
    let mut executed = 0;
    loop {
        let pc = machine.cpu.pc();
        let resuming = executed == 0 && resume == Some(pc);
        if breakpoints.contains(pc) && !resuming {
            breakpoints.resume = Some(pc);
            return Exit {
                stop: Stop::Breakpoint(pc),
                executed,
            };
        }
        if limit.is_some_and(|limit| executed >= limit) {
            return Exit {
                stop: Stop::InstructionLimit,
                executed,
            };
        }
        if let Err(error) = machine.step() {
            if error == CpuError::Halt {
                executed += 1;
            }
            return Exit {
                stop: Stop::Cpu(error),
                executed,
            };
        }
        executed += 1;
    }
}

#[cfg(test)]
mod tests {
    use emu_arch_x86::X86Cpu;
    use emu_shared::{Access, Fault, FaultKind, Permissions};

    use super::*;

    const NOP: u8 = 0x90;
    const HLT: u8 = 0xF4;

    fn machine(code: &[u8]) -> Machine<X86Cpu> {
        let mut machine = Machine::new(X86Cpu::new());
        machine
            .memory
            .map(
                Address(0x1000),
                0x1000,
                Permissions::READ | Permissions::EXECUTE,
            )
            .unwrap();
        machine.memory.host_write(Address(0x1000), code).unwrap();
        machine.cpu.set_pc(Address(0x1000));
        machine
    }

    #[test]
    fn runs_until_halt() {
        let mut machine = machine(&[NOP, NOP, HLT]);
        let exit = run(&mut machine, &mut Breakpoints::new(), None);
        assert_eq!(
            exit,
            Exit {
                stop: Stop::Cpu(CpuError::Halt),
                executed: 3
            }
        );
        assert_eq!(machine.cpu.pc(), Address(0x1003));
    }

    #[test]
    fn stops_at_instruction_limit() {
        let mut machine = machine(&[NOP; 10]);
        let exit = run(&mut machine, &mut Breakpoints::new(), Some(3));
        assert_eq!(
            exit,
            Exit {
                stop: Stop::InstructionLimit,
                executed: 3
            }
        );
        assert_eq!(machine.cpu.pc(), Address(0x1003));
    }

    #[test]
    fn stops_at_breakpoint_and_resumes_past_it() {
        let mut machine = machine(&[NOP, NOP, NOP, HLT]);
        let mut breakpoints = Breakpoints::new();
        breakpoints.add(Address(0x1002));
        let exit = run(&mut machine, &mut breakpoints, None);
        assert_eq!(
            exit,
            Exit {
                stop: Stop::Breakpoint(Address(0x1002)),
                executed: 2
            }
        );
        let exit = run(&mut machine, &mut breakpoints, None);
        assert_eq!(
            exit,
            Exit {
                stop: Stop::Cpu(CpuError::Halt),
                executed: 2
            }
        );
    }

    #[test]
    fn reports_faults() {
        let mut machine = machine(&[]);
        machine.cpu.set_pc(Address(0x5000));
        let exit = run(&mut machine, &mut Breakpoints::new(), None);
        let fault = Fault {
            address: Address(0x5000),
            access: Access::Fetch,
            kind: FaultKind::Unmapped,
        };
        assert_eq!(
            exit,
            Exit {
                stop: Stop::Cpu(CpuError::Fault(fault)),
                executed: 0
            }
        );
    }

    #[test]
    fn breakpoint_at_entry_fires() {
        let mut machine = machine(&[NOP, HLT]);
        let mut breakpoints = Breakpoints::new();
        breakpoints.add(Address(0x1000));
        let exit = run(&mut machine, &mut breakpoints, None);
        assert_eq!(
            exit,
            Exit {
                stop: Stop::Breakpoint(Address(0x1000)),
                executed: 0
            }
        );
        let exit = run(&mut machine, &mut breakpoints, None);
        assert_eq!(exit.stop, Stop::Cpu(CpuError::Halt));
    }

    #[test]
    fn breakpoint_wins_over_limit() {
        let mut machine = machine(&[NOP, NOP, NOP, HLT]);
        let mut breakpoints = Breakpoints::new();
        breakpoints.add(Address(0x1002));
        let exit = run(&mut machine, &mut breakpoints, Some(2));
        assert_eq!(
            exit,
            Exit {
                stop: Stop::Breakpoint(Address(0x1002)),
                executed: 2
            }
        );
    }
}
