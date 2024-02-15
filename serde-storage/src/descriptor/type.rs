use crate::{book_values, descriptor::Description};
use crate::descriptor::Describable;

/// Descriptor for a char.
pub struct CharDescription {
    /// Bytes of the description.
    bytes: Vec<u8>,

    /// Type name of the integer.
    name: String,
}

const CHAR_NUMBER: u8 = 21;
book_values!(CHAR_NUMBER);

impl Default for CharDescription {
    fn default() -> Self {
        let name = "char".to_string();
        Self {
            bytes: vec![CHAR_NUMBER],
            name,
        }
    }
}

impl Description for CharDescription {
    fn get_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Describable<CharDescription> for char {
    fn describe() -> CharDescription {
        CharDescription::default()
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

impl Describable<BoolDescription> for bool {
    fn describe() -> BoolDescription {
        BoolDescription::default()
    }
}

/// Descriptor for a f32.
pub struct F32Description {
    /// Bytes of the description.
    bytes: Vec<u8>,

    /// Type name of the integer.
    name: String,
}

const F32_NUMBER: u8 = 22;
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

impl Describable<F32Description> for f32 {
    fn describe() -> F32Description {
        F32Description::default()
    }
}

/// Descriptor for a f64.
pub struct F64Description {
    /// Bytes of the description.
    bytes: Vec<u8>,

    /// Type name of the integer.
    name: String,
}

const F64_NUMBER: u8 = 23;
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

impl Describable<F64Description> for f64 {
    fn describe() -> F64Description {
        F64Description::default()
    }
}
