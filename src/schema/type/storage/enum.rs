use crate::{
    data::row_type::RowType,
    schema::r#type::{
        data_types::{
            Bool, Byte, Double, Float, Integer, Long, Short, UInteger, ULong,
            UShort, VarChar,
        },
        r#enum::{StorageData, StorageDataType},
    },
};

use serde_storage::{
    de::{decoder, decoder::single_item::SingleItemDecoder, Error},
    ser::{
        encoder,
        encoder::{
            output_descriptor::DescriptorBytes, single_item::SingleItemEncoder,
            OutputDescriptor,
        },
    },
};

impl<const N: u16> encoder::Storable for StorageData<N> {
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

impl<const N: u16> decoder::Storable<Self> for StorageData<N> {
    fn decode(_: SingleItemDecoder, _: Vec<u8>) -> Result<Self, Error> {
        Err(Error::NotDeserializable)
    }

    fn decode_with_descriptor(
        decoder: SingleItemDecoder,
        value: Vec<u8>,
        descriptor: Vec<u8>,
    ) -> Result<Self, Error> {
        let descriptor_bytes = DescriptorBytes(descriptor);
        let descriptor: OutputDescriptor = descriptor_bytes
            .try_into()
            .map_err(|_| Error::NotDeserializable)?;
        let row_type: RowType = descriptor
            .try_into()
            .map_err(|_| Error::NotDeserializable)?;
        let types = row_type.0;

        if types.len() != 1 {
            return Err(Error::NotDeserializable);
        }

        let data_type = types.first().expect("Existing data type");
        match data_type {
            StorageDataType::Bool => Ok(decoder.emit::<Bool>(value)?.into()),
            StorageDataType::Byte => Ok(decoder.emit::<Byte>(value)?.into()),
            StorageDataType::Short => Ok(decoder.emit::<Short>(value)?.into()),
            StorageDataType::Integer => {
                Ok(decoder.emit::<Integer>(value)?.into())
            }
            StorageDataType::Long => Ok(decoder.emit::<Long>(value)?.into()),
            StorageDataType::UShort => {
                Ok(decoder.emit::<UShort>(value)?.into())
            }
            StorageDataType::UInteger => {
                Ok(decoder.emit::<UInteger>(value)?.into())
            }
            StorageDataType::ULong => Ok(decoder.emit::<ULong>(value)?.into()),
            StorageDataType::Float => Ok(decoder.emit::<Float>(value)?.into()),
            StorageDataType::Double => {
                Ok(decoder.emit::<Double>(value)?.into())
            }
            StorageDataType::VarChar(_) => {
                Ok(decoder.emit::<VarChar<N>>(value)?.into())
            }
        }
    }
}
