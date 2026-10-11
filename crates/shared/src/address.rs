#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
#[repr(transparent)]
pub struct Address(pub u64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_size_as_u64() {
        assert_eq!(std::mem::size_of::<Address>(), std::mem::size_of::<u64>());
    }

    #[test]
    fn default_is_zero() {
        assert_eq!(Address::default(), Address(0));
    }

    #[test]
    fn orders_by_value() {
        assert!(Address(0x1000) < Address(0x2000));
        assert!(Address(0) < Address(u64::MAX));
    }
}
