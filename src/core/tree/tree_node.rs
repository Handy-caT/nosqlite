pub struct TreeNode<T> {
    pub value: T,
    pub left_index: i32,
    pub right_index: i32,
    pub height: u8,
}

impl <T> TreeNode<T> {
    pub fn new(value: T) -> TreeNode<T> {
        TreeNode {
            value,
            left_index: -1,
            right_index: -1,
            height: 1,
        }
    }
}

impl <T: Copy> Copy for TreeNode<T> {}

impl <T: Clone + Copy> Clone for TreeNode<T> {
    fn clone(&self) -> Self {
        *self
    }
}


impl <T: Default> Default for TreeNode<T> {
    fn default() -> Self {
        TreeNode {
            value: T::default(),
            height: 1,
            left_index: -1,
            right_index: -1,
        }
    }
}