use crate::{Access, Address};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fault {
    pub address: Address,
    pub access: Access,
    pub kind: FaultKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FaultKind {
    Unmapped,
    Protection,
}
