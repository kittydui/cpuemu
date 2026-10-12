use bitflags::bitflags;
use iced_x86::ConditionCode;

use crate::operand::Size;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Flags: u64 {
        const CF = 1 << 0;
        const RESERVED = 1 << 1;
        const PF = 1 << 2;
        const AF = 1 << 4;
        const ZF = 1 << 6;
        const SF = 1 << 7;
        const TF = 1 << 8;
        const IF = 1 << 9;
        const DF = 1 << 10;
        const OF = 1 << 11;
    }
}

impl Flags {
    pub const RESET: Self = Self::RESERVED;

    pub const STATUS: Self = Self::CF
        .union(Self::PF)
        .union(Self::AF)
        .union(Self::ZF)
        .union(Self::SF)
        .union(Self::OF);

    // Replaces the flags in `mask` with those set in `values`, leaving the rest alone.
    pub(crate) fn update(&mut self, mask: Flags, values: Flags) {
        *self = (*self - mask) | (values & mask);
    }
}

// Jump-condition.
pub(crate) fn jc(flags: Flags, code: ConditionCode) -> bool {
    let (cf, zf, sf, of, pf) = (
        flags.contains(Flags::CF),
        flags.contains(Flags::ZF),
        flags.contains(Flags::SF),
        flags.contains(Flags::OF),
        flags.contains(Flags::PF),
    );
    match code {
        ConditionCode::o => of,
        ConditionCode::no => !of,
        ConditionCode::b => cf,
        ConditionCode::ae => !cf,
        ConditionCode::e => zf,
        ConditionCode::ne => !zf,
        ConditionCode::be => cf || zf,
        ConditionCode::a => !cf && !zf,
        ConditionCode::s => sf,
        ConditionCode::ns => !sf,
        ConditionCode::p => pf,
        ConditionCode::np => !pf,
        ConditionCode::l => sf != of,
        ConditionCode::ge => sf == of,
        ConditionCode::le => zf || sf != of,
        ConditionCode::g => !zf && sf == of,
        ConditionCode::None => false,
    }
}

pub(crate) fn add(a: u64, b: u64, size: Size) -> (u64, Flags) {
    // mask a and b to the correct size so we dont always just add r* registers (rax, rbx, etc.)
    let (a, b) = (a & size.mask(), b & size.mask());
    // add b to a.
    let result = a.wrapping_add(b) & size.mask();

    let mut flags = result_flags(result, size);
    // set carry flag if result is lt a, (didn't fit as an unsigned number).
    flags.set(Flags::CF, result < a);

    // a carry came out of bit 3 into bit 4. `a ^ b` is the sum without carries,
    // so XORing it with the real result leaves exactly the bits a carry arrived at.
    flags.set(Flags::AF, (a ^ b ^ result) & 0x10 != 0);

    // set overflow flag if the result didn't fit as a signed number.
    flags.set(
        Flags::OF,
        (a ^ result) & (b ^ result) & size.sign_bit() != 0,
    );
    (result, flags)
}

pub(crate) fn sub(a: u64, b: u64, size: Size) -> (u64, Flags) {
    let (a, b) = (a & size.mask(), b & size.mask());
    let result = a.wrapping_sub(b) & size.mask();
    let mut flags = result_flags(result, size);
    flags.set(Flags::CF, a < b);
    flags.set(Flags::AF, (a ^ b ^ result) & 0x10 != 0);
    flags.set(Flags::OF, (a ^ b) & (a ^ result) & size.sign_bit() != 0);
    (result, flags)
}

pub(crate) fn logic(result: u64, size: Size) -> Flags {
    // the caller has already computed the result so
    // all we do here is set the result flags.
    result_flags(result & size.mask(), size)
}

fn result_flags(result: u64, size: Size) -> Flags {
    let mut flags = Flags::empty();
    // set Zero Flag if the resulting number is 0
    flags.set(Flags::ZF, result == 0);
    // copy the result's top bit to Sign Flag, 1 if negative (signed), 0 (unsigned).
    flags.set(Flags::SF, result & size.sign_bit() != 0);
    // set Parity Flag if low byte of the result has even number of 1 bits.
    flags.set(Flags::PF, (result as u8).count_ones() % 2 == 0);
    flags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_signed_overflow() {
        assert_eq!(
            add(0x7F, 1, Size::Byte),
            (0x80, Flags::SF | Flags::AF | Flags::OF)
        );
    }

    #[test]
    fn add_carry_to_zero() {
        let carry = Flags::CF | Flags::ZF | Flags::PF | Flags::AF;
        assert_eq!(add(0xFF, 1, Size::Byte), (0, carry));
        assert_eq!(add(u64::MAX, 1, Size::Qword), (0, carry));
    }

    #[test]
    fn add_ignores_bits_above_size() {
        assert_eq!(add(0x1FF, 1, Size::Byte), add(0xFF, 1, Size::Byte));
    }

    #[test]
    fn sub_borrow() {
        assert_eq!(
            sub(0, 1, Size::Byte),
            (0xFF, Flags::CF | Flags::SF | Flags::PF | Flags::AF)
        );
    }

    #[test]
    fn sub_signed_overflow() {
        assert_eq!(sub(0x80, 1, Size::Byte), (0x7F, Flags::AF | Flags::OF));
    }

    #[test]
    fn sub_equal_is_zero() {
        assert_eq!(sub(5, 5, Size::Dword), (0, Flags::ZF | Flags::PF));
    }

    #[test]
    fn logic_flags() {
        assert_eq!(logic(0x80, Size::Byte), Flags::SF);
        assert_eq!(logic(0, Size::Qword), Flags::ZF | Flags::PF);
    }

    #[test]
    fn parity_uses_low_byte_only() {
        assert_eq!(logic(0x0300, Size::Word), Flags::PF);
    }

    #[test]
    fn update_keeps_flags_outside_mask() {
        let mut flags = Flags::RESET | Flags::CF | Flags::DF;
        flags.update(Flags::STATUS - Flags::CF, Flags::ZF | Flags::CF);
        assert_eq!(flags, Flags::RESET | Flags::CF | Flags::DF | Flags::ZF);
    }
}
