use crate::structs::tree::{
    nodes::{
        normalized_tree_index::NormalizedTreeIndex, tree_index::TreeIndex,
        tree_node::TreeNode,
    },
    vectors::{
        optimized_tree_vec::INITIAL_LEVELS,
        tree_vec::{Levels, NormalizedIndexes, OptimizedFunctions, TreeVec},
    },
};
use std::ops::{Index, IndexMut};

/// Struct that represents normalized tree vector.
/// In this vector child indexes are 2i+1 and 2i+2.
/// This vector has empty vector to contain empty indexes.
/// Indexes vector contains indexes of data vector,
/// so data is independent from indexes.
/// # Type parameters
/// * `T` - Type of the data that the vector stores.
#[derive(Debug)]
pub struct NormalizedTreeVector<T> {
    /// Number of allocated levels.
    allocated_levels: u8,

    /// Maximum length of the vector before allocation.
    max_length: usize,

    /// Length of the vector.
    length: usize,

    /// Vector that stores the data.
    data: Vec<T>,

    /// Vector that stores the indexes of the data vector.
    indexes: Vec<NormalizedTreeIndex>,

    /// Vector that stores the indexes of the empty spaces.
    empty: Vec<usize>,
}

/// [`NormalizedTreeVector`] implementation.
impl<T: Default + Clone> NormalizedTreeVector<T> {
    /// Swaps two indexes.
    /// Indexes must be in bounds.
    /// # Arguments
    /// * `index1` - First index.
    /// * `index2` - Second index.
    pub fn swap_indexes(&mut self, index1: usize, index2: usize) {
        let index1_new = self.indexes[index1].index;
        let index2_new = self.indexes[index2].index;

        self.indexes[index1].index = index2_new;
        self.indexes[index2].index = index1_new;
    }

    /// Function to get parent index of given index.
    /// Index must be more than 0 to have parent.
    /// # Arguments
    /// * `index` - Index to get parent index.
    /// # Returns
    /// Parent index of given index.
    #[must_use]
    pub fn get_parent_index(index: usize) -> Option<usize> {
        if index == 0 {
            None
        } else {
            let parent_index = (index - 1) / 2;
            Some(parent_index)
        }
    }
}

/// [`Levels`] implementation for [`NormalizedTreeVector`].
impl<T> Levels for NormalizedTreeVector<T> {
    fn get_allocated_levels(&self) -> u8 {
        self.allocated_levels
    }

    fn get_max_length(&self) -> usize {
        self.max_length
    }
}

impl<T: Default + Clone> OptimizedFunctions<T> for NormalizedTreeVector<T> {
    fn get_allocated_levels_mut(&mut self) -> &mut u8 {
        &mut self.allocated_levels
    }

    fn get_max_length_mut(&mut self) -> &mut usize {
        &mut self.max_length
    }

    fn get_length(&self) -> usize {
        self.length
    }

    fn get_length_mut(&mut self) -> &mut usize {
        &mut self.length
    }

    fn allocate_level(&mut self) {
        let length = 2usize.pow(u32::from(self.allocated_levels)) - 1;

        self.data.reserve(length);
        self.indexes.reserve(length);

        self.max_length += length;
        self.allocated_levels += 1;
    }
}

impl<T: Default + Clone> TreeVec<T> for NormalizedTreeVector<T> {
    fn new() -> NormalizedTreeVector<T> {
        let mut vec = NormalizedTreeVector {
            allocated_levels: 0,
            max_length: 0,
            length: 0,
            data: Vec::new(),
            indexes: Vec::new(),
            empty: Vec::new(),
        };

        let length = 2usize.pow(u32::from(INITIAL_LEVELS)) - 1;

        vec.data.reserve(length);
        vec.indexes.reserve(length);

        vec.max_length = length;
        vec.allocated_levels = INITIAL_LEVELS;

        vec
    }

    fn push(&mut self, value: T) -> usize {
        let index = self.length;

        let data_index = if self.empty.is_empty() {
            index
        } else {
            self.empty.pop().unwrap()
        };

        if index == self.max_length {
            self.allocate_level();
        }

        if data_index == index {
            self.data.push(value);
        } else {
            self.data[data_index] = value;
        }

        let tree_index = NormalizedTreeIndex {
            index: Some(data_index),
            height: NormalizedTreeIndex::find_height(index),
        };
        self.indexes.push(tree_index);
        self.length += 1;

        index
    }

    fn get(&self, index: usize) -> Option<TreeNode<T>> {
        if index >= self.length {
            None
        } else {
            let tree_index = TreeIndex {
                index: self.indexes[index].index,
                left_index: Some(2 * index + 1),
                right_index: Some(2 * index + 2),
                height: self.indexes[index].height,
            };
            let data = self.data[tree_index.index.unwrap()].clone();

            let node = TreeNode {
                value: data,
                indexes: tree_index,
            };

            Some(node)
        }
    }

    fn get_value_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.length {
            Some(&mut self.data[index])
        } else {
            None
        }
    }

    fn remove(&mut self, index: usize) -> Option<TreeNode<T>> {
        if index >= self.length {
            None
        } else if index == self.length - 1 {
            let item = self.indexes.pop().unwrap();
            // We can unwrap because index is always Some
            let data_index = item.index.unwrap();
            let height = item.height;

            let data = self.data[data_index].clone();
            if data_index == index {
                self.data.pop();
            } else {
                self.empty.push(data_index);
            }

            self.length -= 1;

            let tree_index = TreeIndex {
                index: Some(data_index),
                left_index: Some(2 * index + 1),
                right_index: Some(2 * index + 2),
                height,
            };

            let node = TreeNode {
                value: data,
                indexes: tree_index,
            };

            Some(node)
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.length
    }
}

impl<T: Default + Clone> NormalizedIndexes<T> for NormalizedTreeVector<T> {
    fn get_index_mut(&mut self, index: usize) -> &mut NormalizedTreeIndex {
        &mut self.indexes[index]
    }

    fn get_index(&self, index: usize) -> &NormalizedTreeIndex {
        &self.indexes[index]
    }

    fn get_indexes(&mut self) -> &mut Vec<NormalizedTreeIndex> {
        &mut self.indexes
    }
}

impl<T: Default + Clone> Index<usize> for NormalizedTreeVector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Default + Clone> IndexMut<usize> for NormalizedTreeVector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Clone> Clone for NormalizedTreeVector<T> {
    fn clone(&self) -> Self {
        NormalizedTreeVector {
            allocated_levels: self.allocated_levels,
            max_length: self.max_length,
            length: self.length,
            data: self.data.clone(),
            indexes: self.indexes.clone(),
            empty: self.empty.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::tree::vectors::{
        normalized_tree_vec::NormalizedTreeVector,
        optimized_tree_vec::INITIAL_LEVELS,
        tree_vec::{Levels, TreeVec},
    };

    #[test]
    fn test_normalized_tree_vector_new() {
        let vec = NormalizedTreeVector::<u64>::new();

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.get_allocated_levels(), INITIAL_LEVELS);
        assert_eq!(vec.get_max_length(), 2usize.pow(INITIAL_LEVELS as u32) - 1);
    }

    #[test]
    fn test_normalized_tree_vector_push() {
        let mut vec = NormalizedTreeVector::<u64>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);

        assert_eq!(vec.get_allocated_levels(), INITIAL_LEVELS);
        assert_eq!(vec.get_max_length(), 2usize.pow(INITIAL_LEVELS as u32) - 1);
        assert_eq!(vec.indexes[0].height, 1);
        assert_eq!(vec.indexes[2].height, 2);
    }

    #[test]
    fn test_normalized_tree_vector_get() {
        let mut vec = NormalizedTreeVector::<u64>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.len(), 3);

        assert!(vec.get(0).is_some());
        assert_eq!(vec.get(0).unwrap().value, 1)
    }

    #[test]
    fn test_normalized_tree_vector_swap_indexes() {
        let mut vec = NormalizedTreeVector::<u64>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.len(), 3);

        vec.swap_indexes(0, 2);

        assert_eq!(vec.get(0).unwrap().indexes.index, Some(2));
        assert_eq!(vec.get(2).unwrap().indexes.index, Some(0));

        assert_eq!(vec.get(0).unwrap().indexes.height, 1);
        assert_eq!(vec.get(0).unwrap().value, 3);
        assert_eq!(vec.get(2).unwrap().indexes.height, 2);
        assert_eq!(vec.get(2).unwrap().value, 1);
    }

    #[test]
    fn test_normalized_tree_vector_get_parent() {
        assert_eq!(NormalizedTreeVector::<u64>::get_parent_index(0), None);
        assert_eq!(NormalizedTreeVector::<u64>::get_parent_index(1), Some(0));
        assert_eq!(NormalizedTreeVector::<u64>::get_parent_index(5), Some(2));
    }

    #[test]
    fn test_normalized_tree_vector_remove() {
        let mut vec = NormalizedTreeVector::<u64>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.remove(1), None);
        assert_eq!(vec.len(), 3);

        let node = vec.remove(2);
        assert!(node.is_some());
        assert_eq!(node.unwrap().value, 3);
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.empty.len(), 0);
    }

    #[test]
    fn test_normalized_tree_vector_remove_swap_push() {
        let mut vec = NormalizedTreeVector::<u64>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.len(), 3);

        vec.swap_indexes(0, 2);
        let node = vec.remove(2);
        assert!(node.is_some());

        assert_eq!(node.unwrap().value, 1);
        assert_eq!(vec.empty.len(), 1);

        vec.push(4);
        let node = vec.get(2);
        assert!(node.is_some());

        let node = node.unwrap();
        assert_eq!(node.value, 4);
        assert_eq!(node.indexes.index, Some(0));
        assert_eq!(vec.empty.len(), 0);
    }
}
