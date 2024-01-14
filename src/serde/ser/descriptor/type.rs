use crate::serde::ser::descriptor::Description;

/// Descriptor for a [`String`].
pub struct StringDescription {
    /// Bytes of the description.
    bytes: Vec<u8>,

    /// Type name of the integer.
    name: String,
}

impl StringDescription {
    /// Create a new [`StringDescription`].
    pub fn new(len: usize) -> Self {
        let name = format!("str{}", len);
        Self {
            bytes: vec![6],
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

impl BoolDescription {
    /// Create a new [`BoolDescription`].
    pub fn new() -> Self {
        let name = "bool".to_string();
        Self {
            bytes: vec![0],
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
