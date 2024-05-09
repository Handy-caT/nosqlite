//! Data selector definitions.

use crate::schema::{column, r#type::r#enum::StorageDataType};

/// Represents a data selector.
#[derive(Debug, Clone)]
pub struct DataSelector {
    /// The column names to select. None means all columns.
    pub row_names: Option<Vec<column::Name>>,

    /// The filters to apply.
    pub filters: Option<Vec<SelectorFilter>>,
}

/// Represents a filter for a column.
#[derive(Debug, Clone)]
pub struct SelectorFilter {
    /// The column name.
    pub column_name: column::Name,

    /// The value to compare.
    pub value: StorageDataType,

    /// The filter type.
    pub filter_type: FilterType,
}

/// Represents a [`SelectorFilter`] type for a column.
#[derive(Debug, Clone, PartialEq)]
pub enum FilterType {
    /// Represents an equal filter.
    Equal,

    /// Represents a not equal filter.
    NotEqual,

    /// Represents a greater than filter.
    GreaterThan,

    /// Represents a less than filter.
    LessThan,

    /// Represents a greater than or equal filter.
    GreaterThanOrEqual,

    /// Represents a less than or equal filter.
    LessThanOrEqual,
}
