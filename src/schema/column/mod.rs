pub mod primary_key;

use crate::schema::r#type::r#enum::StorageData;

/// Represents database column.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Column {
    /// Marks column as not null.
    not_null: bool,
    /// Default value for column.
    default: Option<StorageData>,
}

impl Column {
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
}


#[cfg(test)]
mod tests {
    use crate::schema::{
        column::Column,
        r#type::r#enum::{Integer, StorageData},
    };
    
    #[test]
    fn test_set_not_null() {
        let mut column = Column::default();
        column.set_not_null(true);
        assert_eq!(column.not_null, true);
    }
    
    #[test]
    fn test_set_default() {
        let mut column = Column::default();
        column.set_default(Some(StorageData::Integer(Integer(1))));
        assert_eq!(column.default, Some(StorageData::Integer(Integer(1))));
    }
}
