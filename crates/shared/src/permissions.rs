use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct Permissions: u8 {
        const READ = 1;
        const WRITE = 2;
        const EXECUTE = 4;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Access {
    Read,
    Write,
    Fetch,
}

impl Access {
    pub const fn required(self) -> Permissions {
        match self {
            Access::Read => Permissions::READ,
            Access::Write => Permissions::WRITE,
            Access::Fetch => Permissions::EXECUTE,
        }
    }
}
