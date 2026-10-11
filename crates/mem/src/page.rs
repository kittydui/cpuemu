use emu_shared::{Address, Permissions};

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SHIFT: u32 = 12;

pub(crate) struct Page {
    pub permissions: Permissions,
    pub data: Box<[u8; PAGE_SIZE]>,
}

impl Page {
    pub fn new(perms: Permissions) -> Self {
        Self {
            permissions: perms,
            data: Box::new([0; PAGE_SIZE]),
        }
    }
}

// Address
pub trait PageExt {
    fn page_number(self) -> u64;
    fn page_offset(self) -> usize;
    fn from_page(page: u64) -> Self;
}

impl PageExt for Address {
    fn page_number(self) -> u64 {
        self.0 >> PAGE_SHIFT
    }

    fn page_offset(self) -> usize {
        (self.0 & (PAGE_SIZE as u64 - 1)) as usize
    }

    fn from_page(page: u64) -> Self {
        Address(page << PAGE_SHIFT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_page_is_zeroed() {
        let page = Page::new(Permissions::READ | Permissions::WRITE);
        assert_eq!(page.permissions, Permissions::READ | Permissions::WRITE);
        assert_eq!(page.data.len(), PAGE_SIZE);
        assert!(page.data.iter().all(|&byte| byte == 0));
    }

    #[test]
    fn page_number() {
        assert_eq!(Address(0).page_number(), 0);
        assert_eq!(Address(0xFFF).page_number(), 0);
        assert_eq!(Address(0x1000).page_number(), 1);
        assert_eq!(Address(0x1234_5678).page_number(), 0x12345);
        assert_eq!(Address(u64::MAX).page_number(), u64::MAX >> PAGE_SHIFT);
    }

    #[test]
    fn page_offset() {
        assert_eq!(Address(0x1000).page_offset(), 0);
        assert_eq!(Address(0xFFF).page_offset(), 0xFFF);
        assert_eq!(Address(0x1234_5678).page_offset(), 0x678);
    }

    #[test]
    fn from_page() {
        assert_eq!(Address::from_page(0), Address(0));
        assert_eq!(Address::from_page(0x12345), Address(0x1234_5000));
        assert_eq!(
            Address::from_page(u64::MAX >> PAGE_SHIFT),
            Address(0xFFFF_FFFF_FFFF_F000)
        );
    }

    #[test]
    fn from_page_round_trips() {
        let address = Address(0x7FFF_1234_5678);
        let start = Address::from_page(address.page_number());
        assert_eq!(start, Address(0x7FFF_1234_5000));
        assert_eq!(start.0 + address.page_offset() as u64, address.0);
    }
}
