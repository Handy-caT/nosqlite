
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