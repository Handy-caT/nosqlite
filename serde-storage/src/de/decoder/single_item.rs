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

    /// Emit a bool.
    pub fn emit_bool(self, value: Vec<u8>) -> Result<bool, Error> {
        self.decoder.emit_bool(value)
    }

    /// Emit a i8.
    pub fn emit_i8(self, value: Vec<u8>) -> Result<i8, Error> {
        self.decoder.emit_i8(value)
    }

    /// Emit a i16.
    pub fn emit_i16(self, value: Vec<u8>) -> Result<i16, Error> {
        self.decoder.emit_i16(value)
    }

    /// Emit a i32.
    pub fn emit_i32(self, value: Vec<u8>) -> Result<i32, Error> {
        self.decoder.emit_i32(value)
    }

    /// Emit a i64.
    pub fn emit_i64(self, value: Vec<u8>) -> Result<i64, Error> {
        self.decoder.emit_i64(value)
    }

    /// Emit a i128.
    pub fn emit_i128(self, value: Vec<u8>) -> Result<i128, Error> {
        self.decoder.emit_i128(value)
    }

    /// Emit a string.
    pub fn emit_str(self, value: Vec<u8>) -> Result<String, Error> {
        self.decoder.emit_str(value)
    }

    /// Emit a f32.
    pub fn emit_f32(self, value: Vec<u8>) -> Result<f32, Error> {
        self.decoder.emit_f32(value)
    }

    /// Emit a f64.
    pub fn emit_f64(self, value: Vec<u8>) -> Result<f64, Error> {
        self.decoder.emit_f64(value)
    }

    /// Emit a bytes.
    pub fn emit_bytes(self, value: &[u8]) -> Result<Vec<u8>, Error> {
        self.decoder.emit_bytes(value)
    }
}
