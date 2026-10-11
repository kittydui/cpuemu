use crate::{MapError, PAGE_SHIFT, PAGE_SIZE, Page, PageExt};
use emu_shared::{Address, Permissions};
use std::collections::BTreeMap;

const LAST_PAGE: u64 = u64::MAX >> PAGE_SHIFT;

#[derive(Default)]
pub struct AddressSpace {
    pub(crate) pages: BTreeMap<u64, Page>,
}

impl AddressSpace {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn map(
        &mut self,
        address: Address,
        size: u64,
        permissions: Permissions,
    ) -> Result<(), MapError> {
        let (first, last) = page_range(address, size)?;
        if self.pages.range(first..=last).next().is_some() {
            return Err(MapError::Overlap);
        }
        for page in first..=last {
            self.pages.insert(page, Page::new(permissions));
        }
        Ok(())
    }

    pub fn unmap(&mut self, address: Address, size: u64) -> Result<(), MapError> {
        let (first, last) = page_range(address, size)?;
        self.require_mapped(first, last)?;
        for page in first..=last {
            self.pages.remove(&page);
        }
        Ok(())
    }

    pub fn protect(
        &mut self,
        address: Address,
        size: u64,
        permissions: Permissions,
    ) -> Result<(), MapError> {
        let (first, last) = page_range(address, size)?;
        self.require_mapped(first, last)?;
        for (_, page) in self.pages.range_mut(first..=last) {
            page.permissions = permissions;
        }
        Ok(())
    }

    fn require_mapped(&self, first: u64, last: u64) -> Result<(), MapError> {
        let mapped = self.pages.range(first..=last).count() as u64;
        if mapped == last - first + 1 {
            Ok(())
        } else {
            Err(MapError::NotMapped)
        }
    }
}

fn page_range(address: Address, size: u64) -> Result<(u64, u64), MapError> {
    if size == 0 {
        return Err(MapError::ZeroSize);
    }
    if address.page_offset() != 0 || size % PAGE_SIZE as u64 != 0 {
        return Err(MapError::Misaligned);
    }
    let first = address.page_number();
    let last = first + (size >> PAGE_SHIFT) - 1;
    if last > LAST_PAGE {
        return Err(MapError::Overflow);
    }
    Ok((first, last))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_rejects_misaligned() {
        let mut space = AddressSpace::new();
        assert_eq!(
            space.map(Address(0x1001), 0x1000, Permissions::READ),
            Err(MapError::Misaligned)
        );
        assert_eq!(
            space.map(Address(0x1000), 0x800, Permissions::READ),
            Err(MapError::Misaligned)
        );
        assert!(space.pages.is_empty());
    }

    #[test]
    fn map_rejects_zero_size() {
        let mut space = AddressSpace::new();
        assert_eq!(
            space.map(Address(0x1000), 0, Permissions::READ),
            Err(MapError::ZeroSize)
        );
    }

    #[test]
    fn map_rejects_overlap() {
        let mut space = AddressSpace::new();
        space
            .map(Address(0x2000), 0x2000, Permissions::READ)
            .unwrap();
        assert_eq!(
            space.map(Address(0x1000), 0x2000, Permissions::READ),
            Err(MapError::Overlap)
        );
        assert_eq!(space.pages.len(), 2);
        assert!(!space.pages.contains_key(&1));
    }

    #[test]
    fn map_last_page() {
        let mut space = AddressSpace::new();
        space
            .map(Address(0xFFFF_FFFF_FFFF_F000), 0x1000, Permissions::READ)
            .unwrap();
        assert!(space.pages.contains_key(&LAST_PAGE));
    }

    #[test]
    fn map_rejects_past_end() {
        let mut space = AddressSpace::new();
        assert_eq!(
            space.map(Address(0xFFFF_FFFF_FFFF_F000), 0x2000, Permissions::READ),
            Err(MapError::Overflow)
        );
    }

    #[test]
    fn unmap_part_of_mapping() {
        let mut space = AddressSpace::new();
        space
            .map(Address(0x1000), 0x3000, Permissions::READ)
            .unwrap();
        space.unmap(Address(0x2000), 0x1000).unwrap();
        assert!(space.pages.contains_key(&1));
        assert!(!space.pages.contains_key(&2));
        assert!(space.pages.contains_key(&3));
    }

    #[test]
    fn unmap_over_gap_changes_nothing() {
        let mut space = AddressSpace::new();
        space
            .map(Address(0x1000), 0x1000, Permissions::READ)
            .unwrap();
        space
            .map(Address(0x3000), 0x1000, Permissions::READ)
            .unwrap();
        assert_eq!(
            space.unmap(Address(0x1000), 0x3000),
            Err(MapError::NotMapped)
        );
        assert_eq!(space.pages.len(), 2);
    }

    #[test]
    fn protect_part_of_mapping() {
        let mut space = AddressSpace::new();
        space
            .map(Address(0x1000), 0x3000, Permissions::READ)
            .unwrap();
        space
            .protect(
                Address(0x2000),
                0x1000,
                Permissions::READ | Permissions::WRITE,
            )
            .unwrap();
        assert_eq!(space.pages[&1].permissions, Permissions::READ);
        assert_eq!(
            space.pages[&2].permissions,
            Permissions::READ | Permissions::WRITE
        );
        assert_eq!(space.pages[&3].permissions, Permissions::READ);
    }

    #[test]
    fn protect_over_gap_changes_nothing() {
        let mut space = AddressSpace::new();
        space
            .map(Address(0x1000), 0x1000, Permissions::READ)
            .unwrap();
        space
            .map(Address(0x3000), 0x1000, Permissions::READ)
            .unwrap();
        assert_eq!(
            space.protect(Address(0x1000), 0x3000, Permissions::WRITE),
            Err(MapError::NotMapped)
        );
        assert_eq!(space.pages[&1].permissions, Permissions::READ);
        assert_eq!(space.pages[&3].permissions, Permissions::READ);
    }
}
