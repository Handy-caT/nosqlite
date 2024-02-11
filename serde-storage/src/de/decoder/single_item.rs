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

    // pub fn emit_u8(self, value: Vec<u8>) -> Result<u8, Error> {
    //     u8::decode(self, value)
    // }
}
