use crate::serde::ser::encoder::StorageEncoder;

pub struct SingleItemEncoder<'a> {
    encoder: &'a StorageEncoder,

    value_written: &'a bool,
}
