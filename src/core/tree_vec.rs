use crate::core::tree_node::TreeNode;

pub struct TreeVec<T> {
    data: Vec<TreeNode<T>>,
    empty: Vec<u64>,
}

impl <T: Default> TreeVec<T> {
    pub fn new() -> TreeVec<T> {
        TreeVec {
            data: Vec::new(),
            empty: Vec::new(),
        }
    }

    pub fn add(&mut self, value: T) -> u64 {
        let node = TreeNode::new(value);
        let index = if self.empty.len() > 0 {
            self.empty.pop().unwrap()
        } else {
            self.data.len() as u64
        };

        if index == self.data.len() as u64 {
            self.data.push(node);
        } else {
            self.data[index as usize] = node;
        }

        index
    }

    pub fn get(&mut self, index: u64) -> &mut TreeNode<T> {
        &mut self.data[index as usize]
    }

    pub fn remove(&mut self, index: u64) {
        self.empty.push(index);
        self.data[index as usize] = TreeNode::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tree_vec = TreeVec::<u64>::new();
        assert_eq!(tree_vec.data.len(), 0);
        assert_eq!(tree_vec.empty.len(), 0);
    }

    #[test]
    fn test_add() {
        let mut tree_vec = TreeVec::<u64>::new();
        tree_vec.add(1);
        assert_eq!(tree_vec.data.len(), 1);
        assert_eq!(tree_vec.empty.len(), 0);
        assert_eq!(tree_vec.data[0].value, 1);
    }

    #[test]
    fn test_get() {
        let mut tree_vec = TreeVec::<u64>::new();
        tree_vec.add(1);
        assert_eq!(tree_vec.get(0).value, 1);
    }

    #[test]
    fn test_remove() {
        let mut tree_vec = TreeVec::<u64>::new();
        tree_vec.add(1);
        tree_vec.remove(0);
        assert_eq!(tree_vec.data.len(), 1);
        assert_eq!(tree_vec.empty.len(), 1);
        assert_eq!(tree_vec.empty[0], 0);
    }

    #[test]
    fn test_add_remove() {
        let mut tree_vec = TreeVec::<u64>::new();
        tree_vec.add(1);
        tree_vec.add(2);
        tree_vec.add(3);
        tree_vec.remove(1);

        assert_eq!(tree_vec.data.len(), 3);
        assert_eq!(tree_vec.empty.len(), 1);
        assert_eq!(tree_vec.empty[0], 1);

        tree_vec.add(6);

        assert_eq!(tree_vec.data.len(), 3);
        assert_eq!(tree_vec.empty.len(), 0);

        assert_eq!(tree_vec.data[0].value, 1);
        assert_eq!(tree_vec.data[1].value, 6);
        assert_eq!(tree_vec.data[2].value, 3);
    }

}