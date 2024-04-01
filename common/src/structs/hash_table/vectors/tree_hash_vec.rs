use crate::structs::{
    hash_table::vectors::{
        hash_vec::{HashVec, Indexes, InternalStatistics},
        key_value::KeyValue,
        statistics::{
            functions::{statistics_add_actions, statistics_remove_actions},
            hash_vec,
        },
    },
    tree::{
        object::{
            tree::{Tree, VecFunctions},
            BalancedTree,
        },
        vectors::{optimized_tree_vec::OptimizedTreeVec, tree_vec::TreeVec},
    },
};

type TreeBuckets<K, V> =
    Vec<BalancedTree<KeyValue<K, V>, OptimizedTreeVec<KeyValue<K, V>>>>;

/// A hash vector that uses a tree to store the values.
/// * `V` - Type of the value
/// * `K` - Type of the key
#[derive(Debug)]
pub struct TreeHashVec<
    K: Clone + Default + PartialOrd,
    V: Clone + Default + PartialOrd,
> {
    /// The data of the hash vector as a vector of trees.
    /// [`OptimizedTreeVec`] is used as the underlying data
    /// structure for the trees.
    data: TreeBuckets<K, V>,

    /// The size of the hash vector. This is the number of buckets.
    /// It is a power of 2. If N is not a power of 2,
    /// it will be rounded up to the next power of 2.
    pub size: usize,

    /// Statistics of the hash vector
    statistics: hash_vec::Stats,
}

/// Implementation of basic [`HashVec`] trait for [`TreeHashVec`]
impl<
        K: Default + Eq + Clone + PartialOrd,
        V: Default + Eq + Clone + PartialOrd,
    > HashVec<K, V> for TreeHashVec<K, V>
{
    fn new(size: usize) -> TreeHashVec<K, V> {
        let mut vec = TreeHashVec {
            data: Vec::new(),
            size,
            statistics: hash_vec::Stats::new(size),
        };

        let mut pow = size.ilog2();
        if size > 2usize.pow(pow) {
            pow += 1;
        }
        let size = 2usize.pow(pow);

        for _ in 0..size {
            vec.data.push(BalancedTree::<
                KeyValue<K, V>,
                OptimizedTreeVec<KeyValue<K, V>>,
            >::default());
        }

        vec.size = size;

        vec
    }

    fn push(&mut self, index: usize, key: K, value: V) -> (usize, usize) {
        let data = KeyValue::new(key, value);

        let data_index = self.data[index].push(data);
        statistics_add_actions(self, index);

        (index, data_index)
    }

    fn get(&mut self, index: usize, key: &K) -> Option<KeyValue<K, V>> {
        let item = KeyValue::new(key.clone(), V::default());
        let item_index = self.data[index].find(&item);

        match item_index {
            Some(i) => self.data[index].get(i),
            None => None,
        }
    }

    fn update(
        &mut self,
        index: usize,
        key: &K,
        value: V,
    ) -> Option<KeyValue<K, V>> {
        let item = KeyValue::new(key.clone(), V::default());
        let item_index = self.data[index].find(&item);

        match item_index {
            Some(i) => {
                let item = KeyValue::new(key.clone(), value);
                self.data[index].remove_by_index(i);
                self.data[index].push(item.clone());
                Some(item)
            }
            None => None,
        }
    }

    fn have_key(&mut self, index: usize, key: &K) -> bool {
        let item_index = self.find_key(index, key);
        item_index.is_some()
    }

    fn remove(&mut self, index: usize, key: &K) -> Option<KeyValue<K, V>> {
        let item = KeyValue::new(key.clone(), V::default());
        let has_item = self.data[index].find(&item);

        match has_item {
            Some(_) => {
                statistics_remove_actions(self, index);
                let item = self.data[index].remove_by_value(&item);
                Some(item.unwrap())
            }
            None => None,
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn len(&self) -> usize {
        self.statistics.size
    }
}

/// Implementation of [`InternalStatistics`] trait for [`TreeHashVec`]
impl<
        K: Default + Eq + Clone + PartialOrd,
        V: Default + Eq + Clone + PartialOrd,
    > InternalStatistics<K, V> for TreeHashVec<K, V>
{
    fn get_max_len(&self) -> usize {
        self.statistics.max_length
    }

    fn get_statistics(&self) -> &hash_vec::Stats {
        &self.statistics
    }

    fn get_statistics_mut(&mut self) -> &mut hash_vec::Stats {
        &mut self.statistics
    }

    fn get_bucket_len(&self, index: usize) -> Option<usize> {
        if index >= self.size {
            None
        } else {
            Some(self.data[index].len())
        }
    }
}

/// Implementation of [`Indexes`] trait for [`TreeHashVec`]
impl<
        K: Default + Eq + Clone + PartialOrd,
        V: Eq + Clone + Default + PartialOrd,
    > Indexes<K, V> for TreeHashVec<K, V>
{
    fn remove_by_index(
        &mut self,
        index: usize,
        value_index: usize,
    ) -> Option<KeyValue<K, V>> {
        let has_item = self.data[index].get(value_index);
        match has_item {
            Some(_) => {
                statistics_remove_actions(self, index);
                let item = self.data[index].remove_by_index(value_index);
                Some(item.unwrap())
            }
            None => None,
        }
    }

    fn get_by_index(
        &mut self,
        index: usize,
        value_index: usize,
    ) -> Option<KeyValue<K, V>> {
        self.data[index].get(value_index)
    }

    fn find_key(&mut self, index: usize, key: &K) -> Option<usize> {
        let item = KeyValue::new(key.clone(), V::default());
        self.data[index].find(&item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_hash_vec_new() {
        let vec = TreeHashVec::<usize, usize>::new(8);

        assert_eq!(vec.data.len(), 8);
        for i in 0..8 {
            assert_eq!(vec.data[i].len(), 0);
        }
        assert_eq!(vec.statistics.size, 0);
    }

    #[test]
    fn test_static_hash_vec_new_sizes() {
        let vec = TreeHashVec::<usize, usize>::new(10);

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.data.len(), 16);
        assert_eq!(vec.size, 16);
        assert_eq!(vec.size(), 16);

        let vec = TreeHashVec::<usize, usize>::new(32);

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.data.len(), 32);
        assert_eq!(vec.size, 32);
        assert_eq!(vec.size(), 32);
    }

    #[test]
    fn test_tree_hash_vec_push() {
        let mut vec = TreeHashVec::<usize, usize>::new(8);

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
        let mut vec = TreeHashVec::<usize, usize>::new(8);

        vec.push(0, 1, 1);
        vec.push(0, 2, 2);

        assert_eq!(vec.len(), 2);
        assert_eq!(vec.statistics.size, 2);

        assert_eq!(vec.get(0, &1), Some(KeyValue::new(1, 1)));
        assert_eq!(vec.get(0, &2), Some(KeyValue::new(2, 2)));

        vec.update(0, &1, 2);

        assert_eq!(vec.len(), 2);
        assert_eq!(vec.statistics.size, 2);
        assert_eq!(vec.get(0, &1), Some(KeyValue::new(1, 2)));
    }

    #[test]
    fn test_tree_hash_vec_remove_by_index() {
        let mut vec = TreeHashVec::<usize, usize>::new(8);

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
        let mut vec = TreeHashVec::<usize, usize>::new(8);

        vec.push(0, 1, 1);
        vec.push(0, 2, 2);

        assert_eq!(vec.get(0, &1), Some(KeyValue::new(1, 1)));
        assert_eq!(vec.get(0, &2), Some(KeyValue::new(2, 2)));
        assert_eq!(vec.get(0, &3), None);
    }

    #[test]
    fn test_tree_hash_vec_have_key() {
        let mut vec = TreeHashVec::<usize, usize>::new(8);

        vec.push(0, 1, 1);
        vec.push(0, 2, 2);

        assert!(vec.have_key(0, &1));
        assert!(vec.have_key(0, &2));
        assert!(!vec.have_key(0, &3));
    }

    #[test]
    fn test_tree_hash_vec_get_by_index() {
        let mut vec = TreeHashVec::<usize, usize>::new(8);

        vec.push(0, 1, 1);
        vec.push(0, 2, 2);
        
        assert_eq!(vec.get_by_index(0, 0), Some(KeyValue::new(1, 1)));
        assert_eq!(vec.get_by_index(0, 1), Some(KeyValue::new(2, 2)));
        assert_eq!(vec.get_by_index(0, 2), None);
    }

    #[test]
    fn test_tree_hash_vec_find_key() {
        let mut vec = TreeHashVec::<usize, usize>::new(8);

        vec.push(0, 1, 1);
        vec.push(0, 2, 2);

        assert_eq!(vec.find_key(0, &1), Some(0));
        assert_eq!(vec.find_key(0, &2), Some(1));
        assert_eq!(vec.find_key(0, &3), None);
    }

    #[test]
    fn test_tree_hash_get_bucket_len() {
        let mut vec = TreeHashVec::<usize, usize>::new(8);

        vec.push(0, 1, 1);
        vec.push(0, 2, 2);

        assert_eq!(vec.get_bucket_len(0), Some(2));
        assert_eq!(vec.get_bucket_len(1), Some(0));
        assert_eq!(vec.get_bucket_len(9), None);
    }

    #[test]
    fn test_tree_hash_vec_remove() {
        let mut vec = TreeHashVec::<usize, usize>::new(8);

        vec.push(0, 1, 1);
        vec.push(0, 2, 2);

        assert_eq!(vec.statistics.get_count(), 1);
        assert_eq!(vec.statistics.max_length, 2);

        assert_eq!(vec.remove(0, &1), Some(KeyValue::new(1, 1)));
        assert_eq!(vec.find_key(0, &1), None);
        assert!(!vec.have_key(0, &1));
        assert_eq!(vec.statistics.get_count(), 1);
        assert_eq!(vec.statistics.max_length, 1);

        assert_eq!(vec.remove(0, &2), Some(KeyValue::new(2, 2)));
        assert_eq!(vec.find_key(0, &2), None);
        assert!(!vec.have_key(0, &2));
        assert_eq!(vec.statistics.get_count(), 0);
        assert_eq!(vec.statistics.max_length, 0);

        assert_eq!(vec.remove(0, &3), None);

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.statistics.size, 0);

        assert_eq!(vec.data[0].len(), 0);
    }
}
