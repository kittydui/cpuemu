use emu_shared::{Address, Permissions};

use crate::{AddressSpace, PAGE_SHIFT, PAGE_SIZE, PageExt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Region {
    pub start: Address,
    pub size: u64,
    pub permissions: Permissions,
}

impl AddressSpace {
    pub fn regions(&self) -> Vec<Region> {
        let mut regions: Vec<Region> = Vec::new();
        for (&number, page) in &self.pages {
            if let Some(last) = regions.last_mut() {
                let next = last.start.page_number() + (last.size >> PAGE_SHIFT);
                if next == number && last.permissions == page.permissions {
                    last.size += PAGE_SIZE as u64;
                    continue;
                }
            }
            regions.push(Region {
                start: Address::from_page(number),
                size: PAGE_SIZE as u64,
                permissions: page.permissions,
            });
        }
        regions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn region(start: u64, size: u64, permissions: Permissions) -> Region {
        Region {
            start: Address(start),
            size,
            permissions,
        }
    }

    #[test]
    fn empty_space_has_no_regions() {
        assert!(AddressSpace::new().regions().is_empty());
    }

    #[test]
    fn single_mapping() {
        let mut space = AddressSpace::new();
        space.map(Address(0x1000), 0x3000, Permissions::READ).unwrap();
        assert_eq!(space.regions(), [region(0x1000, 0x3000, Permissions::READ)]);
    }

    #[test]
    fn adjacent_same_permissions_merge() {
        let mut space = AddressSpace::new();
        space.map(Address(0x1000), 0x1000, Permissions::READ).unwrap();
        space.map(Address(0x2000), 0x2000, Permissions::READ).unwrap();
        assert_eq!(space.regions(), [region(0x1000, 0x3000, Permissions::READ)]);
    }

    #[test]
    fn adjacent_different_permissions_split() {
        let mut space = AddressSpace::new();
        space.map(Address(0x1000), 0x1000, Permissions::READ).unwrap();
        space.map(Address(0x2000), 0x1000, Permissions::WRITE).unwrap();
        assert_eq!(
            space.regions(),
            [
                region(0x1000, 0x1000, Permissions::READ),
                region(0x2000, 0x1000, Permissions::WRITE),
            ]
        );
    }

    #[test]
    fn gap_splits() {
        let mut space = AddressSpace::new();
        space.map(Address(0x1000), 0x1000, Permissions::READ).unwrap();
        space.map(Address(0x3000), 0x1000, Permissions::READ).unwrap();
        assert_eq!(
            space.regions(),
            [
                region(0x1000, 0x1000, Permissions::READ),
                region(0x3000, 0x1000, Permissions::READ),
            ]
        );
    }

    #[test]
    fn protect_middle_splits_in_three() {
        let mut space = AddressSpace::new();
        space.map(Address(0x1000), 0x3000, Permissions::READ).unwrap();
        space.protect(Address(0x2000), 0x1000, Permissions::all()).unwrap();
        assert_eq!(
            space.regions(),
            [
                region(0x1000, 0x1000, Permissions::READ),
                region(0x2000, 0x1000, Permissions::all()),
                region(0x3000, 0x1000, Permissions::READ),
            ]
        );
    }

    #[test]
    fn last_page() {
        let mut space = AddressSpace::new();
        space.map(Address(0xFFFF_FFFF_FFFF_E000), 0x2000, Permissions::READ).unwrap();
        assert_eq!(space.regions(), [region(0xFFFF_FFFF_FFFF_E000, 0x2000, Permissions::READ)]);
    }
}
