use crate::structs::tree::{
    nodes::tree_index::TreeIndex,
    vectors::tree_vec::{Levels, TreeVec},
};
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct AdditionalIndexVec {
    pub indexes: Vec<TreeIndex>,
    allocated_levels: u8,
    max_length: usize,
}

impl AdditionalIndexVec {
    pub fn new() -> AdditionalIndexVec {
        let mut vec = AdditionalIndexVec {
            indexes: Vec::new(),
            allocated_levels: 0,
            max_length: 0,
        };

        vec.allocate_level();

        vec
    }

    pub fn new_with_existing<
        T: Default + Clone,
        M: TreeVec<T> + Levels + Sized,
    >(
        tree_vec: &M,
    ) -> AdditionalIndexVec {
        let mut vec = AdditionalIndexVec {
            indexes: Vec::new(),
            allocated_levels: tree_vec.get_allocated_levels(),
            max_length: tree_vec.get_max_length(),
        };

        let length = tree_vec.get_max_length();
        vec.indexes.reserve(length);

        // let length = tree_vec.len();
        // vec.indexes.resize(length, TreeIndex::default());

        vec
    }

    fn allocate_level(&mut self) {
        let new_length = 2usize.pow(u32::from(self.allocated_levels) + 1) - 1;
        let additional = new_length - self.max_length;

        self.indexes.reserve(additional);

        self.max_length = new_length;
        self.allocated_levels += 1;
    }

    pub fn push(&mut self, index: TreeIndex) {
        self.indexes.push(index);
    }

    pub fn get_indexes(&self) -> &Vec<TreeIndex> {
        &self.indexes
    }

    pub fn get_indexes_mut(&mut self) -> &mut Vec<TreeIndex> {
        &mut self.indexes
    }

    pub fn remove(&mut self, index: usize) -> TreeIndex {
        self.indexes.remove(index)
    }

    pub fn len(&self) -> usize {
        self.indexes.len()
    }
}

impl Index<usize> for AdditionalIndexVec {
    type Output = TreeIndex;

    fn index(&self, index: usize) -> &Self::Output {
        &self.indexes[index]
    }
}

impl IndexMut<usize> for AdditionalIndexVec {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.indexes[index]
    }
}

impl Clone for AdditionalIndexVec {
    fn clone(&self) -> Self {
        let mut vec = AdditionalIndexVec {
            indexes: self.indexes.clone(),
            allocated_levels: self.allocated_levels,
            max_length: self.max_length,
        };

        vec
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::tree::vectors::{
        additional_indexes::additional_index_vec::AdditionalIndexVec,
        optimized_tree_vec::OptimizedTreeVec,
        tree_vec::{Levels, TreeVec},
    };

    #[test]
    fn test_additional_index_vec() {
        let optimized_tree_vec = OptimizedTreeVec::<u64>::new();

        let vec = AdditionalIndexVec::new_with_existing(&optimized_tree_vec);

        assert_eq!(vec.len(), 0);
        assert_eq!(
            vec.allocated_levels,
            optimized_tree_vec.get_allocated_levels()
        );
        assert_eq!(vec.max_length, optimized_tree_vec.get_max_length());
    }
}
