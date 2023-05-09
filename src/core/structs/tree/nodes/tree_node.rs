use std::fmt::{Debug, Formatter};
use crate::core::structs::tree::nodes::tree_index::TreeIndex;

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

impl <T: PartialEq> PartialEq for TreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value && self.indexes == other.indexes
    }
}

impl <T: Debug> Debug for TreeNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreeNode")
            .field("value", &self.value)
            .field("indexes", &self.indexes)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node_new_with_index() {
        let tree_node = TreeNode::<u64>::new_with_index(1, 0);
        assert_eq!(tree_node.value, 1);
        assert_eq!(tree_node.indexes.index, 0);
    }

    #[test]
    fn test_tree_node_new() {
        let tree_node = TreeNode::<u64>::new(1);
        assert_eq!(tree_node.value, 1);
        assert_eq!(tree_node.indexes.index, -1);
    }
}