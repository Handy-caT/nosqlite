use crate::schema::r#type::r#enum::{StorageData, StorageDataType};
use serde_storage::{
    de::{
        decoder::{single_item::SingleItemDecoder, Storable},
        Error,
    },
    ser::encoder::OutputDescriptor,
};
use std::str::FromStr;
use serde_storage::ser::encoder::output_descriptor::DescriptorBytes;
use crate::data::row_type::RowType;

pub mod data_types;
pub mod r#enum;
mod storage;

impl FromStr for StorageDataType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "bool" => Ok(StorageDataType::Bool),
            "u8" => Ok(StorageDataType::Byte),
            "i16" => Ok(StorageDataType::Short),
            "i32" => Ok(StorageDataType::Integer),
            "i128" => Ok(StorageDataType::Long),
            "u16" => Ok(StorageDataType::UShort),
            "u32" => Ok(StorageDataType::UInteger),
            "u128" => Ok(StorageDataType::ULong),
            "f32" => Ok(StorageDataType::Float),
            "f64" => Ok(StorageDataType::Double),
            _ => {
                if input.starts_with("array_char_") {
                    let size = input
                        .trim_start_matches("array_char_")
                        .parse()
                        .unwrap();
                    return Ok(StorageDataType::VarChar(size));
                }

                Err(())
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Descriptor(OutputDescriptor);

impl TryInto<Vec<StorageDataType>> for Descriptor {
    type Error = DescriptorError;

    fn try_into(self) -> Result<Vec<StorageDataType>, Self::Error> {
        let descriptors = self.0.get_descriptors();
        let mut data_types = Vec::new();

        for descriptor in descriptors {
            let data_type = StorageDataType::from_str(&descriptor.1)
                .map_err(|_| DescriptorError::InvalidDescriptor)?;
            data_types.push(data_type);
        }

        Ok(data_types)
    }
}

#[derive(Debug)]
pub enum DescriptorError {
    InvalidDescriptor,
}

#[cfg(test)]
mod test_descriptor {
    use super::*;
    use serde_storage::ser::{Storable, StorageEncoder};

    #[test]
    fn test_descriptor() {
        let value: Vec<Box<dyn Storable>> = vec![
            Box::new(1u32),
            Box::new(true),
            Box::new("Hello, world!".to_string()),
        ];

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_struct(value);
        assert!(res.is_ok());

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 3);
        assert_eq!(descriptor[0].1, "u32");
        assert_eq!(descriptor[1].1, "bool");
        assert_eq!(descriptor[2].1, "array_char_13");

        let descriptor: Descriptor = Descriptor(encoder.descriptor);
        let data_types: Vec<StorageDataType> = descriptor.try_into().unwrap();

        assert_eq!(data_types.len(), 3);
        assert_eq!(data_types[0], StorageDataType::UInteger);
        assert_eq!(data_types[1], StorageDataType::Bool);
        assert_eq!(data_types[2], StorageDataType::VarChar(13));
    }
}

#[derive(Debug, Clone)]
pub struct DataRow(Vec<StorageData>);

impl Storable<Self> for DataRow {
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
        
        let mut data_vec = Vec::new();
        let mut i = 0;
        
        for (bytes, type_) in descriptor.get_descriptors() {
            let type_ = StorageDataType::from_str(type_.as_str()).map_err(|_| Error::NotDeserializable)?;
            let part_len = type_.size();
            let part = value[i..i + part_len].to_vec();
            i += part_len;
            
            let decoder = SingleItemDecoder {
                decoder: decoder.decoder
            };
            
            let data = decoder.emit_with_descriptor::<StorageData>(part, bytes)?;
            data_vec.push(data);
        }
        
        Ok(DataRow(data_vec))
    }
}

#[cfg(test)]
mod test_decode_data_row {
    use serde_storage::de::decoder::StorageDecoder;
    use serde_storage::ser::{Storable, StorageEncoder};
    use crate::schema::r#type::data_types::VarChar;
    use crate::schema::r#type::DataRow;
    use crate::schema::r#type::r#enum::StorageData;

    #[test]
    fn test_data_row() {
        let value: Vec<Box<dyn Storable>> = vec![
            Box::new(1u32),
            Box::new(true),
            Box::new("Hello, world!".to_string()),
        ];

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_struct(value);
        assert!(res.is_ok());
        let bytes = encoder.output.get_bytes();
        let descriptor = encoder.descriptor.get_descriptor_bytes();
        
        let mut decoder = StorageDecoder;
        
        let data = decoder.emit_with_descriptor::<DataRow>(bytes, descriptor);
        assert!(data.is_ok());
        
        let data = data.unwrap().0;
        assert_eq!(data.len(), 3);
        assert_eq!(data[0], StorageData::UInteger(1.into()));
        assert_eq!(data[1], StorageData::Bool(true.into()));
        assert_eq!(data[2], StorageData::VarChar(VarChar::new("Hello, world!".to_string()).unwrap()));
    }
}