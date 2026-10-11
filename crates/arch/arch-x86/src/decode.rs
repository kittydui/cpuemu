use emu_core::CpuError;
use emu_mem::AddressSpace;
use emu_shared::Address;
use iced_x86::{Decoder, DecoderError, DecoderOptions, Instruction};

const MAX_INSTRUCTION_LEN: usize = 15;

pub(crate) fn decode(memory: &AddressSpace, rip: u64) -> Result<Instruction, CpuError> {
    let mut bytes = [0; MAX_INSTRUCTION_LEN];
    let fetched = memory
        .fetch(Address(rip), &mut bytes)
        .map_err(CpuError::Fault)?;
    let mut decoder = Decoder::with_ip(64, &bytes[..fetched], rip, DecoderOptions::NONE);
    let instruction = decoder.decode();
    match decoder.last_error() {
        DecoderError::None => Ok(instruction),
        DecoderError::NoMoreBytes => {
            let next = Address(rip.wrapping_add(fetched as u64));
            Err(match memory.fetch(next, &mut [0]) {
                Err(fault) => CpuError::Fault(fault),
                Ok(_) => CpuError::IllegalInstruction(Address(rip)),
            })
        }
        _ => Err(CpuError::IllegalInstruction(Address(rip))),
    }
}
