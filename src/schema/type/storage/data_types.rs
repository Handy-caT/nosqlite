use serde_storage::{
    ser::error::Error,
    ser::encoder::{single_item::SingleItemEncoder, storable::Storable},
};

use crate::schema::r#type::data_types::*;

#[rustfmt::skip]
macro_rules! impl_storable {
    ($($t:ty),*) => {
        $(
            impl Storable for $t {
                fn encode(&self, encoder: SingleItemEncoder) -> Result<(), Error> {
                    encoder.emit(*self)
                }
            }
        )*
    }
}

impl_storable!(
    Byte, Bool, Short, Integer, Long, UShort, UInteger, ULong, Float, Double
);

impl<const N: u16> Storable for VarChar<N> {
    fn encode(&self, encoder: SingleItemEncoder) -> Result<(), Error> {
        encoder.emit_str(&self.value)
    }
}
