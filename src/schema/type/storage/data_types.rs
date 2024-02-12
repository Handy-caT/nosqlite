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

impl decoder::Storable<Self> for Bool {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<Self, de::Error> {
        decoder.emit_bool(value).map(Bool)
    }
}

impl decoder::Storable<Self> for Short {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<Self, de::Error> {
        decoder.emit_i16(value).map(Short)
    }
}

impl decoder::Storable<Self> for Integer {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<Self, de::Error> {
        decoder.emit_i32(value).map(Integer)
    }
}

impl decoder::Storable<Self> for Long {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<Self, de::Error> {
        decoder.emit_i128(value).map(Long)
    }
}

impl decoder::Storable<Self> for UShort {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<Self, de::Error> {
        decoder.emit_u16(value).map(UShort)
    }
}

impl decoder::Storable<Self> for UInteger {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<Self, de::Error> {
        decoder.emit_u32(value).map(UInteger)
    }
}

impl decoder::Storable<Self> for ULong {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<Self, de::Error> {
        decoder.emit_u128(value).map(ULong)
    }
}

impl decoder::Storable<Self> for Float {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<Self, de::Error> {
        decoder.emit_f32(value).map(Float)
    }
}

impl decoder::Storable<Self> for Double {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<Self, de::Error> {
        decoder.emit_f64(value).map(Double)
    }
}

impl<const N: u16> decoder::Storable<Self> for VarChar<N> {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<Self, de::Error> {
        let str = decoder.emit_str(value)?;
        VarChar::try_from(str).map_err(|_| de::Error::InvalidLength)
    }
}