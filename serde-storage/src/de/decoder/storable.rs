use crate::de::{decoder::single_item::SingleItemDecoder, error::Error};

pub trait Storable<T> {
    fn decode(decoder: SingleItemDecoder, value: Vec<u8>) -> Result<T, Error>;
}

impl Storable<u8> for u8 {
    fn decode(decoder: SingleItemDecoder, value: Vec<u8>) -> Result<u8, Error> {
        decoder.emit_u8(value)
    }
}

impl Storable<u16> for u16 {
    fn decode(decoder: SingleItemDecoder, value: Vec<u8>) -> Result<u16, Error> {
        decoder.emit_u16(value)
    }
}

impl Storable<u32> for u32 {
    fn decode(decoder: SingleItemDecoder, value: Vec<u8>) -> Result<u32, Error> {
        decoder.emit_u32(value)
    }
}

impl Storable<u64> for u64 {
    fn decode(decoder: SingleItemDecoder, value: Vec<u8>) -> Result<u64, Error> {
        decoder.emit_u64(value)
    }
}

impl Storable<u128> for u128 {
    fn decode(decoder: SingleItemDecoder, value: Vec<u8>) -> Result<u128, Error> {
        decoder.emit_u128(value)
    }
}