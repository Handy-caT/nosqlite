use crate::{
    error::Error,
    ser::encoder::{storable::Storable, StorableInteger, StorageEncoder},
};

pub struct SingleItemEncoder<'a> {
    pub encoder: &'a mut StorageEncoder,

    pub value_written: &'a mut bool,
}

impl<'a> SingleItemEncoder<'a> {
    pub fn emit<T: Storable>(self, value: T) -> Result<(), Error> {
        value.encode(self)
    }

    pub fn emit_with<F>(self, value_cb: F) -> Result<(), Error>
    where
        F: FnOnce(SingleItemEncoder) -> Result<(), Error>,
    {
        value_cb(self)
    }

    pub fn emit_int<T: StorableInteger>(self, value: T) -> Result<(), Error> {
        *self.value_written = true;
        self.encoder.emit_int(value)
    }

    pub fn emit_bool(self, value: bool) -> Result<(), Error> {
        *self.value_written = true;
        self.encoder.emit_bool(value)
    }

    pub fn emit_str(self, value: &str) -> Result<(), Error> {
        *self.value_written = true;
        self.encoder.emit_str(value)
    }

    pub fn emit_bytes(self, value: &[u8]) -> Result<(), Error> {
        *self.value_written = true;
        self.encoder.emit_bytes(value)
    }
}
