use serde_storage::{
    error::Error,
    ser::encoder::{single_item::SingleItemEncoder, storable::Storable},
};

use crate::schema::r#type::data_types::Byte;

impl Storable for Byte {
    fn encode(&self, encoder: SingleItemEncoder) -> Result<(), Error> {
        encoder.emit(*self)
    }
}
