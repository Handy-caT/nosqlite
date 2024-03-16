use crate::schema::r#type::{
    data_types::{Integer, Long, UInteger, ULong, UShort},
    r#enum::{StorageData, StorageDataType},
};

/// A primary key constraint.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PrimaryKey {
    /// The name of the primary key.
    name: String,
    /// The column names that make up the primary key.
    columns: Vec<String>,
}

impl PrimaryKey {
    /// Creates a new [`PrimaryKey`] with the given parameters.
    /// # Arguments
    /// * `name` - The name of the primary key.
    /// * `columns` - The column names that make up the primary key.
    /// # Returns
    /// A new [`PrimaryKey`] with the given parameters.
    pub fn new(name: String, columns: Vec<String>) -> Self {
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
    pub fn get_columns(&self) -> &Vec<String> {
        &self.columns
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
        let columns = vec!["id".to_string()];
        let pk = PrimaryKey::new(name.clone(), columns.clone());

        assert_eq!(pk.name, name);
        assert_eq!(pk.columns, columns);
    }

    #[test]
    fn test_get_name() {
        let name = "pk".to_string();
        let columns = vec!["id".to_string()];
        let pk = PrimaryKey::new(name.clone(), columns.clone());

        assert_eq!(pk.get_name(), &name);
    }

    #[test]
    fn test_get_columns() {
        let name = "pk".to_string();
        let columns = vec!["id".to_string()];
        let pk = PrimaryKey::new(name.clone(), columns.clone());

        assert_eq!(pk.get_columns(), &columns);
    }
}
