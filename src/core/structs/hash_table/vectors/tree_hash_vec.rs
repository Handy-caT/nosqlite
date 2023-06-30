use crate::core::structs::hash_table::vectors::hash_vec::{HashVec, HashVecIndexes, HashVecStatisticsInternal};
use crate::core::structs::hash_table::vectors::key_value::KeyValue;
use crate::core::structs::hash_table::vectors::statistics::hash_vec_statistics::HashVecStatistics;
use crate::core::structs::hash_table::vectors::statistics::statistics_functions::{statistics_add_actions, statistics_remove_actions};
use crate::core::structs::tree::object::balanced_tree::balanced_tree::BalancedTree;
use crate::core::structs::tree::object::tree_object::{TreeObject, TreeObjectVec};
use crate::core::structs::tree::vectors::optimized_tree_vec::OptimizedTreeVec;

/// A hash vector that uses a tree to store the values.
/// * `V` - Type of the value
/// * `N` - Number of buckets, must be a power of 2, if it is not, it will be rounded up to the next power of 2
pub struct TreeHashVec<K: Copy + Default + PartialOrd, V: Copy + Default +  PartialOrd, const N: u64> {
    /// The data of the hash vector as a vector of trees.
    /// OptimizedTreeVec is used as the underlying data structure for the trees.
    data: Vec< BalancedTree< KeyValue<K, V>, OptimizedTreeVec< KeyValue<K, V> > > >,
    /// The size of the hash vector. This is the number of buckets.
    /// It is a power of 2. If N is not a power of 2, it will be rounded up to the next power of 2.
    pub size: u64,
    /// Statistics of the hash vector
    statistics: HashVecStatistics,
}

impl <K: Copy + Default +  PartialOrd, V: Copy + Default + PartialOrd, const N: u64> TreeHashVec<K, V, N> {
    /// Creates a new TreeHashVec
    /// # Returns
    /// * `TreeHashVec<V, N>` - New TreeHashVec
    pub fn new() -> TreeHashVec<K, V, N> {
        let mut vec = TreeHashVec {
            data: Vec::new(),
            size: N,
            statistics: HashVecStatistics::new(N as usize),
        };

        let mut size= N;

        if (N as f64).log2() != (N as f64).log2().floor() {
            let pow= (N as f64).log2().ceil() as u64;
            size = 2u64.pow(pow as u32);
        }

        for _ in 0..size {
            let nodes = OptimizedTreeVec::new();
            vec.data.push(BalancedTree::< KeyValue<K, V>, OptimizedTreeVec< KeyValue<K, V> > >::new(nodes));
        }

        vec.size = size;

        vec
    }
}

/// Implementation of basic HashVec trait for TreeHashVec
impl <K: Default + Eq + Copy + PartialOrd, V: Default + Eq + Copy + PartialOrd, const N: u64> HashVec<K, V, N> for TreeHashVec<K, V, N> {
    fn push(&mut self, index: u64, key: K, value: V) -> (u64, usize) {
        let data = KeyValue::new(key, value);

        let data_index = self.data[index as usize].push(data);
        statistics_add_actions(self, index);

        (index, data_index as usize)
    }

    fn get(&mut self, index: u64, key: K) -> Option<KeyValue<K, V>> {
        let item = KeyValue::new(key, V::default());
        let item_index = self.data[index as usize].find(item);

        match item_index {
            Some(i) => self.data[index as usize].get(i),
            None => None,
        }
    }

    fn update(&mut self, index: u64, key: K, value: V) -> Option<KeyValue<K, V>> {
        let item = KeyValue::new(key, V::default());
        let item_index = self.data[index as usize].find(item);

        match item_index {
            Some(i) => {
                let item = KeyValue::new(key, value);
                self.data[index as usize].remove_by_index(i);
                self.data[index as usize].push(item);
                Some(item)
            },
            None => None,
        }
    }

    fn have_key(&mut self, index: u64, key: K) -> bool {
        let item_index = self.find_key(index, key);
        match item_index {
            Some(_) => true,
            None => false,
        }
    }

    fn remove(&mut self, index: u64, key: K) -> Option<KeyValue<K, V>> {
        let item = KeyValue::new(key, V::default());
        let has_item = self.data[index as usize].find(item);

        match has_item {
            Some(_) => {
                statistics_remove_actions(self, index);
                let item = self.data[index as usize].remove_by_value(item);
                Some(item.unwrap())
            },
            None => None,
        }
    }

    fn size(&self) -> u64 {
        self.size
    }

    fn len(&self) -> u64 {
        self.statistics.size
    }
}

/// Implementation of HashVecStatisticsInternal trait for TreeHashVec
impl <K: Default + Eq + Copy+ PartialOrd, V: Default + Eq + Copy+ PartialOrd, const N: u64> HashVecStatisticsInternal<K, V, N> for TreeHashVec<K, V, N> {
    fn get_max_len(&self) -> usize {
        self.statistics.max_length
    }

    fn get_statistics(&self) -> &HashVecStatistics {
        &self.statistics
    }

    fn get_statistics_mut(&mut self) -> &mut HashVecStatistics {
        &mut self.statistics
    }

    fn get_bucket_len(&self, index: u64) -> Option<usize> {
        if index >= N {
            None
        } else {
            Some(self.data[index as usize].len())
        }
    }
}

/// Implementation of HashVecIndexes trait for TreeHashVec
impl <K: Default + Eq + Copy+ PartialOrd, V: Eq + Copy + Default + PartialOrd, const N: u64> HashVecIndexes<K, V, N> for TreeHashVec<K, V, N> {
    fn remove_by_index(&mut self, index: u64, value_index: usize) -> Option<KeyValue<K, V>> {
        let has_item = self.data[index as usize].get(value_index as i32);
        match has_item {
            Some(_) => {
                statistics_remove_actions(self, index);
                let item = self.data[index as usize].remove_by_index(value_index as i32);
                Some(item.unwrap())
            },
            None => None,
        }
    }

    fn get_by_index(&mut self, index: u64, value_index: usize) -> Option<KeyValue<K, V>> {
        self.data[index as usize].get(value_index as i32)
    }

    fn find_key(&mut self, index: u64, key: K) -> Option<usize> {
        let item = KeyValue::new(key, V::default());
        let item_index = self.data[index as usize].find(item);

        match item_index {
            Some(i) => Some(i as usize),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_hash_vec_new() {
        let vec = TreeHashVec::<u64, u64, 8>::new();

        assert_eq!(vec.data.len(), 8);
        for i in 0..8 {
            assert_eq!(vec.data[i].len(), 0);
        }
        assert_eq!(vec.statistics.size, 0);
    }

    #[test]
    fn test_static_hash_vec_new_sizes() {
        let vec = TreeHashVec::<u64, u64, 10>::new();

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.data.len(), 16);
        assert_eq!(vec.size, 16);
        assert_eq!(vec.size(), 16);

        let vec = TreeHashVec::<u64, u64, 32>::new();

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.data.len(), 32);
        assert_eq!(vec.size, 32);
        assert_eq!(vec.size(), 32);
    }

    #[test]
    fn test_tree_hash_vec_push() {
        let mut vec = TreeHashVec::<u64, u64, 8>::new();

        let (index, value_index) = vec.push(0, 1, 1);

        assert_eq!(index, 0);
        assert_eq!(value_index, 0);
        assert_eq!(vec.data[0].len(), 1);
        assert_eq!(vec.size(), 8);

        assert_eq!(vec.statistics.size, 1);
        assert_eq!(vec.statistics.max_length, 1);
        assert_eq!(vec.statistics.get_count(), 1);
    }

    #[test]
    fn test_tree_hash_vec_update() {
        let mut vec = TreeHashVec::<u64, u64, 8>::new();

        vec.push(0, 1, 1);
        vec.push(0, 2, 2);

        assert_eq!(vec.len(), 2);
        assert_eq!(vec.statistics.size, 2);

        assert_eq!(vec.get(0, 1), Some(KeyValue::new(1, 1)));
        assert_eq!(vec.get(0, 2), Some(KeyValue::new(2, 2)));

        vec.update(0, 1, 2);

        assert_eq!(vec.len(), 2);
        assert_eq!(vec.statistics.size, 2);
        assert_eq!(vec.get(0, 1), Some(KeyValue::new(1, 2)));
    }

    #[test]
    fn test_tree_hash_vec_remove_by_index() {
        let mut vec = TreeHashVec::<u64, u64, 8>::new();

        vec.push(0, 1, 1);
        vec.push(0, 2, 2);

        let item = vec.remove_by_index(0, 0);

        assert_eq!(item, Some(KeyValue::new(1, 1)));
        assert_eq!(vec.data[0].len(), 1);
        assert_eq!(vec.size(), 8);

        assert_eq!(vec.statistics.size, 1);
        assert_eq!(vec.statistics.max_length, 1);
        assert_eq!(vec.statistics.get_count(), 1);

        let item = vec.remove_by_index(0, 3);

        assert_eq!(item, None);
    }

    #[test]
    fn test_tree_hash_vec_get() {
        let mut vec = TreeHashVec::<u64, u64, 8>::new();

        vec.push(0, 1, 1);
        vec.push(0, 2, 2);

        assert_eq!(vec.get(0, 1), Some(KeyValue::new(1, 1)));
        assert_eq!(vec.get(0, 2), Some(KeyValue::new(2, 2)));
        assert_eq!(vec.get(0, 3), None);
    }

    #[test]
    fn test_tree_hash_vec_have_key() {
        let mut vec = TreeHashVec::<u64, u64, 8>::new();

        vec.push(0, 1, 1);
        vec.push(0, 2, 2);

        assert_eq!(vec.have_key(0, 1), true);
        assert_eq!(vec.have_key(0, 2), true);
        assert_eq!(vec.have_key(0, 3), false);
    }

    #[test]
    fn test_tree_hash_vec_get_by_index() {
        let mut vec = TreeHashVec::<u64, u64, 8>::new();

        vec.push(0, 1, 1);
        vec.push(0, 2, 2);

        assert_eq!(vec.get_by_index(0, 0), Some(KeyValue::new(1, 1)));
        assert_eq!(vec.get_by_index(0, 1), Some(KeyValue::new(2, 2)));
        assert_eq!(vec.get_by_index(0, 2), None);
    }

    #[test]
    fn test_tree_hash_vec_find_key() {
        let mut vec = TreeHashVec::<u64, u64, 8>::new();

        vec.push(0, 1, 1);
        vec.push(0, 2, 2);

        assert_eq!(vec.find_key(0, 1), Some(0));
        assert_eq!(vec.find_key(0, 2), Some(1));
        assert_eq!(vec.find_key(0, 3), None);
    }

    #[test]
    fn test_tree_hash_get_bucket_len() {
        let mut vec = TreeHashVec::<u64, u64, 8>::new();

        vec.push(0, 1, 1);
        vec.push(0, 2, 2);

        assert_eq!(vec.get_bucket_len(0), Some(2));
        assert_eq!(vec.get_bucket_len(1), Some(0));
        assert_eq!(vec.get_bucket_len(9), None);
    }

    #[test]
    fn test_tree_hash_vec_remove() {
        let mut vec = TreeHashVec::<u64, u64, 8>::new();

        vec.push(0, 1, 1);
        vec.push(0, 2, 2);

        assert_eq!(vec.statistics.get_count(), 1);
        assert_eq!(vec.statistics.max_length, 2);

        assert_eq!(vec.remove(0, 1), Some(KeyValue::new(1, 1)));
        assert_eq!(vec.find_key(0, 1), None);
        assert_eq!(vec.have_key(0, 1), false);
        assert_eq!(vec.statistics.get_count(), 1);
        assert_eq!(vec.statistics.max_length, 1);

        assert_eq!(vec.remove(0, 2), Some(KeyValue::new(2, 2)));
        assert_eq!(vec.find_key(0, 2), None);
        assert_eq!(vec.have_key(0, 2), false);
        assert_eq!(vec.statistics.get_count(), 0);
        assert_eq!(vec.statistics.max_length, 0);

        assert_eq!(vec.remove(0, 3), None);

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.statistics.size, 0);

        assert_eq!(vec.data[0].len(), 0);
    }
}