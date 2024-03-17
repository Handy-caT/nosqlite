use crate::schema::{
    r#type::{
        data_types::{Integer, Long, UInteger, ULong, UShort},
        r#enum::{StorageData, StorageDataType},
    },
    Column,
};

/// A primary key constraint.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PrimaryKey {
    /// The name of the primary key.
    name: String,
    /// The column names that make up the primary key.
    columns: String,
}

impl PrimaryKey {
    /// Creates a new [`PrimaryKey`] with the given parameters.
    /// # Arguments
    /// * `name` - The name of the primary key.
    /// * `columns` - The column names that make up the primary key.
    /// # Returns
    /// A new [`PrimaryKey`] with the given parameters.
    pub fn new(name: String, columns: String) -> Self {
        PrimaryKey { name, columns }
    }

    /// Returns the name of the primary key.
    /// # Returns
    /// * `&String` - The name of the primary key.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Returns the column names that make up the primary key.
    /// # Returns
    /// * `&Vec<String>` - The column names that make up the primary key.
    pub fn get_column(&self) -> &String {
        &self.columns
    }

    /// Checks if the column type is valid for primary key.
    /// # Arguments
    /// * `column` - The column to check.
    /// # Returns
    /// * `bool` - True if the column type is valid for primary key, false otherwise.
    pub fn check_type(column: Column) -> bool {
        match column.get_type() {
            StorageDataType::Integer
            | StorageDataType::Long
            | StorageDataType::UShort
            | StorageDataType::UInteger
            | StorageDataType::ULong => true,
            StorageDataType::Bool
            | StorageDataType::Byte
            | StorageDataType::Short
            | StorageDataType::Float
            | StorageDataType::Double
            | StorageDataType::VarChar(_) => false,
        }
    }
}

/// Enum that represents the primary key type.
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    Long,
    UShort,
    UInteger,
    ULong,
}

impl From<Type> for StorageDataType {
    fn from(pk_type: Type) -> Self {
        match pk_type {
            Type::Integer => StorageDataType::Integer,
            Type::Long => StorageDataType::Long,
            Type::UShort => StorageDataType::UShort,
            Type::UInteger => StorageDataType::UInteger,
            Type::ULong => StorageDataType::ULong,
        }
    }
}

/// Enum that represents the primary key data.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Data {
    Integer(Integer),
    Long(Long),
    UShort(UShort),
    UInteger(UInteger),
    ULong(ULong),
}

impl TryFrom<StorageData> for Data {
    type Error = &'static str;

    fn try_from(data: StorageData) -> Result<Self, Self::Error> {
        match data {
            StorageData::Integer(data) => Ok(Data::Integer(data)),
            StorageData::Long(data) => Ok(Data::Long(data)),
            StorageData::UShort(data) => Ok(Data::UShort(data)),
            StorageData::UInteger(data) => Ok(Data::UInteger(data)),
            StorageData::ULong(data) => Ok(Data::ULong(data)),
            _ => Err("Invalid data type for primary key"),
        }
    }
}

impl From<Data> for StorageData {
    fn from(pk_data: Data) -> Self {
        match pk_data {
            Data::Integer(data) => StorageData::Integer(data),
            Data::Long(data) => StorageData::Long(data),
            Data::UShort(data) => StorageData::UShort(data),
            Data::UInteger(data) => StorageData::UInteger(data),
            Data::ULong(data) => StorageData::ULong(data),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::column::primary_key::PrimaryKey;

    #[test]
    fn test_new() {
        let name = "pk".to_string();
        let column = "id".to_string();
        let pk = PrimaryKey::new(name.clone(), column.clone());

        assert_eq!(pk.name, name);
        assert_eq!(pk.columns, column);
    }

    #[test]
    fn test_get_name() {
        let name = "pk".to_string();
        let column = "id".to_string();
        let pk = PrimaryKey::new(name.clone(), column);

        assert_eq!(pk.get_name(), &name);
    }

    #[test]
    fn test_get_columns() {
        let name = "pk".to_string();
        let column = "id".to_string();
        let pk = PrimaryKey::new(name.clone(), column.clone());

        assert_eq!(pk.get_column(), &column);
    }
}
