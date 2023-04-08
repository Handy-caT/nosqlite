use std::ops::{Index, IndexMut};
use crate::core::structs::tree::tree_index::TreeIndex;
use crate::core::structs::tree::vectors::optimized_tree_vec::OptimizedTreeVec;

struct AdditionalIndexVec {
    pub indexes: Vec<TreeIndex>
}

impl AdditionalIndexVec {
    fn new<T>(tree_vec: &OptimizedTreeVec<T>) -> AdditionalIndexVec {
        let mut vec = AdditionalIndexVec {
            indexes: Vec::new()
        };

        let length = tree_vec.max_length;
        vec.indexes.reserve(length as usize);

        vec
    }
}

impl Index<usize> for AdditionalIndexVec {
    type Output = TreeIndex;

    fn index(&self, index: usize) -> &Self::Output {
        &self.indexes[index as usize]
    }
}

impl IndexMut<usize> for AdditionalIndexVec {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.indexes[index as usize]
    }
}

