
pub trait HashTable<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: K) -> Option<V>;
    fn get(&self, key: K) -> Option<&V>;

    fn len(&self) -> usize;
}