use serde_storage::{
    de::{
        self,
        decoder::{self, single_item::SingleItemDecoder},
    },
    ser::{
        self,
        encoder::{self, single_item::SingleItemEncoder},
    },
};

use crate::schema::r#type::data_types::*;

#[rustfmt::skip]
macro_rules! impl_storable {
    ($($t:ty),*) => {
        $(
            impl encoder::Storable for $t {
                fn encode(&self, encoder: SingleItemEncoder) -> Result<(), ser::Error> {
                    encoder.emit(self.0)
                }
            }
        )*
    }
}

impl_storable!(
    Byte, Bool, Short, Integer, Long, UShort, UInteger, ULong, Float, Double
);

impl<const N: u16> encoder::Storable for VarChar<N> {
    fn encode(&self, encoder: SingleItemEncoder) -> Result<(), ser::Error> {
        encoder.emit_str(&self.value)
    }
}

impl decoder::Storable<Self> for Byte {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<Self, de::Error> {
        decoder.emit_u8(value).map(Byte)
    }
}
