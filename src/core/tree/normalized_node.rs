
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
}

impl <T: Copy> Copy for NormalizedNode<T> {}

impl <T: Clone + Copy> Clone for NormalizedNode<T> {
    fn clone(&self) -> Self {
        *self
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
