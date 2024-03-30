use crate::structs::tree::{
    nodes::{tree_index::TreeIndex, tree_node::TreeNode},
    vectors::{
        optimized_tree_vec::INITIAL_LEVELS,
        tree_vec::{
            Backward, DefaultFunctions, Levels, OptimizedFunctions, TreeVec,
        },
        vec_functions::{allocate_level, get, push, remove},
    },
};
use std::ops::{Index, IndexMut};

/// Struct that represents a vector that stores tree nodes that also
/// stores the parents of the nodes.
/// # Type parameters
/// * `T` - Type of the data that the vector stores
#[derive(Debug)]
pub struct BackwardsTreeVec<T> {
    /// Number of allocated levels.
    allocated_levels: u8,

    /// Maximum length of the vector.
    max_length: usize,

    /// Length of the vector.
    length: usize,

    /// Vector that stores the data.
    data: Vec<T>,

    /// Vector that stores the indexes of the nodes.
    indexes: Vec<TreeIndex>,

    /// Vector that stores the empty indexes.
    empty: Vec<usize>,

    /// Vector that stores the parents of the nodes.
    parents: Vec<usize>,
}

impl<T: Default + Clone> Levels for BackwardsTreeVec<T> {
    fn get_allocated_levels(&self) -> u8 {
        self.allocated_levels
    }

    fn get_max_length(&self) -> usize {
        self.max_length
    }
}

impl<T: Default + Clone> DefaultFunctions<T> for BackwardsTreeVec<T> {
    fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    fn get_data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    fn get_empty(&self) -> &Vec<usize> {
        &self.empty
    }

    fn get_empty_mut(&mut self) -> &mut Vec<usize> {
        &mut self.empty
    }

    fn get_indexes(&self) -> &Vec<TreeIndex> {
        &self.indexes
    }

    fn get_indexes_mut(&mut self) -> &mut Vec<TreeIndex> {
        &mut self.indexes
    }
}

impl<T: Default + Clone> OptimizedFunctions<T> for BackwardsTreeVec<T> {
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
        allocate_level(self);
    }
}

impl<T: Default + Clone> TreeVec<T> for BackwardsTreeVec<T> {
    fn new() -> BackwardsTreeVec<T> {
        let mut vec = BackwardsTreeVec {
            allocated_levels: 0,
            max_length: 0,
            length: 0,
            data: Vec::new(),
            indexes: Vec::new(),
            empty: Vec::new(),
            parents: Vec::new(),
        };

        let length = 2usize.pow(u32::from(INITIAL_LEVELS)) - 1;

        vec.data.reserve(length);
        vec.indexes.reserve(length);

        vec.max_length = length;
        vec.allocated_levels = INITIAL_LEVELS;

        vec
    }

    fn push(&mut self, value: T) -> usize {
        let index = push(self, value);
        if index == self.length - 1 {
            self.parents.push(0);
        }

        index
    }

    fn get(&self, index: usize) -> Option<TreeNode<T>> {
        get(self, index)
    }

    fn get_value_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.length {
            Some(&mut self.data[index])
        } else {
            None
        }
    }

    fn remove(&mut self, index: usize) -> Option<TreeNode<T>> {
        remove(self, index)
    }

    fn len(&self) -> usize {
        self.length
    }
}

impl<T: Default + Clone> Index<usize> for BackwardsTreeVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.data[index]
    }
}

impl<T: Default + Clone> IndexMut<usize> for BackwardsTreeVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.data[index]
    }
}

impl<T: Default + Clone> Backward for BackwardsTreeVec<T> {
    fn get_parent(&self, index: usize) -> Option<usize> {
        if index == 0 {
            return None;
        }

        let parent = self.parents[index];

        if parent == 0 {
            return None;
        }

        Some(parent)
    }

    fn add_parent(&mut self, index: usize, parent: usize) -> Option<()> {
        if index < self.length {
            self.parents[index] = parent;
            Some(())
        } else {
            None
        }
    }
}

impl<T: Clone> Clone for BackwardsTreeVec<T> {
    fn clone(&self) -> Self {
        let mut vec = BackwardsTreeVec {
            allocated_levels: self.allocated_levels,
            max_length: self.max_length,
            length: self.length,
            data: self.data.clone(),
            indexes: self.indexes.clone(),
            empty: self.empty.clone(),
            parents: self.parents.clone(),
        };

        vec
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::tree::vectors::{
        backward_tree_vec::BackwardsTreeVec,
        optimized_tree_vec::INITIAL_LEVELS,
        tree_vec::{DefaultFunctions, Levels, TreeVec},
    };

    #[test]
    fn test_backwards_tree_vec_new() {
        let vec = BackwardsTreeVec::<i32>::new();

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.get_allocated_levels(), INITIAL_LEVELS);
        assert_eq!(
            vec.get_max_length(),
            2usize.pow(u32::from(INITIAL_LEVELS)) - 1
        );
        assert_eq!(vec.get_data().len(), 0);
        assert_eq!(vec.get_indexes().len(), 0);
        assert_eq!(vec.get_empty().len(), 0);
        assert_eq!(vec.parents.len(), 0);
    }

    #[test]
    fn test_backwards_tree_vec_push() {
        let mut vec = BackwardsTreeVec::<i32>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec.get_allocated_levels(), INITIAL_LEVELS);
        assert_eq!(
            vec.get_max_length(),
            2usize.pow(u32::from(INITIAL_LEVELS)) - 1
        );
        assert_eq!(vec.get_data().len(), 3);
        assert_eq!(vec.get_indexes().len(), 3);
        assert_eq!(vec.get_empty().len(), 0);
        assert_eq!(vec.parents.len(), 3);
    }

    #[test]
    fn test_backwards_tree_vec_remove() {
        let mut vec = BackwardsTreeVec::<i32>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.len(), 3);

        let node = vec.remove(1).unwrap();

        assert_eq!(node.value, 2);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec.get_data().len(), 3);
        assert_eq!(vec.get_indexes().len(), 3);
        assert_eq!(vec.get_empty().len(), 1);
        assert_eq!(vec.parents.len(), 3);
    }
}
