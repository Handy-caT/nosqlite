use crate::de::{
    decoder::{single_item::SingleItemDecoder, storable::Storable},
    error::Error,
};

pub mod single_item;
pub mod storable;

/// StorageDecoder is a helper for decoding items.
#[derive(Default, Debug)]
pub struct StorageDecoder;

impl StorageDecoder {
    /// Emit a value.
    pub fn emit<T: Storable<T>>(&mut self, value: Vec<u8>) -> Result<T, Error> {
        self.emit_with(|decoder| T::decode(decoder, value))
    }

    /// Emit a value with a callback.
    pub fn emit_with<F, T>(&mut self, value_cb: F) -> Result<T, Error>
    where
        F: FnOnce(SingleItemDecoder) -> Result<T, Error>,
    {
        let decoder = SingleItemDecoder { decoder: self };

        value_cb(decoder)
    }

    /// Emit u8.
    pub fn emit_u8(&mut self, value: Vec<u8>) -> Result<u8, Error> {
        if value.len() != 1 {
            return Err(Error::InvalidLength);
        }

        Ok(value[0])
    }

    /// Emit u16.
    pub fn emit_u16(&mut self, value: Vec<u8>) -> Result<u16, Error> {
        if value.len() != 2 {
            return Err(Error::InvalidLength);
        }

        let bytes = value.try_into().unwrap();
        Ok(u16::from_be_bytes(bytes))
    }

    /// Emit u32.
    pub fn emit_u32(&mut self, value: Vec<u8>) -> Result<u32, Error> {
        if value.len() != 4 {
            return Err(Error::InvalidLength);
        }

        let bytes = value.try_into().unwrap();
        Ok(u32::from_be_bytes(bytes))
    }

    /// Emit u64.
    pub fn emit_u64(&mut self, value: Vec<u8>) -> Result<u64, Error> {
        if value.len() != 8 {
            return Err(Error::InvalidLength);
        }

        let bytes = value.try_into().unwrap();
        Ok(u64::from_be_bytes(bytes))
    }

    /// Emit u128.
    pub fn emit_u128(&mut self, value: Vec<u8>) -> Result<u128, Error> {
        if value.len() != 16 {
            return Err(Error::InvalidLength);
        }

        let bytes = value.try_into().unwrap();
        Ok(u128::from_be_bytes(bytes))
    }
}
