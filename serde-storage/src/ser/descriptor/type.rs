use crate::{book_values, ser::descriptor::Description};

/// Descriptor for a [`String`].
pub struct StringDescription {
    /// Bytes of the description.
    bytes: Vec<u8>,

    /// Type name of the integer.
    name: String,
}

const STRING_NUMBER: u8 = 6;
book_values!(STRING_NUMBER);

impl StringDescription {
    /// Create a new [`StringDescription`].
    pub fn new(len: usize) -> Self {
        let name = format!("str{}", len);
        Self {
            bytes: vec![STRING_NUMBER],
            name,
        }
    }
}

impl Description for StringDescription {
    fn get_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub struct BoolDescription {
    /// Bytes of the description.
    bytes: Vec<u8>,

    /// Type name of the integer.
    name: String,
}

const BOOL_NUMBER: u8 = 0;
book_values!(BOOL_NUMBER);

impl BoolDescription {
    /// Create a new [`BoolDescription`].
    pub fn new() -> Self {
        let name = "bool".to_string();
        Self {
            bytes: vec![BOOL_NUMBER],
            name,
        }
    }
}

impl Description for BoolDescription {
    fn get_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub struct BytesDescription {
    /// Bytes of the description.
    bytes: Vec<u8>,

    /// Type name of the integer.
    name: String,
}

const BYTES_NUMBER: u8 = 7;
book_values!(BYTES_NUMBER);

impl BytesDescription {
    /// Create a new [`BytesDescription`].
    pub fn new(bytes: &[u8]) -> Self {
        let name = format!("byte{}", bytes.len());
        Self {
            bytes: vec![BYTES_NUMBER],
            name,
        }
    }
}

impl Description for BytesDescription {
    fn get_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}
