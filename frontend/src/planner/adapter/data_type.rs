use backend::schema::r#type::r#enum::StorageDataType;

use crate::lexer::token;

impl From<token::DataType> for StorageDataType {
    fn from(data_type: token::DataType) -> Self {
        match data_type {
            token::DataType::Integer => StorageDataType::Integer,
            token::DataType::UInteger => StorageDataType::UInteger,
            token::DataType::Bool => StorageDataType::Bool,
            token::DataType::Byte => StorageDataType::Byte,
            token::DataType::Short => StorageDataType::Short,
            token::DataType::UShort => StorageDataType::UShort,
            token::DataType::Long => StorageDataType::Long,
            token::DataType::ULong => StorageDataType::ULong,
            token::DataType::Float => StorageDataType::Float,
            token::DataType::Double => StorageDataType::Double,
            token::DataType::VarChar(size) => StorageDataType::VarChar(size),
        }
    }
}
