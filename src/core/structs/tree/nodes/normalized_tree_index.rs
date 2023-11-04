use crate::core::structs::tree::nodes::tree_index::TreeIndex;
use std::io;

/// Struct that represents a normalized index in a tree
/// A normalized index is a index where left_index = index * 2 + 1 and right_index = index * 2 + 2
pub struct NormalizedTreeIndex {
    /// Index of the node
    pub index: i32,
    /// Height of the node
    pub height: u8,
}

impl NormalizedTreeIndex {
    /// Creates a new node with the given index
    /// Height is set to 1 by default
    /// # Arguments
    /// * `index` - Index of the node
    /// # Returns
    /// * `NormalizedTreeIndex` - New node
    pub fn new(index: i32) -> NormalizedTreeIndex {
        NormalizedTreeIndex { index, height: 1 }
    }

    /// Returns the index of the right child
    /// # Returns
    /// * `i32` - Index of the right child
    pub fn get_right_index(&self) -> i32 {
        self.index * 2 + 2
    }

    /// Returns the index of the left child
    /// # Returns
    /// * `i32` - Index of the left child
    pub fn get_left_index(&self) -> i32 {
        self.index * 2 + 1
    }

    /// Checks if the given node can be normalized
    /// # Arguments
    /// * `node` - TreeIndex to check
    /// # Returns
    /// * `bool` - True if the node can be normalized, false otherwise
    fn can_be_normalized(node: &TreeIndex) -> bool {
        node.left_index == node.index * 2 + 1 && node.right_index == node.index * 2 + 2
    }

    /// Returns the height of the node at the given index
    /// # Arguments
    /// * `index` - Index of the node
    /// # Returns
    /// * `u8` - Height of the node
    pub fn find_height(index: i32) -> u8 {
        (index as f32 + 1.).log2().floor() as u8 + 1
    }
}

impl Copy for NormalizedTreeIndex {}

impl Clone for NormalizedTreeIndex {
    fn clone(&self) -> Self {
        *self
    }
}

impl Into<TreeIndex> for NormalizedTreeIndex {
    fn into(self) -> TreeIndex {
        TreeIndex {
            index: self.index,
            left_index: self.get_left_index(),
            right_index: self.get_right_index(),
            height: self.height,
        }
    }
}

impl From<TreeIndex> for Result<NormalizedTreeIndex, io::Error> {
    fn from(node: TreeIndex) -> Result<NormalizedTreeIndex, io::Error> {
        if !NormalizedTreeIndex::can_be_normalized(&node) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Node is not normalized",
            ));
        }
        Ok(NormalizedTreeIndex {
            index: node.index,
            height: node.height,
        })
    }
}

impl Default for NormalizedTreeIndex {
    fn default() -> Self {
        NormalizedTreeIndex {
            height: 1,
            index: -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalized_tree_index_new() {
        let node = NormalizedTreeIndex::new(0);
        assert_eq!(node.index, 0);
        assert_eq!(node.height, 1);
    }

    #[test]
    fn test_normalized_tree_index_get_right_index() {
        let node = NormalizedTreeIndex::new(0);
        assert_eq!(node.get_right_index(), 2);
    }

    #[test]
    fn test_normalized_tree_index_get_left_index() {
        let node = NormalizedTreeIndex::new(0);
        assert_eq!(node.get_left_index(), 1);
    }

    #[test]
    fn test_normalized_tree_index_into() {
        let node = NormalizedTreeIndex::new(0);
        let tree_node: TreeIndex = node.into();

        assert_eq!(tree_node.index, 0);
        assert_eq!(tree_node.height, 1);
        assert_eq!(tree_node.left_index, 1);
        assert_eq!(tree_node.right_index, 2);
    }

    #[test]
    fn test_normalized_tree_index_from() {
        let mut tree_node = TreeIndex::new_with_index(0);
        tree_node.right_index = 2;
        tree_node.left_index = 1;

        let node: Result<NormalizedTreeIndex, io::Error> = tree_node.into();
        let unwrapped = node.unwrap();

        assert_eq!(unwrapped.index, 0);
        assert_eq!(unwrapped.height, 1);
    }

    #[test]
    fn test_normalized_tree_index_height() {
        assert_eq!(NormalizedTreeIndex::find_height(0), 1);
        assert_eq!(NormalizedTreeIndex::find_height(1), 2);
        assert_eq!(NormalizedTreeIndex::find_height(14), 4)
    }
}
