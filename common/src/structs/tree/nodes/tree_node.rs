use crate::structs::tree::nodes::tree_index::TreeIndex;
use std::fmt::Debug;

/// Struct that represents a node in a tree
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct TreeNode<T> {
    /// Value of the node
    pub value: T,
    /// Indexes of the node
    pub indexes: TreeIndex,
}

impl<T> TreeNode<T> {
    /// Creates a new node with the given value and indexes
    /// # Arguments
    /// * `value` - Value of the node
    /// * `index` - Index of the node
    /// # Returns
    /// * `TreeNode<T>` - New node
    pub fn new_with_index(value: T, index: usize) -> TreeNode<T> {
        TreeNode {
            value,
            indexes: TreeIndex::new_with_index(index),
        }
    }

    /// Creates a new node with the given value and default indexes
    /// # Arguments
    /// * `value` - Value of the node
    /// # Returns
    /// * `TreeNode<T>` - New node
    pub fn new(value: T) -> TreeNode<T> {
        TreeNode {
            value,
            indexes: TreeIndex::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node_new_with_index() {
        let tree_node = TreeNode::<u64>::new_with_index(1, 0);
        assert_eq!(tree_node.value, 1);
        assert_eq!(tree_node.indexes.index, Some(0));
    }

    #[test]
    fn test_tree_node_new() {
        let tree_node = TreeNode::<u64>::new(1);
        assert_eq!(tree_node.value, 1);
        assert_eq!(tree_node.indexes.index, None);
    }
}
