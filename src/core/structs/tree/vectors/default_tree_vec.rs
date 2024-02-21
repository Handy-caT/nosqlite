use crate::core::structs::tree::{
    nodes::{TreeIndex, TreeNode},
    vectors::tree_vec::{DefaultFunctions, Indexes, Levels, TreeVec},
};
use std::ops::{Index, IndexMut};

/// Struct that represents a vector that stores tree nodes.
/// It has empty spaces that can be filled.
/// # Type parameters
/// * `T` - Type of the data that the vector stores
pub struct DefaultTreeVec<T: Sized> {
    /// Vector that stores the data
    data: Vec<T>,

    /// Vector that stores the empty indexes
    empty: Vec<usize>,

    /// Vector that stores the indexes of the nodes
    indexes: Vec<TreeIndex>,

    /// Length of the vector
    length: usize,
}

impl<T: Default + Clone> Indexes<T> for DefaultTreeVec<T> {
    fn get_index_mut(&mut self, index: usize) -> &mut TreeIndex {
        &mut self.indexes[index]
    }

    fn get_index(&self, index: usize) -> &TreeIndex {
        &self.indexes[index]
    }

    fn get_indexes(&mut self) -> &mut Vec<TreeIndex> {
        &mut self.indexes
    }
}

impl<T: Default + Clone> TreeVec<T> for DefaultTreeVec<T> {
    fn new() -> DefaultTreeVec<T> {
        DefaultTreeVec {
            data: Vec::new(),
            empty: Vec::new(),
            indexes: Vec::new(),
            length: 0,
        }
    }

    fn push(&mut self, value: T) -> usize {
        let index = if self.empty.is_empty() {
            self.length += 1;
            self.data.len()
        } else {
            self.empty.pop().unwrap()
        };

        let indexes = TreeIndex::new_with_index(index);

        if index == self.data.len() {
            self.data.push(value);
            self.indexes.push(indexes);
        } else {
            self.data[index] = value;
            self.indexes[index] = indexes;
        }

        index
    }

    fn get(&self, index: usize) -> Option<TreeNode<T>> {
        let item = self.indexes.get(index);
        return if let Some(item) = item {
            if item.index.is_none() {
                None
            } else {
                let value = self.data.get(index);
                Some(TreeNode {
                    value: value.unwrap().clone(),
                    indexes: *item,
                })
            }
        } else {
            None
        };
    }

    fn get_value_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.length {
            Some(&mut self.data[index])
        } else {
            None
        }
    }

    fn remove(&mut self, index: usize) -> Option<TreeNode<T>> {
        let item = *self.indexes.get(index)?;

        item.index?;

        self.indexes[index] = TreeIndex::default();
        self.empty.push(index);

        if index == self.length - 1 {
            self.length -= 1;
        }

        let value = self.data.get(index);

        Some(TreeNode {
            value: value.unwrap().clone(),
            indexes: item,
        })
    }

    fn len(&self) -> usize {
        self.length
    }
}

impl<T: Default + Clone> Index<usize> for DefaultTreeVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Default + Clone> IndexMut<usize> for DefaultTreeVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Default + Clone> DefaultFunctions<T> for DefaultTreeVec<T> {
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

impl<T> Levels for DefaultTreeVec<T> {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn get_allocated_levels(&self) -> u8 {
        let length = f64::from(self.length as u32);
        length.log2().ceil() as u8
    }

    fn get_max_length(&self) -> usize {
        let levels = self.get_allocated_levels();
        2usize.pow(u32::from(levels))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_vec_new() {
        let tree_vec = DefaultTreeVec::<u64>::new();
        assert_eq!(tree_vec.data.len(), 0);
        assert_eq!(tree_vec.empty.len(), 0);
    }

    #[test]
    fn test_default_vec_add() {
        let mut tree_vec = DefaultTreeVec::<u64>::new();
        tree_vec.push(1);
        assert_eq!(tree_vec.data.len(), 1);
        assert_eq!(tree_vec.empty.len(), 0);
        assert_eq!(tree_vec.data[0], 1);
    }

    #[test]
    fn test_default_vec_get() {
        let mut tree_vec = DefaultTreeVec::<u64>::new();
        tree_vec.push(1);

        let item = tree_vec.get(0);

        assert!(item.is_some());
        assert_eq!(item.unwrap().value, 1)
    }

    #[test]
    fn test_default_vec_add_remove() {
        let mut tree_vec = DefaultTreeVec::<u64>::new();
        tree_vec.push(1);
        tree_vec.push(2);
        tree_vec.push(3);
        tree_vec.remove(1);

        assert_eq!(tree_vec.data.len(), 3);
        assert_eq!(tree_vec.empty.len(), 1);
        assert_eq!(tree_vec.empty[0], 1);

        tree_vec.push(6);

        assert_eq!(tree_vec.data.len(), 3);
        assert_eq!(tree_vec.empty.len(), 0);

        assert_eq!(tree_vec.data[0], 1);
        assert_eq!(tree_vec.data[1], 6);
        assert_eq!(tree_vec.data[2], 3);
    }

    #[test]
    fn test_default_vec_get_removed() {
        let mut vec = DefaultTreeVec::<i32>::new();
        let index = vec.push(1);

        vec.push(2);
        vec.push(3);

        assert!(vec.remove(index).is_some());
        assert!(vec.get(index).is_none());
    }

    #[test]
    fn test_default_vec_remove() {
        let mut vec = DefaultTreeVec::<i32>::new();
        let index = vec.push(1);

        vec.push(2);
        vec.push(3);

        assert!(vec.remove(index).is_some());

        assert_eq!(vec.data.len(), 3);
        assert_eq!(vec.empty.len(), 1);
        assert_eq!(vec.empty[0], 0);

        assert!(vec.remove(index).is_none());
    }

    #[test]
    fn test_default_vec_remove_last() {
        let mut vec = DefaultTreeVec::<i32>::new();

        vec.push(1);
        vec.push(2);
        let index = vec.push(3);

        assert!(vec.remove(index).is_some());
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn test_default_vec_get_out_of_bounds() {
        let mut vec = DefaultTreeVec::<i32>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert!(vec.get(5).is_none());
    }

    #[test]
    fn test_default_vec_remove_out_of_bounds() {
        let mut vec = DefaultTreeVec::<i32>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert!(vec.remove(5).is_none());
        assert_eq!(vec.empty.len(), 0);
    }
}
