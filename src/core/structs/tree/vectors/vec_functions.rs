use crate::core::structs::tree::tree_index::TreeIndex;
use crate::core::structs::tree::vectors::tree_vec::{DefaultFunctions, OptimizedFunctions};

pub(in crate::core::structs::tree::vectors) fn push<T, V: DefaultFunctions<T> + OptimizedFunctions<T>>(vec: &mut V, value: T) -> i32 {
    let index = if vec.get_empty().len() > 0 {
        vec.get_empty_mut().pop().unwrap() as u64
    } else {
        vec.get_length() as u64
    };

    let indexes = TreeIndex::new_with_index(index as i32);

    if index >= vec.get_max_length() as u64 {
        vec.allocate_level();
    }

    if index == vec.get_data().len() as u64 {
        if index == vec.get_max_length() {
            vec.allocate_level();
        }
        vec.get_data_mut().push(value);
        vec.get_indexes_mut().push(indexes);
    } else {
        vec.get_data_mut()[index as usize] = value;
        vec.get_indexes_mut()[index as usize] = indexes;
    }

    index as i32
}