use std::fmt::Debug;

pub struct TreeIndex {
    pub index: i32,
    pub left_index: i32,
    pub right_index: i32,
    pub height: u8,
}

impl TreeIndex {
    pub fn new_with_index(index: i32) -> TreeIndex {
        TreeIndex {
            index,
            left_index: -1,
            right_index: -1,
            height: 1,
        }
    }

    pub fn new() -> TreeIndex {
        TreeIndex {
            index: -1,
            left_index: -1,
            right_index: -1,
            height: 1,
        }
    }
}

impl Clone for TreeIndex {
    fn clone(&self) -> Self {
        TreeIndex {
            index: self.index,
            left_index: self.left_index,
            right_index: self.right_index,
            height: self.height,
        }
    }
}

impl Copy for TreeIndex {}

impl Default for TreeIndex {
    fn default() -> Self {
        TreeIndex {
            index: -1,
            left_index: -1,
            right_index: -1,
            height: 1,
        }
    }
}

impl PartialEq for TreeIndex {
    fn eq(&self, other: &Self) -> bool {
        return self.height == other.height && self.index == other.index &&
            self.right_index == other.right_index && self.left_index == other.left_index
    }
}

impl Debug for TreeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TreeIndex {{ index: {}, left_index: {}, right_index: {}, height: {} }}", self.index, self.left_index, self.right_index, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_index_new_with_index() {
        let tree_index = TreeIndex::new_with_index(0);
        assert_eq!(tree_index.index, 0);
        assert_eq!(tree_index.left_index, -1);
        assert_eq!(tree_index.right_index, -1);
        assert_eq!(tree_index.height, 1);
    }

    #[test]
    fn test_tree_index_new() {
        let tree_index = TreeIndex::new();
        assert_eq!(tree_index.index, -1);
        assert_eq!(tree_index.left_index, -1);
        assert_eq!(tree_index.right_index, -1);
        assert_eq!(tree_index.height, 1);
    }


    #[test]
    fn test_tree_index_default() {
        let tree_index = TreeIndex::default();
        assert_eq!(tree_index.index, -1);
        assert_eq!(tree_index.left_index, -1);
        assert_eq!(tree_index.right_index, -1);
        assert_eq!(tree_index.height, 1);
    }

    #[test]
    fn test_tree_index_eq() {
        let tree_index1 = TreeIndex::new_with_index(0);
        let tree_index2 = TreeIndex::new_with_index(0);
        assert_eq!(tree_index1, tree_index2);
    }
}