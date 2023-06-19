use crate::core::structs::hash_table::vectors::hash_vec::{HashVec, HashVecIndexes, HashVecInternal, HashVecStatisticsInternal};
use crate::core::structs::hash_table::vectors::statistics::hash_vec_statistics::HashVecStatistics;
use crate::core::structs::hash_table::vectors::statistics::statistics_functions::{statistics_add_actions, statistics_remove_actions};
use crate::core::structs::tree::object::balanced_tree::balanced_tree::BalancedTree;
use crate::core::structs::tree::object::tree_object::{TreeObject, TreeObjectVec};
use crate::core::structs::tree::vectors::optimized_tree_vec::OptimizedTreeVec;

/// A hash vector that uses a tree to store the values.
/// * `V` - Type of the value
/// * `N` - Number of buckets, must be a power of 2, if it is not, it will be rounded up to the next power of 2
struct TreeHashVec<V: Copy + Default + Copy + Default + PartialOrd, const N: u64> {
    /// The data of the hash vector as a vector of trees.
    /// OptimizedTreeVec is used as the underlying data structure for the trees.
    data: Vec<BalancedTree<V, OptimizedTreeVec<V>>>,
    /// Statistics of the hash vector
    statistics: HashVecStatistics,
}

impl <V: Copy + Default + PartialOrd, const N: u64> TreeHashVec<V, N> {
    /// Creates a new TreeHashVec
    /// # Returns
    /// * `TreeHashVec<V, N>` - New TreeHashVec
    pub fn new() -> TreeHashVec<V, N> {
        let mut vec = TreeHashVec {
            data: Vec::new(),
            statistics: HashVecStatistics::new(N as usize),
        };

        for _ in 0..N {
            let nodes = OptimizedTreeVec::new();
            vec.data.push(BalancedTree::<V, OptimizedTreeVec<V>>::new(nodes));
        }

        vec
    }
}

/// Implementation of basic HashVec trait for TreeHashVec
impl <V: Default + Eq + Copy + Default + PartialOrd, const N: u64> HashVec<V, N> for TreeHashVec<V, N> {
    fn push(&mut self, index: u64, value: V) -> (u64, usize) {
        let data_index = self.data[index as usize].push(value);
        statistics_add_actions(self, index);

        (index, data_index as usize)
    }

    fn have_item(&mut self, index: u64, value: V) -> bool {
        let item_index = self.find_item(index, value);
        match item_index {
            Some(_) => true,
            None => false,
        }
    }

    fn find_item(&mut self, index: u64, value: V) -> Option<usize> {
        let i = self.data[index as usize].find(value);
        match i {
            Some(i) => Some(i as usize),
            None => None,
        }
    }

    fn remove(&mut self, index: u64, value: V) -> Option<V> {
        let item = self.data[index as usize].remove_by_value(value);
        match item {
            Some(_) => {
                statistics_remove_actions(self, index);
                Some(item.unwrap())
            },
            None => None,
        }
    }

    fn len(&self) -> u64 {
        self.statistics.size
    }
}

/// Implementation of HashVecStatisticsInternal trait for TreeHashVec
impl <V: Default + Eq + Copy + Default + PartialOrd, const N: u64> HashVecStatisticsInternal<V, N> for TreeHashVec<V, N> {
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
impl <V: Default + Eq + Copy + Default + PartialOrd, const N: u64> HashVecIndexes<V, N> for TreeHashVec<V, N> {
    fn remove_by_index(&mut self, index: u64, value_index: usize) -> Option<V> {
        let item = self.data[index as usize].remove_by_index(value_index as i32);
        match item {
            Some(_) => {
                statistics_remove_actions(self, index);
                Some(item.unwrap())
            },
            None => None,
        }
    }

    fn get_by_index(&mut self, index: u64, value_index: usize) -> Option<V> {
        self.data[index as usize].get(value_index as i32)
    }
}