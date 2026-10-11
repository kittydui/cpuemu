#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapError {
    Misaligned,
    ZeroSize,
    Overflow,
    Overlap,
    NotMapped,
}
