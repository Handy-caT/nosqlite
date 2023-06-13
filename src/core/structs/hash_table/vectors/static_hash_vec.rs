use crate::core::structs::hash_table::vectors::hash_vec::{HashVec, HashVecIndexes, HashVecInternal};

/// A static hash table that uses vectors as buckets.
/// # Arguments
/// * `V` - Type of the value
/// * `N` - Number of buckets, must be a power of 2, if it is not, it will be rounded up to the next power of 2
struct StaticHashVec<V, const N: u64> {
    data: Vec<Vec<V>>,
    size: u64,
    max_length: usize,
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
            size: 0,
            max_length: 0,
        }
    }

    pub fn get_vec(&self, index: u64) -> &Vec<V> {
        &self.data[index as usize]
    }

    pub fn get_vec_mut(&mut self, index: u64) -> &mut Vec<V> {
        &mut self.data[index as usize]
    }
}

/// Implementation of basic HashVec trait for StaticHashVec
impl <V: Default + Eq, const N: u64> HashVec<V, N> for StaticHashVec<V, N> {
    fn push(&mut self, index: u64, value: V) -> (u64, usize) {
        self.data[index as usize].push(value);
        let data_index = self.data[index as usize].len() - 1;

        if self.data[index as usize].len() > self.max_length {
            self.max_length = self.data[index as usize].len();
        }
        self.size += 1;

        (index, data_index)
    }

    fn have_item(&self, index: u64, value: V) -> bool {
        let item_index = self.find_item(index, value);
        match item_index {
            Some(_) => true,
            None => false,
        }
    }

    fn find_item(&self, index: u64, value: V) -> Option<usize> {
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
                self.size -= 1;
                Some(self.data[index as usize].swap_remove(i))
              },
              None => None,
         }
    }

    fn len(&self) -> u64 {
        self.size
    }
}

/// Implementation of HashVecIndexes trait for StaticHashVec
impl <V: Default + Eq, const N: u64> HashVecIndexes<V, N> for StaticHashVec<V, N> {
    fn remove_by_index(&mut self, index: u64, value_index: usize) -> Option<V> {
        if value_index >= self.data[index as usize].len() {
            None
        } else {
            self.size -= 1;
            Some(self.data[index as usize].swap_remove(value_index))
        }
    }

    fn get_by_index(&self, index: u64, value_index: usize) -> Option<&V> {
        if value_index >= self.data[index as usize].len() {
            None
        } else {
            Some(&self.data[index as usize][value_index])
        }
    }

    fn get_by_index_mut(&mut self, index: u64, value_index: usize) -> Option<&mut V> {
        if value_index >= self.data[index as usize].len() {
            None
        } else {
            Some(&mut self.data[index as usize][value_index])
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