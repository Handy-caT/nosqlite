use crate::{
    error::Error, ser::encoder::single_item::SingleItemEncoder,
};

pub trait Storable {
    // fn to_storable(&self) -> Vec<u8>;
    fn encode(&self, encoder: SingleItemEncoder) -> Result<(), Error>;
}

macro_rules! impl_integer {
    ($($t:ty),*) => {
        $(
            impl Storable for $t {
                fn encode(&self, encoder: SingleItemEncoder) -> Result<(), Error> {
                    encoder.emit_int(*self)
                }
            }
        )*
    }
}

impl_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl Storable for bool {
    fn encode(&self, encoder: SingleItemEncoder) -> Result<(), Error> {
        encoder.emit_bool(*self)
    }
}

impl Storable for String
{
    fn encode(&self, encoder: SingleItemEncoder) -> Result<(), Error> {
        encoder.emit_str(self)
    }
}

impl Storable for Vec<u8> {
    fn encode(&self, encoder: SingleItemEncoder) -> Result<(), Error> {
        encoder.emit_bytes(self)
    }
}
