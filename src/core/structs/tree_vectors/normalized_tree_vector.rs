use std::ops::{Index, IndexMut};
use crate::core::structs::tree_nodes::normalized_node::NormalizedNode;

const INITIAL_LEVELS: u8 = 6;

pub struct NormalizedTreeVector<T> {
    pub data: Vec<NormalizedNode<T>>,
    pub size: u64,
    allocated_levels: u8,
    max_length: u64,
}

impl <T: Default + Copy> NormalizedTreeVector<T> {
    pub fn new() -> NormalizedTreeVector<T> {
        let mut vec = NormalizedTreeVector {
            data: Vec::new(),
            size: 0,
            allocated_levels: 0,
            max_length: 0,
        };

        let length = 2u64.pow(INITIAL_LEVELS as u32) - 1;

        vec.data.reserve(length as usize);
        vec.max_length = length;
        vec.allocated_levels = INITIAL_LEVELS;

        vec
    }

    pub fn get_parent_index(index: i32) -> i32 {
        (index - 1) / 2
    }

    fn allocate_level(&mut self) {
        let new_length = 2u64.pow(self.allocated_levels as u32 + 1) - 1;
        let additional = new_length - self.max_length;

        self.data.reserve(additional as usize);
        self.max_length = new_length;
        self.allocated_levels += 1;
    }

    pub fn add(&mut self, value: T) -> i32 {
        let index = self.size;
        let node = NormalizedNode::new(value, index as i32);

        if index == self.max_length {
            self.allocate_level();
        }

        self.data.push(node);
        self.size += 1;

        index as i32
    }

    pub fn get(self: &mut NormalizedTreeVector<T>, index: i32) -> NormalizedNode<T> {
        self.data[index as usize]
    }

    pub fn swap(&mut self, index1: i32, index2: i32) {
        let mut node1 = self.data[index1 as usize];
        node1.index = index2;
        let mut node2 = self.data[index2 as usize];
        node2.index = index1;

        self.data[index1 as usize] = node2;
        self.data[index2 as usize] = node1;
    }

    pub fn remove(&mut self, index: i32) {
        self.data[index as usize] = NormalizedNode::default();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl <T> Index<u64> for NormalizedTreeVector<T> {
    type Output = NormalizedNode<T>;

    fn index(&self, index: u64) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl <T> IndexMut<u64> for NormalizedTreeVector<T> {
    fn index_mut(&mut self, index: u64) -> &mut NormalizedNode<T> {
        &mut self.data[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalized_tree_vec_new() {
        let tree_vec = NormalizedTreeVector::<u64>::new();
        assert_eq!(tree_vec.data.len(), 0);
    }

    #[test]
    fn test_normalized_tree_vec_add() {
        let mut tree_vec = NormalizedTreeVector::<u64>::new();
        let index = tree_vec.add(1);
        assert_eq!(index, 0);
        assert_eq!(tree_vec.data.len(), 1);
    }

    #[test]
    fn test_normalized_tree_vec_get() {
        let mut tree_vec = NormalizedTreeVector::<u64>::new();
        let index = tree_vec.add(1);
        let node = tree_vec.get(index);
        assert_eq!(node.value, 1);
        assert_eq!(node.index, 0);
    }

    #[test]
    fn test_normalized_tree_vec_swap() {
        let mut tree_vec = NormalizedTreeVector::<u64>::new();
        let index1 = tree_vec.add(1);
        let index2 = tree_vec.add(2);

        tree_vec.swap(index1, index2);

        let node1 = tree_vec.get(index1);
        let node2 = tree_vec.get(index2);

        assert_eq!(node1.value, 2);
        assert_eq!(node1.index, 0);
        assert_eq!(node2.value, 1);
        assert_eq!(node2.index, 1);
    }
    
}


