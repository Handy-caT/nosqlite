use crate::core::structs::tree::nodes::tree_index::TreeIndex;
use crate::core::structs::tree::nodes::tree_node::TreeNode;
use crate::core::structs::tree::vectors::tree_vec::{DefaultFunctions, OptimizedFunctions, TreeVecLevels};

/// Function to push a value to a vector
/// # Arguments
/// * `vec` - Vector to push the value to
/// * `value` - Value to push
/// # Returns
/// * `i32` - Index of the pushed value
/// # Type parameters
/// * `T` - Type of the data that the vector stores
/// * `V` - Type of the vector
pub(in crate::core::structs::tree::vectors) fn push<T, V: DefaultFunctions<T> + OptimizedFunctions<T> + TreeVecLevels>(vec: &mut V, value: T) -> i32 {
    let index = if vec.get_empty().len() > 0 {
        vec.get_empty_mut().pop().unwrap() as u64
    } else {
        *vec.get_length_mut() += 1;
        vec.get_data().len() as u64
    };

    let indexes = TreeIndex::new_with_index(index as i32);

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

/// Function to get a value from a vector
/// # Arguments
/// * `vec` - Vector to get the value from
/// * `index` - Index of the value to get
/// # Returns
/// * `Option<TreeNode<T>>` - Value at the index
/// # Type parameters
/// * `T` - Type of the data that the vector stores
/// * `V` - Type of the vector
pub(in crate::core::structs::tree::vectors) fn get<T: Default + Copy, V: DefaultFunctions<T> + OptimizedFunctions<T> + TreeVecLevels>(vec: &V, index: i32) -> Option<TreeNode<T>> {
    let item = vec.get_indexes().get(index as usize);
    return if item.is_none() {
        None
    } else {
        let item = item.unwrap();
        if item.index == -1 {
            None
        } else {
            let value = vec.get_data().get(index as usize);
            Some(TreeNode {
                value: *value.unwrap(),
                indexes: *item,
            })
        }
    }
}

/// Function to remove a value from a vector
/// # Arguments
/// * `vec` - Vector to remove the value from
/// * `index` - Index of the value to remove
/// # Returns
/// * `Option<TreeNode<T>>` - Value at the index
/// # Type parameters
/// * `T` - Type of the data that the vector stores
/// * `V` - Type of the vector
pub(in crate::core::structs::tree::vectors) fn remove<T: Default + Copy, V: DefaultFunctions<T> + OptimizedFunctions<T> + TreeVecLevels>(vec: &mut V, index: i32) -> Option<TreeNode<T>> {
    vec.get_empty_mut().push(index as u64);
    let item = vec.get_indexes().get(index as usize);
    if item.is_none() {
        return None;
    }

    let item = *item.unwrap();
    if item.index == -1 {
        return None;
    }

    vec.get_indexes_mut()[index as usize] = TreeIndex::default();

    if index as u64 == vec.get_length() - 1 {
        *vec.get_length_mut() -= 1;
    }

    let value = vec.get_data().get(index as usize);

    Some(TreeNode {
        value: *value.unwrap(),
        indexes: item,
    })
}

/// Function to allocate a new level in a vector
/// # Arguments
/// * `vec` - Vector to allocate the level to
/// # Type parameters
/// * `T` - Type of the data that the vector stores
/// * `V` - Type of the vector
pub(in crate::core::structs::tree::vectors) fn allocate_level<T: Default + Copy, V: DefaultFunctions<T> + OptimizedFunctions<T> + TreeVecLevels>(vec: &mut V) {
    let new_length = 2u64.pow(vec.get_allocated_levels() as u32 + 1) - 1;
    let additional = new_length - vec.get_max_length();

    vec.get_data_mut().reserve(additional as usize);
    vec.get_indexes_mut().reserve(additional as usize);

    *vec.get_length_mut() = new_length;
    *vec.get_allocated_levels_mut() += 1;
}