use std::ops::{Index, IndexMut};
use crate::core::structs::tree::tree_index::TreeIndex;
use crate::core::structs::tree::vectors::tree_vec::{TreeVec, TreeVecLevels};

pub struct AdditionalIndexVec {
    pub indexes: Vec<TreeIndex>,
    allocated_levels: u8,
    max_length: u64,
}

impl AdditionalIndexVec {
    pub fn new<T: Default + Copy, M: TreeVec<T> + TreeVecLevels + Sized>(tree_vec: &M) -> AdditionalIndexVec {
        let mut vec = AdditionalIndexVec {
            indexes: Vec::new(),
            allocated_levels: tree_vec.get_allocated_levels(),
            max_length: tree_vec.get_max_length(),
        };

        let length = tree_vec.get_max_length();
        vec.indexes.reserve(length as usize);

        // let length = tree_vec.len();
        // vec.indexes.resize(length, TreeIndex::default());

        vec
    }

    fn allocate_level(&mut self) {
        let new_length = 2u64.pow(self.allocated_levels as u32 + 1) - 1;
        let additional = new_length - self.max_length;

        self.indexes.reserve(additional as usize);

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

    pub fn remove(&mut self, index: i32) -> TreeIndex {
        self.indexes.remove(index as usize)
    }

    pub fn len(&self) -> usize {
        self.indexes.len()
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

#[cfg(test)]
mod tests {
    use crate::core::structs::tree::vectors::additional_index_vec::AdditionalIndexVec;
    use crate::core::structs::tree::vectors::optimized_tree_vec::OptimizedTreeVec;
    use crate::core::structs::tree::vectors::tree_vec::TreeVecLevels;

    #[test]
    fn test_additional_index_vec() {
        let optimized_tree_vec = OptimizedTreeVec::<u64>::new();

        let vec = AdditionalIndexVec::new(&optimized_tree_vec);

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.allocated_levels, optimized_tree_vec.get_allocated_levels());
        assert_eq!(vec.max_length, optimized_tree_vec.get_max_length());
    }
}
