use crate::core::structs::tree::{
    nodes::{tree_index::TreeIndex, tree_node::TreeNode},
    vectors::tree_vec::{DefaultFunctions, Levels, OptimizedFunctions},
};

/// Function to push a value to a vector
/// # Arguments
/// * `vec` - Vector to push the value to
/// * `value` - Value to push
/// # Returns
/// * `i32` - Index of the pushed value
/// # Type parameters
/// * `T` - Type of the data that the vector stores
/// * `V` - Type of the vector
pub(in crate::core::structs::tree::vectors) fn push<
    T,
    V: DefaultFunctions<T> + OptimizedFunctions<T> + Levels,
>(
    vec: &mut V,
    value: T,
) -> usize {
    let index = if vec.get_empty().is_empty() {
        *vec.get_length_mut() += 1;
        vec.get_data().len()
    } else {
        vec.get_empty_mut().pop().unwrap()
    };

    let indexes = TreeIndex::new_with_index(index);

    if index == vec.get_data().len() {
        if index == vec.get_max_length() {
            vec.allocate_level();
        }
        vec.get_data_mut().push(value);
        vec.get_indexes_mut().push(indexes);
    } else {
        vec.get_data_mut()[index] = value;
        vec.get_indexes_mut()[index] = indexes;
    }

    index
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
pub(in crate::core::structs::tree::vectors) fn get<
    T: Default + Copy,
    V: DefaultFunctions<T> + OptimizedFunctions<T> + Levels,
>(
    vec: &V,
    index: usize,
) -> Option<TreeNode<T>> {
    let item = vec.get_indexes().get(index);
    return if let Some(item) = item {
        if item.index.is_none() {
            None
        } else {
            let value = vec.get_data().get(index);
            Some(TreeNode {
                value: *value.unwrap(),
                indexes: *item,
            })
        }
    } else {
        None
    };
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
pub(in crate::core::structs::tree::vectors) fn remove<
    T: Default + Copy,
    V: DefaultFunctions<T> + OptimizedFunctions<T> + Levels,
>(
    vec: &mut V,
    index: usize,
) -> Option<TreeNode<T>> {
    vec.get_empty_mut().push(index);
    let item = *vec.get_indexes().get(index)?;

    item.index?;

    vec.get_indexes_mut()[index] = TreeIndex::default();

    if index == vec.get_length() - 1 {
        *vec.get_length_mut() -= 1;
    }

    let value = vec.get_data().get(index);

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
pub(in crate::core::structs::tree::vectors) fn allocate_level<
    T: Default + Copy,
    V: DefaultFunctions<T> + OptimizedFunctions<T> + Levels,
>(
    vec: &mut V,
) {
    let new_length = 2usize.pow(u32::from(vec.get_allocated_levels()) + 1) - 1;
    let additional = new_length - vec.get_max_length();

    vec.get_data_mut().reserve(additional);
    vec.get_indexes_mut().reserve(additional);

    *vec.get_length_mut() = new_length;
    *vec.get_allocated_levels_mut() += 1;
}
