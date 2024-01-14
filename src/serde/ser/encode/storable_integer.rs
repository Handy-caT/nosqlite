
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