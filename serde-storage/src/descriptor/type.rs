use crate::{book_values, descriptor::Description};

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

/// Descriptor for a bool.
pub struct BoolDescription {
    /// Bytes of the description.
    bytes: Vec<u8>,

    /// Type name of the integer.
    name: String,
}

const BOOL_NUMBER: u8 = 0;
book_values!(BOOL_NUMBER);

impl Default for BoolDescription {
    fn default() -> Self {
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

/// Descriptor for a u8 array.
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

/// Descriptor for a f32.
pub struct F32Description {
    /// Bytes of the description.
    bytes: Vec<u8>,

    /// Type name of the integer.
    name: String,
}

const F32_NUMBER: u8 = 8;
book_values!(F32_NUMBER);

impl Default for F32Description {
    fn default() -> Self {
        let name = "f32".to_string();
        Self {
            bytes: vec![F32_NUMBER],
            name,
        }
    }
}

impl Description for F32Description {
    fn get_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

/// Descriptor for a f64.
pub struct F64Description {
    /// Bytes of the description.
    bytes: Vec<u8>,

    /// Type name of the integer.
    name: String,
}

const F64_NUMBER: u8 = 9;
book_values!(F64_NUMBER);

impl Default for F64Description {
    fn default() -> Self {
        let name = "f64".to_string();
        Self {
            bytes: vec![F64_NUMBER],
            name,
        }
    }
}

impl Description for F64Description {
    fn get_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}
