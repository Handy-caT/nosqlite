use serde_storage::{
    de::{
        self,
        decoder::{self, single_item::SingleItemDecoder},
    },
    descriptor::backwards::get_type_by_description_bytes,
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

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<Self, de::Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "u8" {
            return Err(de::Error::InvalidType);
        }

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

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<Self, de::Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "bool" {
            return Err(de::Error::InvalidType);
        }

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

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<Self, de::Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "i16" {
            return Err(de::Error::InvalidType);
        }

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

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<Self, de::Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "i32" {
            return Err(de::Error::InvalidType);
        }

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

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<Self, de::Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "i64" {
            return Err(de::Error::InvalidType);
        }

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

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<Self, de::Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "u16" {
            return Err(de::Error::InvalidType);
        }

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

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<Self, de::Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "u32" {
            return Err(de::Error::InvalidType);
        }

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

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<Self, de::Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "u64" {
            return Err(de::Error::InvalidType);
        }

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

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<Self, de::Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "f32" {
            return Err(de::Error::InvalidType);
        }

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

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<Self, de::Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "f64" {
            return Err(de::Error::InvalidType);
        }

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

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<Self, de::Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "varchar" {
            return Err(de::Error::InvalidType);
        }

        let str = decoder.emit_str(value)?;
        VarChar::try_from(str).map_err(|_| de::Error::InvalidLength)
    }
}
