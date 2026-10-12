use emu_arch_x86::X86Cpu;
use emu_core::{Cpu, Machine};
use emu_interpreter::{Breakpoints, run};
use emu_shared::{Address, Permissions};

const CODE_BASE: Address = Address(0x1000);

const CODE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/nop_example.bin"));

fn setup() -> Machine<X86Cpu> {
    let mut machine = Machine::new(X86Cpu::new());
    machine
        .memory
        .map(CODE_BASE, 0x1000, Permissions::READ | Permissions::EXECUTE)
        .unwrap();
    machine.memory.host_write(CODE_BASE, CODE).unwrap();
    machine.cpu.set_pc(CODE_BASE);
    machine
}

fn main() {
    let mut machine = setup();
    let exit = run(&mut machine, &mut Breakpoints::new(), Some(1000));

    println!("exit reason:  {:?}", exit.stop);
    println!("executed:     {} instructions", exit.executed);
    for register in machine.cpu.registers() {
        let mut bytes = vec![0; register.size];
        machine.cpu.read_register(register.id, &mut bytes);
        let hex: String = bytes
            .iter()
            .rev()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        println!("{:>8} = 0x{hex}", register.name);
    }
}

#[cfg(test)]
mod tests {
    use emu_core::CpuError;
    use emu_interpreter::{Exit, Stop};

    use super::*;

    #[test]
    fn runs_to_hlt() {
        let mut machine = setup();
        let exit = run(&mut machine, &mut Breakpoints::new(), Some(1000));
        assert_eq!(
            exit,
            Exit {
                stop: Stop::Cpu(CpuError::Halt),
                executed: 4,
            }
        );
        assert_eq!(machine.cpu.pc(), Address(CODE_BASE.0 + CODE.len() as u64));
    }
}
