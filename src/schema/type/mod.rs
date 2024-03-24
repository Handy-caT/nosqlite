use std::str::FromStr;
use serde_storage::de::decoder::single_item::SingleItemDecoder;
use serde_storage::de::decoder::Storable;
use serde_storage::de::Error;
use serde_storage::ser::encoder::OutputDescriptor;
use crate::schema::r#type::r#enum::{StorageData, StorageDataType};

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
                    let size = input.trim_start_matches("array_char_").parse().unwrap();
                    return Ok(StorageDataType::VarChar(size));
                }

                Err(())
            },
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
            let data_type = StorageDataType::from_str(&descriptor.1).map_err(|_| DescriptorError::InvalidDescriptor)?;
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
    use serde_storage::ser::{Storable, StorageEncoder};
    use super::*;

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

    fn decode_with_descriptor(decoder: SingleItemDecoder, value: Vec<u8>, descriptor: Vec<u8>) -> Result<Self, Error> {
        todo!()
    }
}