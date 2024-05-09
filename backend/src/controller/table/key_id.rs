//! Contains the [`KeyId`] type definition.

use crate::{data::id::NumericId, schema::column::primary_key};

/// Represents a mapper from a primary key to a unique identifier.
#[derive(Debug, Clone)]
pub struct KeyId {
    /// The unique identifier.
    pub id: NumericId,

    /// The primary key value.
    pub key: primary_key::Data,
}

impl PartialEq for KeyId {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for KeyId {}

impl PartialOrd for KeyId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for KeyId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}
