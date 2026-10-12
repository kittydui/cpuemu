use std::collections::BTreeSet;

use emu_shared::Address;

#[derive(Debug, Clone, Default)]
pub struct Breakpoints {
    addresses: BTreeSet<Address>,
    pub(crate) resume: Option<Address>,
}

impl Breakpoints {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, address: Address) -> bool {
        self.addresses.insert(address)
    }

    pub fn remove(&mut self, address: Address) -> bool {
        self.addresses.remove(&address)
    }

    pub fn contains(&self, address: Address) -> bool {
        self.addresses.contains(&address)
    }

    pub fn iter(&self) -> impl Iterator<Item = Address> + '_ {
        self.addresses.iter().copied()
    }
}
