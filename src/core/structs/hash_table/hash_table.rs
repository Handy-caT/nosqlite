

struct HashTable<K, V> {
    table: Vec<Vec<(K, V)>>,
    size: usize,
}