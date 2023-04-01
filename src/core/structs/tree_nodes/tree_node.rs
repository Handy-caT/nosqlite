use crate::core::structs::tree_object::index_vector::TreeIndex;

pub struct TreeNode<T> {
    pub value: T,
    pub indexes: TreeIndex,
}

impl <T> TreeNode<T> {
    pub fn new_with_index(value: T, index: i32) -> TreeNode<T> {
        TreeNode {
            value,
            indexes: TreeIndex::new_with_index(index),
        }
    }

    pub fn new(value: T) -> TreeNode<T> {
        TreeNode {
            value,
            indexes: TreeIndex::new(),
        }
    }
}

impl <T: Copy> Copy for TreeNode<T> {}

impl <T: Clone + Copy> Clone for TreeNode<T> {
    fn clone(&self) -> Self {
        TreeNode {
            value: self.value.clone(),
            indexes: self.indexes.clone(),
        }
    }
}


impl <T: Default> Default for TreeNode<T> {
    fn default() -> Self {
        TreeNode {
            value: T::default(),
            indexes: TreeIndex::default(),
        }
    }
}