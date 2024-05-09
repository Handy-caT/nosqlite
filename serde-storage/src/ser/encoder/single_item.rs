use crate::ser::{
    encoder::{storable::Storable, StorableInteger, StorageEncoder},
    error::Error,
};

/// SingleItemEncoder is a helper for encoding a single item.
#[derive(Debug)]
pub struct SingleItemEncoder<'e> {
    /// Encoder to use.
    pub encoder: &'e mut StorageEncoder,

    /// Whether a value has been written.
    pub value_written: &'e mut bool,
}

impl<'e> SingleItemEncoder<'e> {
    /// Emit a value.
    pub fn emit<T: Storable>(self, value: T) -> Result<(), Error> {
        value.encode(self)
    }

    /// Emit a value with a callback.
    pub fn emit_with<F>(self, value_cb: F) -> Result<(), Error>
    where
        F: FnOnce(SingleItemEncoder) -> Result<(), Error>,
    {
        value_cb(self)
    }

    /// Emit an integer.
    pub fn emit_int<T: StorableInteger>(self, value: T) -> Result<(), Error> {
        *self.value_written = true;
        self.encoder.emit_int(value)
    }

    /// Emit a boolean.
    pub fn emit_bool(self, value: bool) -> Result<(), Error> {
        *self.value_written = true;
        self.encoder.emit_bool(value)
    }

    /// Emit a string.
    pub fn emit_str(self, value: &str) -> Result<(), Error> {
        *self.value_written = true;
        self.encoder.emit_str(value)
    }

    /// Emit byte array.
    pub fn emit_bytes(self, value: &[u8]) -> Result<(), Error> {
        *self.value_written = true;
        self.encoder.emit_bytes(value)
    }

    /// Emit a f32.
    pub fn emit_f32(self, value: f32) -> Result<(), Error> {
        *self.value_written = true;
        self.encoder.emit_f32(value)
    }

    /// Emit a f64.
    pub fn emit_f64(self, value: f64) -> Result<(), Error> {
        *self.value_written = true;
        self.encoder.emit_f64(value)
    }

    pub fn emit_struct(
        self,
        values: Vec<Box<dyn Storable>>,
    ) -> Result<(), Error> {
        *self.value_written = true;
        self.encoder.emit_struct(values)
    }
}
