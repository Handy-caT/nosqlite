use crate::ser::encoder::StorageEncoder;

/// Storage encoder for structs.
pub struct StructEncoder<'a> {
    /// Encoder for the storage.
    encoder: &'a mut StorageEncoder,
}

impl<'a> StructEncoder<'a> {}

pub struct StructDescriptor {
    /// Bytes of the description.
    bytes: Vec<u8>,

    /// Type name of the integer.
    name: String,
}
