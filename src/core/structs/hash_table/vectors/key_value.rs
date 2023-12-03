use std::{cmp::Ordering, fmt::Debug};

/// [`KeyValue`] is a struct that holds a key and a value.
/// It is used as a value in [`HashVec`].
/// * `K` - key type
/// * `V` - value type
#[derive(Clone, Copy, Debug, Default)]
pub struct KeyValue<K, V> {
    pub key: K,
    pub value: V,
}

/// Equality is based on the key
impl<K: PartialEq, V: PartialEq> PartialEq<Self> for KeyValue<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Copy, V: Copy> KeyValue<K, V> {
    /// Creates a new [`KeyValue`]
    /// # Arguments
    /// * `key` - key
    /// * `value` - value
    /// # Returns
    /// * `Self` - [`KeyValue`]
    pub fn new(key: K, value: V) -> Self {
        KeyValue { key, value }
    }

    /// Returns a tuple of (key, value)
    /// # Returns
    /// * `(K, V)` - Tuple of (key, value)
    pub fn as_tuple(&self) -> (K, V) {
        (self.key, self.value)
    }
}

impl<K: PartialOrd, V: PartialOrd> PartialOrd for KeyValue<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::structs::hash_table::vectors::key_value::KeyValue;

    #[test]
    fn test_key_value_new() {
        let key_value = KeyValue::new(1, 2);
        assert_eq!(key_value.key, 1);
        assert_eq!(key_value.value, 2);
    }

    #[test]
    fn test_key_value_as_tuple() {
        let key_value = KeyValue::new(1, 2);
        assert_eq!(key_value.as_tuple(), (1, 2));
    }
}
