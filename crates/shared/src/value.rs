use crate::Endian;

// A fixed-size integer that can be read from or written to bytes in either byte order.
// Lets memory and register access be generic, e.g. `read_value::<u32>(..)`.
pub trait Value: Copy {
    const SIZE: usize;

    // `bytes` must be exactly `SIZE` long.
    fn from_bytes(bytes: &[u8], endian: Endian) -> Self;
    fn to_bytes(self, endian: Endian, out: &mut [u8]);
}

macro_rules! impl_value {
    ($($ty:ty),*) => {$(
        impl Value for $ty {
            const SIZE: usize = size_of::<$ty>();

            fn from_bytes(bytes: &[u8], endian: Endian) -> Self {
                let bytes = bytes.try_into().expect("byte count must match the value size");
                match endian {
                    Endian::Little => <$ty>::from_le_bytes(bytes),
                    Endian::Big => <$ty>::from_be_bytes(bytes),
                }
            }

            fn to_bytes(self, endian: Endian, out: &mut [u8]) {
                let bytes = match endian {
                    Endian::Little => self.to_le_bytes(),
                    Endian::Big => self.to_be_bytes(),
                };
                out.copy_from_slice(&bytes);
            }
        }
    )*};
}

impl_value!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn little_endian_round_trip() {
        let mut bytes = [0; 4];
        0x1122_3344u32.to_bytes(Endian::Little, &mut bytes);
        assert_eq!(bytes, [0x44, 0x33, 0x22, 0x11]);
        assert_eq!(u32::from_bytes(&bytes, Endian::Little), 0x1122_3344);
    }

    #[test]
    fn big_endian_round_trip() {
        let mut bytes = [0; 4];
        0x1122_3344u32.to_bytes(Endian::Big, &mut bytes);
        assert_eq!(bytes, [0x11, 0x22, 0x33, 0x44]);
        assert_eq!(u32::from_bytes(&bytes, Endian::Big), 0x1122_3344);
    }

    #[test]
    fn signed() {
        assert_eq!(i16::from_bytes(&[0xFF, 0xFF], Endian::Little), -1);
    }
}
