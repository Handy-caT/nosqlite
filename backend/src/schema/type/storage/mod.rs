use serde_storage::ser::encoder::{
    output_descriptor::DescriptorTypes, OutputDescriptor,
};
use std::str::FromStr as _;

use crate::{data::row_type::RowType, schema::r#type::r#enum::StorageDataType};

mod data_types;
mod r#enum;
mod integration;

impl TryFrom<OutputDescriptor> for StorageDataType {
    type Error = OutputDescriptorParseError;

    fn try_from(descriptor: OutputDescriptor) -> Result<Self, Self::Error> {
        use OutputDescriptorParseError as E;

        if descriptor.len() != 1 {
            return Err(E::InvalidValue);
        };

        let values = descriptor.get_descriptors();
        let value = values.first().expect("Existing descriptor");
        StorageDataType::from_str(value.1.as_str()).map_err(|_| E::InvalidValue)
    }
}

impl TryFrom<OutputDescriptor> for RowType {
    type Error = OutputDescriptorParseError;

    fn try_from(descriptor: OutputDescriptor) -> Result<Self, Self::Error> {
        use OutputDescriptorParseError as E;

        let values = descriptor.get_descriptors();
        let mut types = vec![];

        for value in values {
            let data_type = StorageDataType::from_str(value.1.as_str())
                .map_err(|_| E::InvalidValue)?;
            types.push(data_type);
        }

        Ok(RowType(types))
    }
}

#[derive(Debug)]
pub enum OutputDescriptorParseError {
    InvalidValue,
}

impl From<RowType> for OutputDescriptor {
    fn from(row_type: RowType) -> Self {
        let data_types = row_type
            .0
            .iter()
            .map(|data_type| data_type.to_string())
            .collect();
        let descriptor = DescriptorTypes(data_types);
        OutputDescriptor::try_from(descriptor).expect("Valid descriptor")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_storage::ser::{encoder::StorageEncoder, Storable};

    #[test]
    fn test_try_into_one() {
        let value: Vec<Box<dyn Storable>> = vec![Box::new(1u32)];

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_struct(value);
        assert!(res.is_ok());

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 1);
        assert_eq!(descriptor[0].1, "u32");

        let data_type = encoder.descriptor.try_into();
        assert!(data_type.is_ok());

        let data_type: StorageDataType = data_type.unwrap();
        assert_eq!(data_type, StorageDataType::UInteger);
    }

    #[test]
    fn test_try_into_many() {
        let value: Vec<Box<dyn Storable>> =
            vec![Box::new(1u32), Box::new(2u8), Box::new(3u16)];

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_struct(value);
        assert!(res.is_ok());

        let descriptor = encoder.descriptor.get_descriptors();
        assert_eq!(descriptor.len(), 3);
        assert_eq!(descriptor[0].1, "u32");
        assert_eq!(descriptor[1].1, "u8");
        assert_eq!(descriptor[2].1, "u16");

        let data_type = encoder.descriptor.try_into();
        assert!(data_type.is_ok());

        let data_type: RowType = data_type.unwrap();
        assert_eq!(
            data_type.0,
            vec![
                StorageDataType::UInteger,
                StorageDataType::Byte,
                StorageDataType::UShort
            ]
        );
    }

    #[test]
    fn test_try_into_row_type() {
        let value: Vec<Box<dyn Storable>> =
            vec![Box::new(1u32), Box::new(2u8), Box::new(3u16)];

        let mut encoder = StorageEncoder::new();

        let res = encoder.emit_struct(value);
        assert!(res.is_ok());

        let data_type = encoder.descriptor.try_into();
        assert!(data_type.is_ok());

        let data_type: RowType = data_type.unwrap();
        assert_eq!(
            data_type.0,
            vec![
                StorageDataType::UInteger,
                StorageDataType::Byte,
                StorageDataType::UShort
            ]
        );

        let output_descriptor: OutputDescriptor = data_type.into();
        let descriptors = output_descriptor.get_descriptors();
        assert_eq!(descriptors.len(), 3);
        assert_eq!(descriptors[0].1, "u32");
        assert_eq!(descriptors[1].1, "u8");
        assert_eq!(descriptors[2].1, "u16");
    }
}
