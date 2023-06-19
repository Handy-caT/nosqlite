use crate::core::structs::hash_table::vectors::hash_vec::{HashVec, HashVecIndexes, HashVecInternal, HashVecStatisticsInternal};
use crate::core::structs::hash_table::vectors::statistics::hash_vec_statistics::HashVecStatistics;
use crate::core::structs::hash_table::vectors::statistics::statistics_functions::{statistics_add_actions, statistics_remove_actions};

/// A static hash table that uses vectors as buckets.
/// # Arguments
/// * `V` - Type of the value
/// * `N` - Number of buckets, must be a power of 2, if it is not, it will be rounded up to the next power of 2
struct StaticHashVec<V, const N: u64> {
    data: Vec<Vec<V>>,
    statistics: HashVecStatistics,
}

impl <V: Default + Eq, const N: u64> StaticHashVec<V, N> {
    /// Creates a new StaticHashVec
    /// # Returns
    /// * `StaticHashVec<V, N>` - New StaticHashVec
    pub fn new() -> Self {
        let mut data = Vec::new();
        let mut i = 0;
        while i < N {
            data.push(Vec::new());
            i+=1;
        }
        StaticHashVec {
            data,
            statistics: HashVecStatistics::new(N as usize)
        }
    }
}

/// Implementation of basic HashVec trait for StaticHashVec
impl <V: Default + Eq + Copy, const N: u64> HashVec<V, N> for StaticHashVec<V, N> {
    fn push(&mut self, index: u64, value: V) -> (u64, usize) {
        self.data[index as usize].push(value);
        let data_index = self.data[index as usize].len() - 1;

        statistics_add_actions(self, index);

        (index, data_index)
    }

    fn have_item(&mut self, index: u64, value: V) -> bool {
        let item_index = self.find_item(index, value);
        match item_index {
            Some(_) => true,
            None => false,
        }
    }

    fn find_item(&mut self, index: u64, value: V) -> Option<usize> {
        let mut i = 0;
        while i < self.data[index as usize].len() {
            if self.data[index as usize][i] == value {
                return Some(i);
            }
            i+=1;
        }
        None
    }

    fn remove(&mut self, index: u64, value: V) -> Option<V> {
       let item_index = self.find_item(index, value);
         match item_index {
              Some(i) => {
                  return self.remove_by_index(index, i);
              },
              None => None,
         }
    }

    fn len(&self) -> u64 {
        self.statistics.size
    }
}

/// Implementation of HashVecIndexes trait for StaticHashVec
impl <V: Default + Eq + Copy, const N: u64> HashVecIndexes<V, N> for StaticHashVec<V, N> {
    fn remove_by_index(&mut self, index: u64, value_index: usize) -> Option<V> {
        if value_index >= self.data[index as usize].len() {
            None
        } else {
            statistics_remove_actions(self, index);
            Some(self.data[index as usize].swap_remove(value_index))
        }
    }

    fn get_by_index(&mut self, index: u64, value_index: usize) -> Option<V> {
        if value_index >= self.data[index as usize].len() {
            None
        } else {
            Some(self.data[index as usize][value_index])
        }
    }
}

impl <V: Default + Eq, const N: u64> HashVecInternal<V, N> for StaticHashVec<V, N> {
    fn get_vec(&self, index: u64) -> Option<&Vec<V>> {
        if  index >= N {
            None
        } else {
            Some(&self.data[index as usize])
        }
    }

    fn get_vec_mut(&mut self, index: u64) -> Option<&mut Vec<V>> {
        if  index >= N {
            None
        } else {
            Some(&mut self.data[index as usize])
        }
    }
}

impl <V: Default + Eq, const N: u64> HashVecStatisticsInternal<V, N> for StaticHashVec<V, N> {
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
    use crate::core::structs::hash_table::vectors::static_hash_vec::StaticHashVec;
    use crate::core::structs::hash_table::vectors::hash_vec::{HashVec, HashVecIndexes, HashVecInternal, HashVecStatisticsInternal};

    #[test]
    fn test_static_hash_vec_new() {
        let hash_vec: StaticHashVec<u64, 8> = StaticHashVec::new();

        assert_eq!(hash_vec.len(), 0);
        assert_eq!(hash_vec.statistics.size, 0);

        assert_eq!(hash_vec.data.len(), 8);
    }

    #[test]
    fn test_static_hash_vec_push() {
        let mut hash_vec: StaticHashVec<u64, 8> = StaticHashVec::new();

        hash_vec.push(0, 1);
        hash_vec.push(0, 2);

        assert_eq!(hash_vec.len(), 2);
        assert_eq!(hash_vec.statistics.size, 2);

        assert_eq!(hash_vec.data[0].len(), 2);
        assert_eq!(hash_vec.data[0][0], 1);
        assert_eq!(hash_vec.data[0][1], 2);

        assert_eq!(hash_vec.statistics.get_count(), 1);
        assert_eq!(hash_vec.statistics.max_length, 2);
    }

    #[test]
    fn test_static_hash_vec_have_item() {
        let mut hash_vec: StaticHashVec<u64, 8> = StaticHashVec::new();

        hash_vec.push(0, 1);
        hash_vec.push(0, 2);

        assert_eq!(hash_vec.have_item(0, 1), true);
        assert_eq!(hash_vec.have_item(0, 2), true);
        assert_eq!(hash_vec.have_item(0, 3), false);
    }

    #[test]
    fn test_static_hash_vec_find_item() {
        let mut hash_vec: StaticHashVec<u64, 8> = StaticHashVec::new();

        hash_vec.push(0, 1);
        hash_vec.push(0, 2);

        assert_eq!(hash_vec.find_item(0, 1), Some(0));
        assert_eq!(hash_vec.find_item(0, 2), Some(1));
        assert_eq!(hash_vec.find_item(0, 3), None);
    }

    #[test]
    fn test_static_hash_vec_remove() {
        let mut hash_vec: StaticHashVec<u64, 8> = StaticHashVec::new();

        hash_vec.push(0, 1);
        hash_vec.push(0, 2);

        assert_eq!(hash_vec.statistics.get_count(), 1);
        assert_eq!(hash_vec.statistics.max_length, 2);

        assert_eq!(hash_vec.remove(0, 1), Some(1));
        assert_eq!(hash_vec.find_item(0, 1), None);
        assert_eq!(hash_vec.have_item(0, 1), false);
        assert_eq!(hash_vec.statistics.get_count(), 1);
        assert_eq!(hash_vec.statistics.max_length, 1);

        assert_eq!(hash_vec.remove(0, 2), Some(2));
        assert_eq!(hash_vec.find_item(0, 2), None);
        assert_eq!(hash_vec.have_item(0, 2), false);
        assert_eq!(hash_vec.statistics.get_count(), 0);
        assert_eq!(hash_vec.statistics.max_length, 0);

        assert_eq!(hash_vec.remove(0, 3), None);

        assert_eq!(hash_vec.len(), 0);
        assert_eq!(hash_vec.statistics.size, 0);

        assert_eq!(hash_vec.data[0].len(), 0);
    }

    #[test]
    fn test_static_hash_vec_remove_by_index() {
        let mut hash_vec: StaticHashVec<u64, 8> = StaticHashVec::new();

        hash_vec.push(0, 1);
        hash_vec.push(0, 2);

        assert_eq!(hash_vec.remove_by_index(0, 0), Some(1));
        assert_eq!(hash_vec.find_item(0, 1), None);
        assert_eq!(hash_vec.have_item(0, 1), false);
        assert_eq!(hash_vec.statistics.get_count(), 1);
        assert_eq!(hash_vec.statistics.max_length, 1);

        assert_eq!(hash_vec.remove_by_index(0, 0), Some(2));
        assert_eq!(hash_vec.find_item(0, 2), None);
        assert_eq!(hash_vec.have_item(0, 2), false);
        assert_eq!(hash_vec.statistics.get_count(), 0);
        assert_eq!(hash_vec.statistics.max_length, 0);

        assert_eq!(hash_vec.remove_by_index(0, 0), None);

        assert_eq!(hash_vec.len(), 0);
        assert_eq!(hash_vec.statistics.size, 0);

        assert_eq!(hash_vec.data[0].len(), 0);
    }

    #[test]
    fn test_static_hash_vec_get_by_index() {
        let mut hash_vec: StaticHashVec<u64, 8> = StaticHashVec::new();

        hash_vec.push(0, 1);
        hash_vec.push(0, 2);

        assert_eq!(hash_vec.get_by_index(0, 0), Some(1));
        assert_eq!(hash_vec.get_by_index(0, 1), Some(2));
        assert_eq!(hash_vec.get_by_index(0, 2), None);
    }

    #[test]
    fn test_static_hash_vec_get_vec() {
        let mut hash_vec: StaticHashVec<u64, 8> = StaticHashVec::new();

        hash_vec.push(0, 1);
        hash_vec.push(0, 2);

        let vec = hash_vec.get_vec(0);

        assert_eq!(vec.is_some(), true);
        assert_eq!(vec.unwrap().len(), 2);

        let vec = hash_vec.get_vec(9);

        assert_eq!(vec.is_some(), false);
    }

    #[test]
    fn test_static_hash_vec_get_statistics() {
        let mut hash_vec: StaticHashVec<u64, 8> = StaticHashVec::new();

        hash_vec.push(0, 1);
        hash_vec.push(0, 2);

        let statistics = hash_vec.get_statistics();

        assert_eq!(statistics.get_count(), 1);
        assert_eq!(statistics.max_length, 2);
        assert_eq!(hash_vec.get_max_len(), 2);
    }
}