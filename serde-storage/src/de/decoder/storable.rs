use crate::{
    de::{decoder::single_item::SingleItemDecoder, error::Error},
    descriptor::backwards::get_type_by_description_bytes,
};

pub trait Storable<T> {
    fn decode(decoder: SingleItemDecoder, value: Vec<u8>) -> Result<T, Error>;

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<T, Error>;
}

impl Storable<u8> for u8 {
    fn decode(decoder: SingleItemDecoder, value: Vec<u8>) -> Result<u8, Error> {
        decoder.emit_u8(value)
    }

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<u8, Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "u8" {
            return Err(Error::InvalidType);
        }

        decoder.emit_u8(value)
    }
}

impl Storable<u16> for u16 {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<u16, Error> {
        decoder.emit_u16(value)
    }

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<u16, Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "u16" {
            return Err(Error::InvalidType);
        }

        decoder.emit_u16(value)
    }
}

impl Storable<u32> for u32 {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<u32, Error> {
        decoder.emit_u32(value)
    }

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<u32, Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "u32" {
            return Err(Error::InvalidType);
        }

        decoder.emit_u32(value)
    }
}

impl Storable<u64> for u64 {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<u64, Error> {
        decoder.emit_u64(value)
    }

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<u64, Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "u64" {
            return Err(Error::InvalidType);
        }

        decoder.emit_u64(value)
    }
}

impl Storable<u128> for u128 {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<u128, Error> {
        decoder.emit_u128(value)
    }

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<u128, Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "u128" {
            return Err(Error::InvalidType);
        }

        decoder.emit_u128(value)
    }
}

impl Storable<bool> for bool {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<bool, Error> {
        decoder.emit_bool(value)
    }

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<bool, Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "bool" {
            return Err(Error::InvalidType);
        }

        decoder.emit_bool(value)
    }
}

impl Storable<i8> for i8 {
    fn decode(decoder: SingleItemDecoder, value: Vec<u8>) -> Result<i8, Error> {
        decoder.emit_i8(value)
    }

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<i8, Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "i8" {
            return Err(Error::InvalidType);
        }

        decoder.emit_i8(value)
    }
}

impl Storable<i16> for i16 {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<i16, Error> {
        decoder.emit_i16(value)
    }

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<i16, Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "i16" {
            return Err(Error::InvalidType);
        }

        decoder.emit_i16(value)
    }
}

impl Storable<i32> for i32 {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<i32, Error> {
        decoder.emit_i32(value)
    }

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<i32, Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "i32" {
            return Err(Error::InvalidType);
        }

        decoder.emit_i32(value)
    }
}

impl Storable<i64> for i64 {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<i64, Error> {
        decoder.emit_i64(value)
    }

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<i64, Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "i64" {
            return Err(Error::InvalidType);
        }

        decoder.emit_i64(value)
    }
}

impl Storable<i128> for i128 {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<i128, Error> {
        decoder.emit_i128(value)
    }

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<i128, Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "i128" {
            return Err(Error::InvalidType);
        }

        decoder.emit_i128(value)
    }
}

impl Storable<String> for String {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<String, Error> {
        decoder.emit_str(value)
    }

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<String, Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "String" {
            return Err(Error::InvalidType);
        }

        decoder.emit_str(value)
    }
}

impl Storable<f32> for f32 {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<f32, Error> {
        decoder.emit_f32(value)
    }

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<f32, Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "f32" {
            return Err(Error::InvalidType);
        }

        decoder.emit_f32(value)
    }
}

impl Storable<f64> for f64 {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<f64, Error> {
        decoder.emit_f64(value)
    }

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<f64, Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "f64" {
            return Err(Error::InvalidType);
        }

        decoder.emit_f64(value)
    }
}

impl Storable<Vec<u8>> for Vec<u8> {
    fn decode(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
    ) -> Result<Vec<u8>, Error> {
        decoder.emit_bytes(value.as_slice())
    }

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<Vec<u8>, Error> {
        if get_type_by_description_bytes(descriptor.as_slice()) != "Vec<u8>" {
            return Err(Error::InvalidType);
        }

        decoder.emit_bytes(value.as_slice())
    }
}
