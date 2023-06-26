use std::cmp::Ordering;
use std::fmt::Debug;

/// KeyValue is a struct that holds a key and a value.
/// It is used as a value in HashVec.
/// * `K` - key type
/// * `V` - value type
pub struct KeyValue<K, V> {
    pub key: K,
    pub value: V
}

impl<K: Eq, V: Eq> PartialEq<Self> for KeyValue<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}

impl<K: Eq + Copy, V: Eq + Copy> KeyValue<K, V> {
    pub fn new(key: K, value: V) -> Self {
        KeyValue {
            key,
            value
        }
    }

    pub fn as_tuple(&self) -> (K, V) {
        (self.key, self.value)
    }
}

impl<K: Eq + PartialOrd, V: Eq + PartialOrd> PartialOrd for KeyValue<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl<K: Copy, V: Copy> Clone for KeyValue<K, V> {
    fn clone(&self) -> Self {
        KeyValue {
            key: self.key,
            value: self.value
        }
    }
}

impl<K: Copy, V: Copy> Copy for KeyValue<K, V> {}

impl<K: Debug, V: Debug> Debug for KeyValue<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyValue")
            .field("key", &self.key)
            .field("value", &self.value)
            .finish()
    }
}