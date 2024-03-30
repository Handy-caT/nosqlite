use smart_default::SmartDefault;
use std::fmt::Debug;

/// Struct that represents tree node indexes
#[derive(Copy, Clone, Debug, PartialEq, SmartDefault)]
pub struct TreeIndex {
    /// Index of the node
    #[default(None)]
    pub index: Option<usize>,
    /// Index of the left child
    #[default(None)]
    pub left_index: Option<usize>,
    /// Index of the right child
    #[default(None)]
    pub right_index: Option<usize>,
    /// Height of the node
    /// By default, height is 1
    #[default = 1]
    pub height: u8,
}

impl TreeIndex {
    /// Creates a new node with the given index
    /// Left and right indexes are set to -1 by default
    /// Height is set to 1 by default
    /// # Arguments
    /// * `index` - Index of the node
    /// # Returns
    /// * `TreeIndex` - New node
    #[must_use]
    pub fn new_with_index(index: usize) -> TreeIndex {
        TreeIndex {
            index: Some(index),
            left_index: None,
            right_index: None,
            height: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_index_new_with_index() {
        let tree_index = TreeIndex::new_with_index(0);
        assert_eq!(tree_index.index, Some(0));
        assert_eq!(tree_index.left_index, None);
        assert_eq!(tree_index.right_index, None);
        assert_eq!(tree_index.height, 1);
    }

    #[test]
    fn test_tree_index_default() {
        let tree_index = TreeIndex::default();
        assert_eq!(tree_index.index, None);
        assert_eq!(tree_index.left_index, None);
        assert_eq!(tree_index.right_index, None);
        assert_eq!(tree_index.height, 1);
    }

    #[test]
    fn test_tree_index_eq() {
        let tree_index1 = TreeIndex::new_with_index(0);
        let tree_index2 = TreeIndex::new_with_index(0);
        assert_eq!(tree_index1, tree_index2);
    }
}
