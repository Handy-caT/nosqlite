use crate::core::structs::hash_table::vectors::{
    hash_vec::{
        HashVec, HashVecIndexes, HashVecInternal, HashVecStatisticsInternal,
    },
    key_value::KeyValue,
    statistics::{
        hash_vec_statistics::HashVecStatistics,
        statistics_functions::{
            statistics_add_actions, statistics_remove_actions,
        },
    },
};

/// A static hash table that uses vectors as buckets.
/// # Arguments
/// * `V` - Type of the value
/// * `N` - Number of buckets, must be a power of 2, if it is not, it will be rounded up to the next power of 2
pub(crate) struct StaticHashVec<K, V, const N: u64> {
    /// The data of the hash vector as a vector of vectors.
    data: Vec<Vec<KeyValue<K, V>>>,
    /// The size of the hash vector. This is the number of buckets.
    /// It is a power of 2. If N is not a power of 2, it will be rounded up to the next power of 2.
    pub size: u64,
    /// Statistics of the hash vector
    statistics: HashVecStatistics,
}

impl<K: Eq, V: Default + Eq, const N: u64> StaticHashVec<K, V, N> {
    /// Creates a new StaticHashVec
    /// # Returns
    /// * `StaticHashVec<V, N>` - New StaticHashVec
    pub fn new() -> Self {
        let mut data = Vec::new();
        let mut i = 0;
        let mut size = N;

        if (N as f64).log2() != (N as f64).log2().floor() {
            let pow = (N as f64).log2().ceil() as u64;
            size = 2u64.pow(pow as u32);
        }

        while i < size {
            data.push(Vec::new());
            i += 1;
        }

        StaticHashVec {
            data,
            size,
            statistics: HashVecStatistics::new(N as usize),
        }
    }
}

/// Implementation of basic HashVec trait for StaticHashVec
impl<K: Eq + Copy + Default, V: Default + Eq + Copy, const N: u64> HashVec<K, V>
    for StaticHashVec<K, V, N>
{
    fn push(&mut self, index: u64, key: K, value: V) -> (u64, usize) {
        let data = KeyValue::new(key, value);

        self.data[index as usize].push(data);
        let data_index = self.data[index as usize].len() - 1;

        statistics_add_actions(self, index);

        (index, data_index)
    }

    fn get(&mut self, index: u64, key: K) -> Option<KeyValue<K, V>> {
        let mut i = 0;
        while i < self.data[index as usize].len() {
            if self.data[index as usize][i].key == key {
                return Some(self.data[index as usize][i]);
            }
            i += 1;
        }
        None
    }

    fn update(
        &mut self,
        index: u64,
        key: K,
        value: V,
    ) -> Option<KeyValue<K, V>> {
        let item_index = self.find_key(index, key);
        match item_index {
            Some(i) => {
                let old_value = self.data[index as usize][i].value;
                self.data[index as usize][i].value = value;
                Some(KeyValue::new(key, old_value))
            }
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
        let item_index = self.find_key(index, key);
        match item_index {
            Some(i) => {
                return self.remove_by_index(index, i);
            }
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

/// Implementation of HashVecIndexes trait for StaticHashVec
impl<K: Eq + Copy + Default, V: Default + Eq + Copy, const N: u64>
    HashVecIndexes<K, V> for StaticHashVec<K, V, N>
{
    fn remove_by_index(
        &mut self,
        index: u64,
        value_index: usize,
    ) -> Option<KeyValue<K, V>> {
        if value_index >= self.data[index as usize].len() {
            None
        } else {
            statistics_remove_actions(self, index);
            Some(self.data[index as usize].swap_remove(value_index))
        }
    }

    fn get_by_index(
        &mut self,
        index: u64,
        value_index: usize,
    ) -> Option<KeyValue<K, V>> {
        if index >= N {
            None
        } else {
            if value_index >= self.data[index as usize].len() {
                None
            } else {
                Some(self.data[index as usize][value_index])
            }
        }
    }

    fn find_key(&mut self, index: u64, key: K) -> Option<usize> {
        let mut i = 0;
        while i < self.data[index as usize].len() {
            if self.data[index as usize][i].key == key {
                return Some(i);
            }
            i += 1;
        }
        None
    }
}

impl<K: Eq + Default, V: Default + Eq, const N: u64> HashVecInternal<K, V>
    for StaticHashVec<K, V, N>
{
    fn get_vec(&self, index: u64) -> Option<&Vec<KeyValue<K, V>>> {
        if index >= N {
            None
        } else {
            Some(&self.data[index as usize])
        }
    }

    fn get_vec_mut(&mut self, index: u64) -> Option<&mut Vec<KeyValue<K, V>>> {
        if index >= N {
            None
        } else {
            Some(&mut self.data[index as usize])
        }
    }
}

impl<K: Eq, V: Default + Eq, const N: u64> HashVecStatisticsInternal<K, V>
    for StaticHashVec<K, V, N>
{
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

#[cfg(test)]
mod tests {
    use crate::core::structs::hash_table::vectors::{
        hash_vec::{
            HashVec, HashVecIndexes, HashVecInternal, HashVecStatisticsInternal,
        },
        key_value::KeyValue,
        static_hash_vec::StaticHashVec,
    };

    #[test]
    fn test_static_hash_vec_new() {
        let hash_vec: StaticHashVec<u64, u64, 8> = StaticHashVec::new();

        assert_eq!(hash_vec.len(), 0);
        assert_eq!(hash_vec.statistics.size, 0);

        assert_eq!(hash_vec.data.len(), 8);
    }

    #[test]
    fn test_static_hash_vec_new_sizes() {
        let hash_vec: StaticHashVec<u64, u64, 10> = StaticHashVec::new();

        assert_eq!(hash_vec.len(), 0);
        assert_eq!(hash_vec.data.len(), 16);
        assert_eq!(hash_vec.size, 16);
        assert_eq!(hash_vec.size(), 16);

        let hash_vec: StaticHashVec<u64, u64, 32> = StaticHashVec::new();

        assert_eq!(hash_vec.len(), 0);
        assert_eq!(hash_vec.data.len(), 32);
        assert_eq!(hash_vec.size, 32);
        assert_eq!(hash_vec.size(), 32);
    }

    #[test]
    fn test_static_hash_vec_update() {
        let mut hash_vec: StaticHashVec<u64, u64, 8> = StaticHashVec::new();

        hash_vec.push(0, 1, 1);
        hash_vec.push(0, 2, 2);

        assert_eq!(hash_vec.len(), 2);
        assert_eq!(hash_vec.statistics.size, 2);

        assert_eq!(hash_vec.get(0, 1), Some(KeyValue::new(1, 1)));
        assert_eq!(hash_vec.get(0, 2), Some(KeyValue::new(2, 2)));

        hash_vec.update(0, 1, 3);

        assert_eq!(hash_vec.len(), 2);
        assert_eq!(hash_vec.get(0, 1).unwrap().value, 3);
    }

    #[test]
    fn test_static_hash_vec_push() {
        let mut hash_vec: StaticHashVec<u64, u64, 8> = StaticHashVec::new();

        hash_vec.push(0, 1, 1);
        hash_vec.push(0, 2, 2);

        assert_eq!(hash_vec.len(), 2);
        assert_eq!(hash_vec.statistics.size, 2);

        assert_eq!(hash_vec.data[0].len(), 2);

        assert_eq!(hash_vec.data[0][0].key, 1);
        assert_eq!(hash_vec.data[0][0].value, 1);
        assert_eq!(hash_vec.data[0][1].key, 2);
        assert_eq!(hash_vec.data[0][1].value, 2);

        assert_eq!(hash_vec.statistics.get_count(), 1);
        assert_eq!(hash_vec.statistics.max_length, 2);
    }

    #[test]
    fn test_static_hash_vec_have_key() {
        let mut hash_vec: StaticHashVec<u64, u64, 8> = StaticHashVec::new();

        hash_vec.push(0, 1, 1);
        hash_vec.push(0, 2, 2);

        assert_eq!(hash_vec.have_key(0, 1), true);
        assert_eq!(hash_vec.have_key(0, 2), true);
        assert_eq!(hash_vec.have_key(0, 3), false);
    }

    #[test]
    fn test_static_hash_vec_find_key() {
        let mut hash_vec: StaticHashVec<u64, u64, 8> = StaticHashVec::new();

        hash_vec.push(0, 1, 1);
        hash_vec.push(0, 2, 2);

        assert_eq!(hash_vec.find_key(0, 1), Some(0));
        assert_eq!(hash_vec.find_key(0, 2), Some(1));
        assert_eq!(hash_vec.find_key(0, 3), None);
    }

    #[test]
    fn test_static_hash_vec_remove() {
        let mut hash_vec: StaticHashVec<u64, u64, 8> = StaticHashVec::new();

        hash_vec.push(0, 1, 1);
        hash_vec.push(0, 2, 2);

        assert_eq!(hash_vec.statistics.get_count(), 1);
        assert_eq!(hash_vec.statistics.max_length, 2);

        assert_eq!(hash_vec.remove(0, 1), Some(KeyValue::new(1, 1)));
        assert_eq!(hash_vec.find_key(0, 1), None);
        assert_eq!(hash_vec.have_key(0, 1), false);
        assert_eq!(hash_vec.statistics.get_count(), 1);
        assert_eq!(hash_vec.statistics.max_length, 1);

        assert_eq!(hash_vec.remove(0, 2), Some(KeyValue::new(2, 2)));
        assert_eq!(hash_vec.find_key(0, 2), None);
        assert_eq!(hash_vec.have_key(0, 2), false);
        assert_eq!(hash_vec.statistics.get_count(), 0);
        assert_eq!(hash_vec.statistics.max_length, 0);

        assert_eq!(hash_vec.remove(0, 3), None);

        assert_eq!(hash_vec.len(), 0);
        assert_eq!(hash_vec.statistics.size, 0);

        assert_eq!(hash_vec.data[0].len(), 0);
    }

    #[test]
    fn test_static_hash_vec_remove_by_index() {
        let mut hash_vec: StaticHashVec<u64, u64, 8> = StaticHashVec::new();

        hash_vec.push(0, 1, 1);
        hash_vec.push(0, 2, 2);

        assert_eq!(hash_vec.remove_by_index(0, 0), Some(KeyValue::new(1, 1)));
        assert_eq!(hash_vec.find_key(0, 1), None);
        assert_eq!(hash_vec.have_key(0, 1), false);
        assert_eq!(hash_vec.statistics.get_count(), 1);
        assert_eq!(hash_vec.statistics.max_length, 1);

        assert_eq!(hash_vec.remove_by_index(0, 0), Some(KeyValue::new(2, 2)));
        assert_eq!(hash_vec.find_key(0, 2), None);
        assert_eq!(hash_vec.have_key(0, 2), false);
        assert_eq!(hash_vec.statistics.get_count(), 0);
        assert_eq!(hash_vec.statistics.max_length, 0);

        assert_eq!(hash_vec.remove_by_index(0, 0), None);

        assert_eq!(hash_vec.len(), 0);
        assert_eq!(hash_vec.statistics.size, 0);

        assert_eq!(hash_vec.data[0].len(), 0);
    }

    #[test]
    fn test_static_hash_vec_get_by_index() {
        let mut hash_vec: StaticHashVec<u64, u64, 8> = StaticHashVec::new();

        hash_vec.push(0, 1, 1);
        hash_vec.push(0, 2, 2);

        assert_eq!(hash_vec.get_by_index(0, 0), Some(KeyValue::new(1, 1)));
        assert_eq!(hash_vec.get_by_index(0, 1), Some(KeyValue::new(2, 2)));
        assert_eq!(hash_vec.get_by_index(0, 2), None);
    }

    #[test]
    fn test_static_hash_vec_get_bucket_len() {
        let mut hash_vec: StaticHashVec<u64, u64, 8> = StaticHashVec::new();

        hash_vec.push(0, 1, 1);
        hash_vec.push(0, 2, 2);

        assert_eq!(hash_vec.get_bucket_len(0), Some(2));
        assert_eq!(hash_vec.get_bucket_len(1), Some(0));
        assert_eq!(hash_vec.get_bucket_len(9), None);
    }

    #[test]
    fn test_static_hash_vec_get_vec() {
        let mut hash_vec: StaticHashVec<u64, u64, 8> = StaticHashVec::new();

        hash_vec.push(0, 1, 1);
        hash_vec.push(0, 2, 2);

        let vec = hash_vec.get_vec(0);

        assert_eq!(vec.is_some(), true);
        assert_eq!(vec.unwrap().len(), 2);

        let vec = hash_vec.get_vec(9);

        assert_eq!(vec.is_some(), false);
    }

    #[test]
    fn test_static_hash_vec_get_statistics() {
        let mut hash_vec: StaticHashVec<u64, u64, 8> = StaticHashVec::new();

        hash_vec.push(0, 1, 1);
        hash_vec.push(0, 2, 2);

        let statistics = hash_vec.get_statistics();

        assert_eq!(statistics.get_count(), 1);
        assert_eq!(statistics.max_length, 2);
        assert_eq!(hash_vec.get_max_len(), 2);
    }
}
