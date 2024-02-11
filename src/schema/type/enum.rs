pub use crate::schema::r#type::data_types::*;
use derive_more::From;

/// Represents a storage data type.
#[derive(Debug, Clone, PartialEq, PartialOrd, From)]
pub enum StorageDataType<const N: u16 = 255> {
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
