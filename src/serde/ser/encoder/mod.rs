mod single_item;
mod storable;
pub mod storable_integer;

pub use storable_integer::StorableInteger;

/// Output bytes after encoding.
#[derive(Default, Debug, Clone)]
pub struct OutputBytes(Vec<u8>);

impl OutputBytes {
    /// Create a new [`OutputBytes`].
    pub fn new() -> Self {
        OutputBytes(Vec::new())
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
#[derive(Default, Debug, Clone)]
pub struct DescriptorBytes(Vec<u8>);

impl DescriptorBytes {
    /// Create a new [`DescriptorBytes`].
    pub fn new() -> Self {
        DescriptorBytes(Vec::new())
    }

    /// Get the bytes from the [`DescriptorBytes`].
    pub fn get_bytes(self) -> Vec<u8> {
        self.0
    }

    /// Append bytes to the [`DescriptorBytes`].
    pub fn append(&mut self, bytes: Vec<u8>) {
        self.0.append(&mut bytes.clone());
    }
}

#[derive(Default, Debug)]
pub struct StorageEncoder {
    output: OutputBytes,
    descriptor: DescriptorBytes,
}

impl StorageEncoder {
    /// Create a new [`StorageEncoder`].
    pub fn new() -> Self {
        <Self as Default>::default()
    }
}
