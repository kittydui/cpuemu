#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
#[repr(transparent)]
pub struct RegisterId(pub u16);

pub struct Register {
    pub id: RegisterId,
    pub name: &'static str,
    pub size: usize,
}
