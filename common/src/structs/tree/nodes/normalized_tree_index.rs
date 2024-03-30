use crate::structs::tree::nodes::tree_index::TreeIndex;
use smart_default::SmartDefault;

/// Struct that represents a normalized index in a tree
/// A normalized index is a index where `left_index` = index * 2 + 1 and
/// `right_index` = index * 2 + 2
#[derive(Copy, Clone, Debug, SmartDefault)]
pub struct NormalizedTreeIndex {
    /// Index of the node
    #[default(None)]
    pub index: Option<usize>,
    /// Height of the node
    #[default = 1]
    pub height: u8,
}

impl NormalizedTreeIndex {
    /// Creates a new node with the given index
    /// Height is set to 1 by default
    /// # Arguments
    /// * `index` - Index of the node
    /// # Returns
    /// * `NormalizedTreeIndex` - New node
    #[must_use]
    pub fn new(index: usize) -> NormalizedTreeIndex {
        NormalizedTreeIndex {
            index: Some(index),
            height: 1,
        }
    }

    /// Returns the index of the right child
    /// # Returns
    /// * `Option<usize>` - Index of the right child
    #[must_use]
    pub fn get_right_index(&self) -> Option<usize> {
        self.index.map(|index| index * 2 + 2)
    }

    /// Returns the index of the left child
    /// # Returns
    /// * `i32` - Index of the left child
    #[must_use]
    pub fn get_left_index(&self) -> Option<usize> {
        self.index.map(|index| index * 2 + 1)
    }

    /// Returns the height of the node at the given index
    /// # Arguments
    /// * `index` - Index of the node
    /// # Returns
    /// * `u8` - Height of the node
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn find_height(index: usize) -> u8 {
        (index as f64 + 1.).log2().floor() as u8 + 1
    }
}

impl From<NormalizedTreeIndex> for Option<TreeIndex> {
    fn from(value: NormalizedTreeIndex) -> Self {
        value.index.map(|index| TreeIndex {
            index: Some(index),
            left_index: value.get_left_index(),
            right_index: value.get_right_index(),
            height: value.height,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalized_tree_index_new() {
        let node = NormalizedTreeIndex::new(0);
        assert_eq!(node.index, Some(0));
        assert_eq!(node.height, 1);
    }

    #[test]
    fn test_normalized_tree_index_get_right_index() {
        let node = NormalizedTreeIndex::new(0);
        assert_eq!(node.get_right_index(), Some(2));
    }

    #[test]
    fn test_normalized_tree_index_get_left_index() {
        let node = NormalizedTreeIndex::new(0);
        assert_eq!(node.get_left_index(), Some(1));
    }

    #[test]
    fn test_normalized_tree_index_into() {
        let node = NormalizedTreeIndex::new(0);
        let tree_node: Option<TreeIndex> = node.into();

        assert!(tree_node.is_some());
        let tree_node = tree_node.unwrap();

        assert_eq!(tree_node.index, Some(0));
        assert_eq!(tree_node.height, 1);
        assert_eq!(tree_node.left_index, Some(1));
        assert_eq!(tree_node.right_index, Some(2));
    }

    #[test]
    fn test_normalized_tree_index_height() {
        assert_eq!(NormalizedTreeIndex::find_height(0), 1);
        assert_eq!(NormalizedTreeIndex::find_height(1), 2);
        assert_eq!(NormalizedTreeIndex::find_height(14), 4)
    }
}
