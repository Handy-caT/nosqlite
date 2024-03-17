use crate::schema::r#type::r#enum::StorageData;
use serde_storage::ser::{encoder, encoder::single_item::SingleItemEncoder};

impl encoder::Storable for StorageData {
    fn encode(
        &self,
        encoder: SingleItemEncoder,
    ) -> Result<(), serde_storage::ser::Error> {
        match self {
            StorageData::Bool(value) => encoder.emit(value.0),
            StorageData::Byte(value) => encoder.emit(value.0),
            StorageData::Short(value) => encoder.emit(value.0),
            StorageData::Integer(value) => encoder.emit(value.0),
            StorageData::Long(value) => encoder.emit(value.0),
            StorageData::UShort(value) => encoder.emit(value.0),
            StorageData::UInteger(value) => encoder.emit(value.0),
            StorageData::ULong(value) => encoder.emit(value.0),
            StorageData::Float(value) => encoder.emit(value.0),
            StorageData::Double(value) => encoder.emit(value.0),
            StorageData::VarChar(value) => encoder.emit_str(&value.value),
        }
    }
}
