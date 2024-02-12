use crate::de::{
    decoder::{storable::Storable, StorageDecoder},
    error::Error,
};

/// SingleItemDecoder is a helper for decoding a single item.
pub struct SingleItemDecoder<'d> {
    /// Decoder to use.
    pub decoder: &'d mut StorageDecoder,
}

impl<'d> SingleItemDecoder<'d> {
    /// Emit a value.
    pub fn emit<T: Storable<T>>(self, value: Vec<u8>) -> Result<T, Error> {
        T::decode(self, value)
    }

    /// Emit u8.
    pub fn emit_u8(self, value: Vec<u8>) -> Result<u8, Error> {
        self.decoder.emit_u8(value)
    }

    /// Emit u16.
    pub fn emit_u16(self, value: Vec<u8>) -> Result<u16, Error> {
        self.decoder.emit_u16(value)
    }

    /// Emit u32.
    pub fn emit_u32(self, value: Vec<u8>) -> Result<u32, Error> {
        self.decoder.emit_u32(value)
    }

    /// Emit u64.
    pub fn emit_u64(self, value: Vec<u8>) -> Result<u64, Error> {
        self.decoder.emit_u64(value)
    }

    /// Emit u128.
    pub fn emit_u128(self, value: Vec<u8>) -> Result<u128, Error> {
        self.decoder.emit_u128(value)
    }
}
