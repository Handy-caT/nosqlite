use crate::core::structs::tree::{
    nodes::{TreeIndex, TreeNode},
    vectors::tree_vec::{
        DefaultFunctions, TreeVec, TreeVecIndexes, TreeVecLevels,
    },
};
use std::ops::{Index, IndexMut};

/// Struct that represents a vector that stores tree nodes.
/// It has empty spaces that can be filled.
/// # Type parameters
/// * `T` - Type of the data that the vector stores
/// # Fields
/// * `data` - Vector that stores the data
/// * `empty` - Vector that stores the indexes of the empty spaces
/// * `indexes` - Vector that stores the indexes of the nodes
/// * `length` - Length of the vector
pub struct DefaultTreeVec<T: Sized> {
    data: Vec<T>,
    empty: Vec<usize>,
    indexes: Vec<TreeIndex>,

    length: usize,
}

impl<T: Default + Copy> DefaultTreeVec<T> {
    /// Creates a new instance of the vector
    pub fn new() -> DefaultTreeVec<T> {
        DefaultTreeVec {
            data: Vec::new(),
            empty: Vec::new(),
            indexes: Vec::new(),
            length: 0,
        }
    }
}

impl<T: Default + Copy> TreeVecIndexes<T> for DefaultTreeVec<T> {
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

impl<T: Default + Copy> TreeVec<T> for DefaultTreeVec<T> {
    fn push(&mut self, value: T) -> usize {
        let index = if self.empty.len() > 0 {
            self.empty.pop().unwrap()
        } else {
            self.length += 1;
            self.data.len()
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
        return if item.is_none() {
            None
        } else {
            let item = item.unwrap();
            if item.index.is_none() {
                None
            } else {
                let value = self.data.get(index);
                Some(TreeNode {
                    value: *value.unwrap(),
                    indexes: *item,
                })
            }
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
        let mut item = self.indexes.get(index);
        if item.is_none() {
            return None;
        }

        let item = *item.unwrap();
        if item.index.is_none() {
            return None;
        }

        self.indexes[index] = TreeIndex::default();
        self.empty.push(index);

        if index == self.length - 1 {
            self.length -= 1;
        }

        let value = self.data.get(index);

        Some(TreeNode {
            value: *value.unwrap(),
            indexes: item,
        })
    }

    fn len(&self) -> usize {
        self.length
    }
}

impl<T: Default + Copy> Index<usize> for DefaultTreeVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Default + Copy> IndexMut<usize> for DefaultTreeVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Default + Copy> DefaultFunctions<T> for DefaultTreeVec<T> {
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

impl<T> TreeVecLevels for DefaultTreeVec<T> {
    fn get_allocated_levels(&self) -> u8 {
        let length = f64::from(self.length as u16);
        let levels = length.log2().ceil() as u8;
        levels
    }

    fn get_max_length(&self) -> usize {
        let levels = self.get_allocated_levels();
        let max_length = 2usize.pow(levels as u32);
        max_length
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

        assert_eq!(item.is_some(), true);
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

        assert_eq!(vec.remove(index).is_some(), true);
        assert_eq!(vec.get(index).is_none(), true);
    }

    #[test]
    fn test_default_vec_remove() {
        let mut vec = DefaultTreeVec::<i32>::new();
        let index = vec.push(1);

        vec.push(2);
        vec.push(3);

        assert_eq!(vec.remove(index).is_some(), true);

        assert_eq!(vec.data.len(), 3);
        assert_eq!(vec.empty.len(), 1);
        assert_eq!(vec.empty[0], 0);

        assert_eq!(vec.remove(index).is_none(), true);
    }

    #[test]
    fn test_default_vec_remove_last() {
        let mut vec = DefaultTreeVec::<i32>::new();

        vec.push(1);
        vec.push(2);
        let index = vec.push(3);

        assert_eq!(vec.remove(index).is_some(), true);
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn test_default_vec_get_out_of_bounds() {
        let mut vec = DefaultTreeVec::<i32>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.get(5).is_none(), true);
    }

    #[test]
    fn test_default_vec_remove_out_of_bounds() {
        let mut vec = DefaultTreeVec::<i32>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.remove(5).is_none(), true);
        assert_eq!(vec.empty.len(), 0);
    }
}
