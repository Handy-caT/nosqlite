use std::io;
use crate::core::structs::tree_nodes::tree_node::TreeNode;
use crate::core::structs::tree_object::index_vector::TreeIndex;

pub struct NormalizedNode<T> {
    pub value: T,
    pub index: i32,
    pub height: u8,
}

impl <T> NormalizedNode<T> {
    pub fn new(value: T, index: i32) -> NormalizedNode<T> {
        NormalizedNode {
            value,
            index,
            height: 1,
        }
    }

    pub fn get_right_index(&self) -> i32 {
        self.index * 2 + 2
    }

    pub fn get_left_index(&self) -> i32 {
        self.index * 2 + 1
    }

    fn can_be_normalized(node: &TreeNode<T>) -> bool {
        node.indexes.left_index == node.indexes.index * 2 + 1 && node.indexes.right_index == node.indexes.index * 2 + 2
    }
}

impl <T: Copy> Copy for NormalizedNode<T> {}

impl <T: Clone + Copy> Clone for NormalizedNode<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl <T: Copy> Into<TreeNode<T>> for NormalizedNode<T> {
    fn into(self) -> TreeNode<T> {
        TreeNode {
            value: self.value,
            indexes: TreeIndex {
                index: self.index,
                left_index: self.get_left_index(),
                right_index: self.get_right_index(),
                height: self.height,
            },
        }
    }
}

impl <T: Copy> From<TreeNode<T>> for Result<NormalizedNode<T>,io::Error>  {
    fn from(node: TreeNode<T>) -> Result<NormalizedNode<T>, io::Error> {
        if !NormalizedNode::can_be_normalized(&node) {
            return Err(io::Error::new(io::ErrorKind::Other, "Node is not normalized"));
        }
        Ok(NormalizedNode {
            index: node.indexes.index,
            value: node.value,
            height: node.indexes.height,
        })
    }
}

impl <T: Default> Default for NormalizedNode<T> {
    fn default() -> Self {
        NormalizedNode {
            value: T::default(),
            height: 1,
            index: -1,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let node = NormalizedNode::<u64>::new(1, 0);
        assert_eq!(node.value, 1);
        assert_eq!(node.index, 0);
        assert_eq!(node.height, 1);
    }

    #[test]
    fn test_get_right_index() {
        let node = NormalizedNode::<u64>::new(1, 0);
        assert_eq!(node.get_right_index(), 2);
    }

    #[test]
    fn test_get_left_index() {
        let node = NormalizedNode::<u64>::new(1, 0);
        assert_eq!(node.get_left_index(), 1);
    }

    #[test]
    fn test_into() {
        let node = NormalizedNode::<u64>::new(1, 0);
        let tree_node: TreeNode<u64> = node.into();
        assert_eq!(tree_node.value, 1);
        assert_eq!(tree_node.indexes.index, 0);
        assert_eq!(tree_node.indexes.height, 1);
        assert_eq!(tree_node.indexes.left_index, 1);
        assert_eq!(tree_node.indexes.right_index, 2);
    }

    #[test]
    fn test_from() {
        let mut tree_node = TreeNode::<u64>::new_with_index(1, 0);
        tree_node.indexes.right_index = 2;
        tree_node.indexes.left_index = 1;

        let node: Result<NormalizedNode<u64>, io::Error> = tree_node.into();
        let unwrapped = node.unwrap();
        assert_eq!(unwrapped.value, 1);
        assert_eq!(unwrapped.index, 0);
        assert_eq!(unwrapped.height, 1);
    }
}