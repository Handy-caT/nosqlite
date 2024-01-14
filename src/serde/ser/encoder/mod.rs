mod single_item;
mod storable;
pub mod storable_integer;

use smart_default::SmartDefault;
pub use storable_integer::StorableInteger;
use crate::serde::error::Error;
use crate::serde::ser::descriptor::{Description, Descriptor as _};
use crate::serde::ser::descriptor::integer::{IntegerDescriptor};

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

    pub fn emit_int<T: StorableInteger>(&mut self, value: T) -> Result<(), Error>{
        self.output.append(value.get_storable());

        // Unwrap is safe because the value is always a valid integer.
        let description = IntegerDescriptor::describe(value).unwrap();

        self.descriptor.append(description);

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
}
