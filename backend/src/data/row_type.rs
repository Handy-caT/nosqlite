use crate::schema::r#type::r#enum::StorageDataType;

/// [`RowType`] is a struct that is used to store data types of a row.
#[derive(Debug, Clone, Default)]
pub struct RowType(pub Vec<StorageDataType>);

impl RowType {
    /// Returns the length of the [`RowType`].
    /// # Returns
    /// * `usize` - Length of the [`RowType`].
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the data type at the provided index.
    /// # Arguments
    /// * `index` - Index of the data type.
    /// # Returns
    /// * `Option<StorageDataType>` - Data type at the provided index.
    pub fn get(&self, index: usize) -> Option<StorageDataType> {
        self.0.get(index).copied()
    }

    /// Returns the data types of the [`RowType`] as a vector of strings.
    pub fn into_data_types(self) -> Vec<String> {
        self.0
            .into_iter()
            .map(|data_type| data_type.to_string())
            .collect()
    }
}
