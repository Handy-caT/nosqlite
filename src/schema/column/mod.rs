pub mod primary_key;

use crate::schema::r#type::r#enum::{StorageData, StorageDataType};

/// Represents database column.
#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    /// Marks column as not null.
    not_null: bool,
    /// Default value for column.
    default: Option<StorageData>,
    /// The type of the column.
    type_: StorageDataType,
}

impl Column {
    /// Creates a new column with the given [`StorageDataType`].
    /// # Arguments
    /// * `type_` - The type of the column.
    /// # Returns
    /// A new column with the given type.
    pub fn new(type_: StorageDataType) -> Self {
        Column {
            not_null: false,
            default: None,
            type_,
        }
    }

    /// Sets the column not null state.
    /// # Arguments
    /// * `not_null` - The not null state.
    pub fn set_not_null(&mut self, not_null: bool) {
        self.not_null = not_null;
    }

    /// Sets the default value for the column.
    /// # Arguments
    /// * `default` - The default value.
    pub fn set_default(&mut self, default: Option<StorageData>) {
        self.default = default;
    }

    /// Returns the type of the column.
    /// # Returns
    /// * `StorageDataType` - The type of the column.
    pub fn get_type(&self) -> StorageDataType {
        self.type_
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::{
        column::Column,
        r#type::r#enum::{Integer, StorageData, StorageDataType},
    };

    #[test]
    fn test_set_not_null() {
        let mut column = Column::new(StorageDataType::Integer);
        column.set_not_null(true);
        assert_eq!(column.not_null, true);
    }

    #[test]
    fn test_set_default() {
        let mut column = Column::new(StorageDataType::Integer);
        column.set_default(Some(StorageData::Integer(Integer(1))));
        assert_eq!(column.default, Some(StorageData::Integer(Integer(1))));
    }

    #[test]
    fn test_get_type() {
        let column = Column::new(StorageDataType::Integer);
        assert_eq!(column.get_type(), StorageDataType::Integer);
    }
}
