pub mod single_item;
pub mod storable;
pub mod storable_integer;

use crate::{
    descriptor::{
        integer::IntegerDescriptor, r#type::BoolDescription, Description,
        Descriptor as _,
    },
    ser::{encoder::single_item::SingleItemEncoder, error::Error},
};

use smart_default::SmartDefault;

use crate::descriptor::{
    array::ArrayDescription,
    integer::IntegerDescription,
    r#type::{CharDescription, F32Description, F64Description},
};
pub use storable::Storable;
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
    /// List of descriptor of encoded values.
    descriptors: Vec<(Vec<u8>, String)>,
}

impl OutputDescriptor {
    /// Create a new [`OutputDescriptor`].
    pub fn new() -> Self {
        <Self as Default>::default()
    }

    /// Get the descriptors from the [`OutputDescriptor`].
    pub fn get_descriptors<'a>(&self) -> Vec<(Vec<u8>, String)> {
        self.descriptors.clone()
    }
    
    /// Get the descriptor bytes from the [`OutputDescriptor`].
    pub fn get_descriptor_bytes(&self) -> Vec<u8> {
        self.descriptors
            .iter()
            .flat_map(|(bytes, _)| bytes.clone())
            .collect()
    }

    /// Append a description to the [`OutputDescriptor`].
    pub fn append<D: Description>(&mut self, description: D) {
        self.descriptors
            .push((description.get_bytes(), description.get_name()));
    }
}

/// StorageEncoder is a helper for encoding items.
#[derive(Default, Debug)]
pub struct StorageEncoder {
    /// [`OutputBytes`] is the output after encoding.
    pub output: OutputBytes,
    /// [`OutputDescriptor`] is the descriptor.
    pub descriptor: OutputDescriptor,
}

impl StorageEncoder {
    /// Create a new [`StorageEncoder`].
    pub fn new() -> Self {
        <Self as Default>::default()
    }

    pub fn emit<T: Storable>(&mut self, value: T) -> Result<(), Error> {
        self.emit_with(|encoder| value.encode(encoder))
    }

    pub fn emit_with<F>(&mut self, value_cb: F) -> Result<(), Error>
    where
        F: FnOnce(SingleItemEncoder) -> Result<(), Error>,
    {
        let mut written = false;
        let encoder = SingleItemEncoder {
            encoder: self,
            value_written: &mut written,
        };

        value_cb(encoder)?;

        if !written {
            return Err(Error::NoValueWritten);
        }

        Ok(())
    }

    /// Encode an [`StorableInteger`] and append it to the output.
    pub fn emit_int<T: StorableInteger>(
        &mut self,
        value: T,
    ) -> Result<(), Error> {
        self.output.append(value.get_storable());

        // Unwrap is safe because the value is always a valid integer.
        let description = IntegerDescriptor::<T>::describe();

        self.descriptor.append(description);

        Ok(())
    }

    /// Encode a [`String`] and append it to the output.
    pub fn emit_str<S: AsRef<str>>(&mut self, value: S) -> Result<(), Error> {
        let bytes = value.as_ref().as_bytes().to_vec();
        self.output.append(bytes);

        let description = ArrayDescription::<char, CharDescription>::new(
            value.as_ref().len() as u32,
        );
        self.descriptor.append(description);

        Ok(())
    }

    /// Encode a `bool` and append it to the output.
    pub fn emit_bool(&mut self, value: bool) -> Result<(), Error> {
        let bytes = if value { vec![1] } else { vec![0] };

        self.output.append(bytes);
        self.descriptor.append(BoolDescription::default());

        Ok(())
    }

    /// Encode a `&[u8]` and append it to the output.
    pub fn emit_bytes(&mut self, value: &[u8]) -> Result<(), Error> {
        let description =
            ArrayDescription::<u8, IntegerDescription>::new(value.len() as u32);

        self.descriptor.append(description);
        self.output.append(value.to_vec());

        Ok(())
    }

    /// Encode a `f32` and append it to the output.
    pub fn emit_f32(&mut self, value: f32) -> Result<(), Error> {
        self.descriptor.append(F32Description::default());
        self.output.append(value.to_be_bytes().to_vec());

        Ok(())
    }

    /// Encode a `f64` and append it to the output.
    pub fn emit_f64(&mut self, value: f64) -> Result<(), Error> {
        self.descriptor.append(F64Description::default());
        self.output.append(value.to_be_bytes().to_vec());

        Ok(())
    }
}

#[cfg(test)]
mod tests_storage_encoder {
    use super::*;

    #[test]
    fn test_storable_integer() {
        let value: u32 = 1;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_int(value);
        assert!(res.is_ok());

        let bytes = encoder.output.get_bytes();
        assert_eq!(bytes, vec![0, 0, 0, 1]);

        let descriptor = encoder.descriptor.get_descriptors();

        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "u32");
    }

    #[test]
    fn test_string() {
        let value = "Hello, world!";

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_str(value);
        assert!(res.is_ok());

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "array_char_13");
    }

    #[test]
    fn test_bool() {
        let value = true;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_bool(value);
        assert!(res.is_ok());

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "bool");
    }

    #[test]
    fn test_bytes() {
        let value = vec![1, 2, 3, 4];

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_bytes(value.as_slice());
        assert!(res.is_ok());

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "array_u8_4");
    }

    #[test]
    fn test_f32() {
        let value = 1.1231234;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_f32(value);
        assert!(res.is_ok());

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "f32");
    }

    #[test]
    fn test_f64() {
        let value = 1.8967892514;

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_f64(value);
        assert!(res.is_ok());

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "f64");
    }
}
