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

    pub fn emit_u8(&mut self, value: Vec<u8>) -> Result<u8, Error> {
        if value.len() != 1 {
            return Err(Error::InvalidLength);
        }

        Ok(value[0])
    }

    pub fn emit_u16(&mut self, value: Vec<u8>) -> Result<u16, Error> {
        if value.len() != 2 {
            return Err(Error::InvalidLength);
        }

        Ok(u16::from_be_bytes([value[0], value[1]]))
    }
}
