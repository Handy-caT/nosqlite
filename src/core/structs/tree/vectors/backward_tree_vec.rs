use crate::core::structs::tree::tree_index::TreeIndex;
use crate::core::structs::tree::vectors::optimized_tree_vec::INITIAL_LEVELS;
use crate::core::structs::tree::vectors::tree_vec::TreeVec;

pub struct BackwardsTreeVec<T> {
    allocated_levels: u8,
    max_length: u64,
    length: u64,

    data: Vec<T>,
    indexes: Vec<TreeIndex>,
    empty: Vec<u64>,

    parents: Vec<u64>,
}

impl <T: Default + Copy> BackwardsTreeVec<T> {
    pub fn new() -> BackwardsTreeVec<T> {
        let mut vec = BackwardsTreeVec {
            allocated_levels: 0,
            max_length: 0,
            length: 0,
            data: Vec::new(),
            indexes: Vec::new(),
            empty: Vec::new(),
            parents: Vec::new(),
        };

        let length = 2u64.pow(INITIAL_LEVELS as u32) - 1;

        vec.data.reserve(length as usize);
        vec.indexes.reserve(length as usize);

        vec.max_length = length;
        vec.allocated_levels = INITIAL_LEVELS;

        vec
    }

    fn allocate_level(&mut self) {
        let new_length = 2u64.pow(self.allocated_levels as u32 + 1) - 1;
        let additional = new_length - self.max_length;

        self.data.reserve(additional as usize);
        self.indexes.reserve(additional as usize);

        self.max_length = new_length;
        self.allocated_levels += 1;
    }
}

