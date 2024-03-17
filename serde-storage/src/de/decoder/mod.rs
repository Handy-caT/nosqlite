pub mod single_item;
pub mod storable;

use crate::de::{decoder::single_item::SingleItemDecoder, error::Error};
use std::mem::size_of;

pub use storable::Storable;

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

    /// Emit a u8.
    pub fn emit_u8(&mut self, value: Vec<u8>) -> Result<u8, Error> {
        if value.len() != size_of::<u8>() {
            return Err(Error::InvalidLength);
        }

        Ok(value[0])
    }

    /// Emit a u16.
    pub fn emit_u16(&mut self, value: Vec<u8>) -> Result<u16, Error> {
        if value.len() != size_of::<u16>() {
            return Err(Error::InvalidLength);
        }

        let bytes = value.try_into().unwrap();
        Ok(u16::from_be_bytes(bytes))
    }

    /// Emit a u32.
    pub fn emit_u32(&mut self, value: Vec<u8>) -> Result<u32, Error> {
        if value.len() != size_of::<u32>() {
            return Err(Error::InvalidLength);
        }

        let bytes = value.try_into().unwrap();
        Ok(u32::from_be_bytes(bytes))
    }

    /// Emit a u64.
    pub fn emit_u64(&mut self, value: Vec<u8>) -> Result<u64, Error> {
        if value.len() != size_of::<u64>() {
            return Err(Error::InvalidLength);
        }

        let bytes = value.try_into().unwrap();
        Ok(u64::from_be_bytes(bytes))
    }

    /// Emit a u128.
    pub fn emit_u128(&mut self, value: Vec<u8>) -> Result<u128, Error> {
        if value.len() != size_of::<u128>() {
            return Err(Error::InvalidLength);
        }

        let bytes = value.try_into().unwrap();
        Ok(u128::from_be_bytes(bytes))
    }

    /// Emit a `bool`.
    pub fn emit_bool(&mut self, value: Vec<u8>) -> Result<bool, Error> {
        if value.len() != size_of::<bool>() {
            return Err(Error::InvalidLength);
        }

        match value[0] {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::InvalidValue),
        }
    }

    /// Emit a i8.
    pub fn emit_i8(&mut self, value: Vec<u8>) -> Result<i8, Error> {
        if value.len() != size_of::<i8>() {
            return Err(Error::InvalidLength);
        }

        Ok(value[0] as i8)
    }

    /// Emit a i16.
    pub fn emit_i16(&mut self, value: Vec<u8>) -> Result<i16, Error> {
        if value.len() != size_of::<i16>() {
            return Err(Error::InvalidLength);
        }

        let bytes = value.try_into().unwrap();
        Ok(i16::from_be_bytes(bytes))
    }

    /// Emit a i32.
    pub fn emit_i32(&mut self, value: Vec<u8>) -> Result<i32, Error> {
        if value.len() != size_of::<i32>() {
            return Err(Error::InvalidLength);
        }

        let bytes = value.try_into().unwrap();
        Ok(i32::from_be_bytes(bytes))
    }

    /// Emit a i64.
    pub fn emit_i64(&mut self, value: Vec<u8>) -> Result<i64, Error> {
        if value.len() != size_of::<i64>() {
            return Err(Error::InvalidLength);
        }

        let bytes = value.try_into().unwrap();
        Ok(i64::from_be_bytes(bytes))
    }

    /// Emit a i128.
    pub fn emit_i128(&mut self, value: Vec<u8>) -> Result<i128, Error> {
        if value.len() != size_of::<i128>() {
            return Err(Error::InvalidLength);
        }

        let bytes = value.try_into().unwrap();
        Ok(i128::from_be_bytes(bytes))
    }

    /// Emit a `String`.
    pub fn emit_str(&mut self, value: Vec<u8>) -> Result<String, Error> {
        let string =
            String::from_utf8(value).map_err(|_| Error::InvalidUtf8)?;
        Ok(string)
    }

    /// Emit a f32.
    pub fn emit_f32(&mut self, value: Vec<u8>) -> Result<f32, Error> {
        if value.len() != size_of::<f32>() {
            return Err(Error::InvalidLength);
        }

        let bytes = value.try_into().unwrap();
        Ok(f32::from_be_bytes(bytes))
    }

    /// Emit a f64.
    pub fn emit_f64(&mut self, value: Vec<u8>) -> Result<f64, Error> {
        if value.len() != size_of::<f64>() {
            return Err(Error::InvalidLength);
        }

        let bytes = value.try_into().unwrap();
        Ok(f64::from_be_bytes(bytes))
    }

    /// Emit a `&[u8]`.
    pub fn emit_bytes(&mut self, value: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(value.to_vec())
    }
}
