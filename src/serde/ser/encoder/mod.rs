mod single_item;
mod storable;
pub mod storable_integer;

use crate::serde::{
    error::Error,
    ser::descriptor::{
        integer::IntegerDescriptor,
        r#type::{BoolDescription, BytesDescription, StringDescription},
        Description, Descriptor as _,
    },
};
use smart_default::SmartDefault;
pub use storable_integer::StorableInteger;

/// Output bytes after encoding.
#[derive(Default, Debug, Clone)]
pub struct OutputBytes(Vec<u8>);

impl OutputBytes {
    /// Create a new [`OutputBytes`].
    pub fn new() -> Self {
        <Self as Default>::default()
    }

    /// Get the bytes from the [`OutputBytes`].
    pub fn get_bytes(self) -> Vec<u8> {
        self.0
    }

    /// Append bytes to the [`OutputBytes`].
    pub fn append(&mut self, bytes: Vec<u8>) {
        self.0.append(&mut bytes.clone());
    }
}

/// Descriptor bytes after encoding.
#[derive(SmartDefault, Debug, Clone)]
pub struct OutputDescriptor {
    /// Bytes of the description.
    bytes: Vec<u8>,

    /// String representation of desciption.
    #[default("|".to_string())]
    name: String,
}

impl OutputDescriptor {
    /// Create a new [`OutputDescriptor`].
    pub fn new() -> Self {
        <Self as Default>::default()
    }

    /// Get the bytes from the [`OutputDescriptor`].
    pub fn get_bytes(self) -> Vec<u8> {
        self.bytes.clone()
    }

    /// Get the name of the type.
    pub fn get_name(self) -> String {
        self.name.clone()
    }

    /// Append a description to the [`OutputDescriptor`].
    pub fn append<D: Description>(&mut self, description: D) {
        self.bytes.append(&mut description.get_bytes().clone());

        self.name.push_str(&description.get_name());
        self.name.push('|');
    }
}

#[derive(Default, Debug)]
pub struct StorageEncoder {
    output: OutputBytes,
    descriptor: OutputDescriptor,
}

impl StorageEncoder {
    /// Create a new [`StorageEncoder`].
    pub fn new() -> Self {
        <Self as Default>::default()
    }

    /// Encode an [`StorableInteger`] and append it to the output.
    pub fn emit_int<T: StorableInteger>(
        &mut self,
        value: T,
    ) -> Result<(), Error> {
        self.output.append(value.get_storable());

        // Unwrap is safe because the value is always a valid integer.
        let description = IntegerDescriptor::describe(value).unwrap();

        self.descriptor.append(description);

        Ok(())
    }

    /// Encode a [`String`] and append it to the output.
    pub fn emit_str<S: AsRef<str>>(&mut self, value: S) -> Result<(), Error> {
        let bytes = value.as_ref().as_bytes().to_vec();
        self.output.append(bytes);

        let description = StringDescription::new(value.as_ref().len());
        self.descriptor.append(description);

        Ok(())
    }

    /// Encode a [`bool`] and append it to the output.
    pub fn emit_bool(&mut self, value: bool) -> Result<(), Error> {
        let bytes = if value { vec![1] } else { vec![0] };

        self.output.append(bytes);
        self.descriptor.append(BoolDescription::new());

        Ok(())
    }

    /// Encode a [`Vec<u8>`] and append it to the output.
    pub fn emit_bytes(&mut self, value: Vec<u8>) -> Result<(), Error> {
        self.descriptor
            .append(BytesDescription::new(value.as_slice()));
        self.output.append(value);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storable_integer() {
        let value: u32 = 1;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_int(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![0, 0, 0, 1]);

        let descriptor = encoder.descriptor.get_name();
        assert_eq!(descriptor, "|u32|");
    }

    #[test]
    fn test_string() {
        let value = "Hello, world!";

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_str(value);
        assert!(res.is_ok());

        let descriptor = encoder.descriptor.get_name();
        assert_eq!(descriptor, "|str13|");
    }

    #[test]
    fn test_bool() {
        let value = true;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_bool(value);
        assert!(res.is_ok());

        let descriptor = encoder.descriptor.get_name();
        assert_eq!(descriptor, "|bool|");
    }

    #[test]
    fn test_bytes() {
        let value = vec![1, 2, 3, 4];

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_bytes(value);
        assert!(res.is_ok());

        let descriptor = encoder.descriptor.get_name();
        assert_eq!(descriptor, "|byte4|");
    }
}
