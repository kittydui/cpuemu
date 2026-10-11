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
}
