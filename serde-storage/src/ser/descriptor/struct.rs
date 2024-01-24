use crate::ser::encoder::StorageEncoder;

/// Storage encoder for structs.
pub struct StructEncoder<'a> {
    /// Encoder for the storage.
    encoder: &'a mut StorageEncoder,
}

impl<'a> StructEncoder<'a> {
}
