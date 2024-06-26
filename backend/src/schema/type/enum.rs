use derive_more::From;
use std::fmt::Display;

pub use crate::schema::r#type::data_types::*;

/// Represents a storage data.
#[derive(Debug, Clone, PartialEq, PartialOrd, From)]
pub enum StorageData<const N: u16 = 255> {
    Bool(Bool),
    Byte(Byte),
    Short(Short),
    Integer(Integer),
    Long(Long),
    UShort(UShort),
    UInteger(UInteger),
    ULong(ULong),
    Float(Float),
    Double(Double),
    VarChar(VarChar<N>),
}

impl<const N: u16> StorageData<N> {
    /// Returns the data type of the storage data.
    pub fn data_type(&self) -> StorageDataType {
        match self {
            StorageData::Bool(_) => StorageDataType::Bool,
            StorageData::Byte(_) => StorageDataType::Byte,
            StorageData::Short(_) => StorageDataType::Short,
            StorageData::Integer(_) => StorageDataType::Integer,
            StorageData::Long(_) => StorageDataType::Long,
            StorageData::UShort(_) => StorageDataType::UShort,
            StorageData::UInteger(_) => StorageDataType::UInteger,
            StorageData::ULong(_) => StorageDataType::ULong,
            StorageData::Float(_) => StorageDataType::Float,
            StorageData::Double(_) => StorageDataType::Double,
            StorageData::VarChar(value) => {
                StorageDataType::VarChar(value.value.len())
            }
        }
    }
}

/// Represents a storage data type.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, From)]
pub enum StorageDataType {
    Bool,
    Byte,
    Short,
    Integer,
    Long,
    UShort,
    UInteger,
    ULong,
    Float,
    Double,
    VarChar(usize),
}

impl StorageDataType {
    /// Returns the size of the data type.
    pub fn size(&self) -> usize {
        match self {
            StorageDataType::Bool => std::mem::size_of::<bool>(),
            StorageDataType::Byte => std::mem::size_of::<u8>(),
            StorageDataType::Short => std::mem::size_of::<i16>(),
            StorageDataType::Integer => std::mem::size_of::<i32>(),
            StorageDataType::Long => std::mem::size_of::<i64>(),
            StorageDataType::UShort => std::mem::size_of::<u16>(),
            StorageDataType::UInteger => std::mem::size_of::<u32>(),
            StorageDataType::ULong => std::mem::size_of::<u64>(),
            StorageDataType::Float => std::mem::size_of::<f32>(),
            StorageDataType::Double => std::mem::size_of::<f64>(),
            StorageDataType::VarChar(size) => *size,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::r#type::{
        data_types::VarChar,
        r#enum::{StorageData, StorageDataType},
    };

    #[test]
    fn test_data_type_varchar() {
        let value = VarChar::<5>::new("hello".to_string()).unwrap();
        let storage_data = StorageData::VarChar(value);

        assert_eq!(storage_data.data_type(), StorageDataType::VarChar(5));
    }
}

impl Display for StorageDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            StorageDataType::Bool => "bool".to_string(),
            StorageDataType::Byte => "u8".to_string(),
            StorageDataType::Short => "i16".to_string(),
            StorageDataType::Integer => "i32".to_string(),
            StorageDataType::Long => "i128".to_string(),
            StorageDataType::UShort => "u16".to_string(),
            StorageDataType::UInteger => "u32".to_string(),
            StorageDataType::ULong => "u128".to_string(),
            StorageDataType::Float => "f32".to_string(),
            StorageDataType::Double => "f64".to_string(),
            StorageDataType::VarChar(size) => format!("array_char_{}", size),
        };
        write!(f, "{}", str)
    }
}
