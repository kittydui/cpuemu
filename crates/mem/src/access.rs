use std::ops::Range;

use emu_shared::{Access, Address, Endian, Fault, FaultKind, Value};

use crate::{AddressSpace, PAGE_SIZE, Page, PageExt};

// Large enough for the biggest `Value` (u128).
const MAX_VALUE_SIZE: usize = 16;

impl AddressSpace {
    pub fn read(&self, address: Address, buffer: &mut [u8]) -> Result<(), Fault> {
        self.read_bytes(address, buffer, true)
    }

    pub fn write(&mut self, address: Address, data: &[u8]) -> Result<(), Fault> {
        self.write_bytes(address, data, true)
    }

    // Typed guest access in the CPU's byte order, e.g. `read_value::<u32>(address, Endian::Big)`.
    pub fn read_value<T: Value>(&self, address: Address, endian: Endian) -> Result<T, Fault> {
        let mut bytes = [0; MAX_VALUE_SIZE];
        let bytes = &mut bytes[..T::SIZE];
        self.read(address, bytes)?;
        Ok(T::from_bytes(bytes, endian))
    }

    pub fn write_value<T: Value>(
        &mut self,
        address: Address,
        value: T,
        endian: Endian,
    ) -> Result<(), Fault> {
        let mut bytes = [0; MAX_VALUE_SIZE];
        let bytes = &mut bytes[..T::SIZE];
        value.to_bytes(endian, bytes);
        self.write(address, bytes)
    }

    // Fills as much of `buffer` as is executable and returns how many bytes it got.
    // Only faults if not even the first byte can be fetched.
    pub fn fetch(&self, address: Address, buffer: &mut [u8]) -> Result<usize, Fault> {
        let mut fetched = 0;
        for chunk in chunks(address, buffer.len()) {
            match self.page(&chunk, Access::Fetch, true) {
                Ok(page) => {
                    let end = chunk.in_buffer.end;
                    buffer[chunk.in_buffer].copy_from_slice(&page.data[chunk.in_page]);
                    fetched = end;
                }
                Err(fault) if fetched == 0 => return Err(fault),
                Err(_) => break,
            }
        }
        Ok(fetched)
    }

    // Ignores permissions.
    pub fn host_read(&self, address: Address, buffer: &mut [u8]) -> Result<(), Fault> {
        self.read_bytes(address, buffer, false)
    }

    pub fn host_write(&mut self, address: Address, data: &[u8]) -> Result<(), Fault> {
        self.write_bytes(address, data, false)
    }

    fn read_bytes(
        &self,
        address: Address,
        buffer: &mut [u8],
        check_permissions: bool,
    ) -> Result<(), Fault> {
        for chunk in chunks(address, buffer.len()) {
            let page = self.page(&chunk, Access::Read, check_permissions)?;
            buffer[chunk.in_buffer].copy_from_slice(&page.data[chunk.in_page]);
        }
        Ok(())
    }

    // Checks every page before writing anything, so a faulting write leaves memory unchanged.
    fn write_bytes(
        &mut self,
        address: Address,
        data: &[u8],
        check_permissions: bool,
    ) -> Result<(), Fault> {
        for chunk in chunks(address, data.len()) {
            self.page(&chunk, Access::Write, check_permissions)?;
        }
        for chunk in chunks(address, data.len()) {
            let page = self.pages.get_mut(&chunk.page).expect("checked above");
            page.data[chunk.in_page].copy_from_slice(&data[chunk.in_buffer]);
        }
        Ok(())
    }

    fn page(&self, chunk: &Chunk, access: Access, check_permissions: bool) -> Result<&Page, Fault> {
        let fault = |kind| Fault {
            address: chunk.address,
            access,
            kind,
        };
        let page = self
            .pages
            .get(&chunk.page)
            .ok_or_else(|| fault(FaultKind::Unmapped))?;
        if check_permissions && !page.permissions.contains(access.required()) {
            return Err(fault(FaultKind::Protection));
        }
        Ok(page)
    }
}

struct Chunk {
    address: Address,
    page: u64,
    in_page: Range<usize>,
    in_buffer: Range<usize>,
}

fn chunks(address: Address, len: usize) -> impl Iterator<Item = Chunk> {
    let mut done = 0;
    std::iter::from_fn(move || {
        if done == len {
            return None;
        }
        let current = Address(address.0.wrapping_add(done as u64));
        let offset = current.page_offset();
        let size = (PAGE_SIZE - offset).min(len - done);
        let chunk = Chunk {
            address: current,
            page: current.page_number(),
            in_page: offset..offset + size,
            in_buffer: done..done + size,
        };
        done += size;
        Some(chunk)
    })
}

#[cfg(test)]
mod tests {
    use emu_shared::Permissions;

    use super::*;

    const RW: Permissions = Permissions::READ.union(Permissions::WRITE);
    const RX: Permissions = Permissions::READ.union(Permissions::EXECUTE);

    #[test]
    fn typed_values_use_the_given_byte_order() {
        let mut space = AddressSpace::new();
        space.map(Address(0x1000), 0x1000, RW).unwrap();
        space
            .write_value(Address(0x1000), 0x1122_3344u32, Endian::Little)
            .unwrap();
        let mut bytes = [0; 4];
        space.read(Address(0x1000), &mut bytes).unwrap();
        assert_eq!(bytes, [0x44, 0x33, 0x22, 0x11]);
        assert_eq!(
            space.read_value::<u32>(Address(0x1000), Endian::Big),
            Ok(0x4433_2211)
        );
    }

    #[test]
    fn typed_read_across_pages_faults_like_a_byte_read() {
        let mut space = AddressSpace::new();
        space.map(Address(0x1000), 0x1000, RW).unwrap();
        assert_eq!(
            space.read_value::<u64>(Address(0x1FFC), Endian::Little),
            Err(fault(0x2000, Access::Read, FaultKind::Unmapped))
        );
    }

    fn fault(address: u64, access: Access, kind: FaultKind) -> Fault {
        Fault {
            address: Address(address),
            access,
            kind,
        }
    }

    #[test]
    fn write_then_read() {
        let mut space = AddressSpace::new();
        space.map(Address(0x1000), 0x1000, RW).unwrap();
        space.write(Address(0x1100), &[1, 2, 3, 4]).unwrap();
        let mut buffer = [0; 4];
        space.read(Address(0x1100), &mut buffer).unwrap();
        assert_eq!(buffer, [1, 2, 3, 4]);
    }

    #[test]
    fn read_across_pages() {
        let mut space = AddressSpace::new();
        space.map(Address(0x1000), 0x2000, RW).unwrap();
        space
            .write(Address(0x1FFC), &[1, 2, 3, 4, 5, 6, 7, 8])
            .unwrap();
        let mut buffer = [0; 8];
        space.read(Address(0x1FFC), &mut buffer).unwrap();
        assert_eq!(buffer, [1, 2, 3, 4, 5, 6, 7, 8]);
        let mut second = [0; 4];
        space.read(Address(0x2000), &mut second).unwrap();
        assert_eq!(second, [5, 6, 7, 8]);
    }

    #[test]
    fn read_unmapped_faults_at_start() {
        let space = AddressSpace::new();
        let mut buffer = [0; 4];
        assert_eq!(
            space.read(Address(0x5000), &mut buffer),
            Err(fault(0x5000, Access::Read, FaultKind::Unmapped))
        );
    }

    #[test]
    fn read_into_unmapped_page_faults_at_page_start() {
        let mut space = AddressSpace::new();
        space.map(Address(0x1000), 0x1000, RW).unwrap();
        let mut buffer = [0; 8];
        assert_eq!(
            space.read(Address(0x1FFC), &mut buffer),
            Err(fault(0x2000, Access::Read, FaultKind::Unmapped))
        );
    }

    #[test]
    fn write_into_unmapped_page_changes_nothing() {
        let mut space = AddressSpace::new();
        space.map(Address(0x1000), 0x1000, RW).unwrap();
        assert_eq!(
            space.write(Address(0x1FFC), &[0xAA; 8]),
            Err(fault(0x2000, Access::Write, FaultKind::Unmapped))
        );
        let mut buffer = [0xFF; 4];
        space.read(Address(0x1FFC), &mut buffer).unwrap();
        assert_eq!(buffer, [0; 4]);
    }

    #[test]
    fn write_to_read_only_faults() {
        let mut space = AddressSpace::new();
        space
            .map(Address(0x1000), 0x1000, Permissions::READ)
            .unwrap();
        assert_eq!(
            space.write(Address(0x1000), &[1]),
            Err(fault(0x1000, Access::Write, FaultKind::Protection))
        );
    }

    #[test]
    fn read_without_read_permission_faults() {
        let mut space = AddressSpace::new();
        space
            .map(Address(0x1000), 0x1000, Permissions::WRITE)
            .unwrap();
        let mut buffer = [0; 1];
        assert_eq!(
            space.read(Address(0x1000), &mut buffer),
            Err(fault(0x1000, Access::Read, FaultKind::Protection))
        );
    }

    #[test]
    fn fetch_stops_at_non_executable_page() {
        let mut space = AddressSpace::new();
        space.map(Address(0x1000), 0x1000, RX).unwrap();
        space.map(Address(0x2000), 0x1000, RW).unwrap();
        space
            .host_write(Address(0x1FF8), &[1, 2, 3, 4, 5, 6, 7, 8])
            .unwrap();
        let mut buffer = [0; 15];
        assert_eq!(space.fetch(Address(0x1FF8), &mut buffer), Ok(8));
        assert_eq!(buffer[..8], [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn fetch_stops_at_unmapped_page() {
        let mut space = AddressSpace::new();
        space.map(Address(0x1000), 0x1000, RX).unwrap();
        let mut buffer = [0; 15];
        assert_eq!(space.fetch(Address(0x1FF8), &mut buffer), Ok(8));
    }

    #[test]
    fn fetch_faults_when_first_byte_unavailable() {
        let mut space = AddressSpace::new();
        space.map(Address(0x1000), 0x1000, RW).unwrap();
        let mut buffer = [0; 15];
        assert_eq!(
            space.fetch(Address(0x1000), &mut buffer),
            Err(fault(0x1000, Access::Fetch, FaultKind::Protection))
        );
        assert_eq!(
            space.fetch(Address(0x5000), &mut buffer),
            Err(fault(0x5000, Access::Fetch, FaultKind::Unmapped))
        );
    }

    #[test]
    fn host_access_ignores_permissions() {
        let mut space = AddressSpace::new();
        space.map(Address(0x1000), 0x1000, RX).unwrap();
        space.host_write(Address(0x1000), &[0x90, 0xC3]).unwrap();
        let mut buffer = [0; 2];
        space.host_read(Address(0x1000), &mut buffer).unwrap();
        assert_eq!(buffer, [0x90, 0xC3]);
        assert!(space.write(Address(0x1000), &[0]).is_err());
    }

    #[test]
    fn host_access_faults_on_unmapped() {
        let space = AddressSpace::new();
        let mut buffer = [0; 1];
        assert_eq!(
            space.host_read(Address(0x5000), &mut buffer),
            Err(fault(0x5000, Access::Read, FaultKind::Unmapped))
        );
    }
}
