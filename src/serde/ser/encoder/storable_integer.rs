/// An integer value, that can be interpreted as a byte array.
pub trait StorableInteger {
    fn get_storable(&self) -> Vec<u8>;
}

macro_rules! impl_integer {
    ($($t:ty),*) => {
        $(
            impl StorableInteger for $t {
                fn get_storable(&self) -> Vec<u8> {
                    self.to_be_bytes().to_vec()
                }
            }
        )*
    }
}

impl_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::base::cast::usize::USIZE_SIZE;

    #[test]
    fn test_storable_integer() {
        let u8_val: u8 = 1;
        let u16_val: u16 = 1;
        let u32_val: u32 = 1;
        let u64_val: u64 = 1;
        let u128_val: u128 = 1;
        let usize_val: usize = 0;
        let i8_val: i8 = 1;
        let i16_val: i16 = 1;
        let i32_val: i32 = 1;
        let i64_val: i64 = 1;
        let i128_val: i128 = 1;
        let isize_val: isize = 0;

        assert_eq!(u8_val.get_storable(), vec![1]);
        assert_eq!(u16_val.get_storable(), vec![0, 1]);
        assert_eq!(u32_val.get_storable(), vec![0, 0, 0, 1]);
        assert_eq!(u64_val.get_storable(), vec![0, 0, 0, 0, 0, 0, 0, 1]);
        assert_eq!(
            u128_val.get_storable(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        );

        assert_eq!(usize_val.get_storable(), vec![0; USIZE_SIZE]);

        assert_eq!(i8_val.get_storable(), vec![1]);
        assert_eq!(i16_val.get_storable(), vec![0, 1]);
        assert_eq!(i32_val.get_storable(), vec![0, 0, 0, 1]);
        assert_eq!(i64_val.get_storable(), vec![0, 0, 0, 0, 0, 0, 0, 1]);
        assert_eq!(
            i128_val.get_storable(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        );

        assert_eq!(isize_val.get_storable(), vec![0; USIZE_SIZE]);
    }
}
