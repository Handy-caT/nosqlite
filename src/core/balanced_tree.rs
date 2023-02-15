use crate::core::tree_node::TreeNode;
use crate::core::tree_vec::TreeVec;

struct BalancedTree<T> {
    nodes: TreeVec<T>,
    root: i32,
}

impl <T: Default + PartialOrd + Copy> BalancedTree<T> {
    pub fn new() -> BalancedTree<T> {
        BalancedTree {
            nodes: TreeVec::new(),
            root: 0,
        }
    }

    fn height(&self, root_index: i32) -> u8 {
        if root_index == -1 {
            0
        } else {
            self.nodes[root_index as u64].height
        }
    }

    fn bfactor(&mut self, root_index: i32) -> i8 {
        let node: &TreeNode<T> = &self.nodes[root_index as u64];
        self.height(node.right_index) as i8 - self.height(node.left_index) as i8
    }

    fn fix_height(&mut self, root_index: i32) {
        let node: &TreeNode<T> = &self.nodes[root_index as u64];
        let left_height = self.height(node.left_index);
        let right_height =  self.height(node.right_index);

        let height = if left_height > right_height {
            left_height + 1
        } else {
            right_height + 1
        };

        let node: &mut TreeNode<T> = &mut self.nodes[root_index as u64];
        node.height = height
    }

    fn rotate_right(&mut self, root_index: i32) -> i32 {
        let left_index = self.nodes[root_index as u64].left_index;

        self.nodes[root_index as u64].left_index = self.nodes[left_index as u64].right_index;
        self.nodes[left_index as u64].right_index = root_index;

        self.fix_height(root_index);
        self.fix_height(left_index);

        left_index
    }

    fn rotate_left(&mut self, root_index: i32) -> i32 {
        let right_index = self.nodes[root_index as u64].right_index;

        self.nodes[root_index as u64].right_index = self.nodes[right_index as u64].left_index;
        self.nodes[right_index as u64].left_index = root_index;

        self.fix_height(root_index);
        self.fix_height(right_index);

        right_index
    }

    fn balance(&mut self, root_index: i32) -> i32 {
        let mut new_root_index = root_index;
        self.fix_height(root_index);

        if self.bfactor(root_index) == 2 {
            if self.bfactor(self.nodes[root_index as u64].right_index) < 0 {
                self.nodes[root_index as u64].right_index = self.rotate_right(self.nodes[root_index as u64].right_index);
            }
            new_root_index = self.rotate_left(root_index);
        }

        if self.bfactor(root_index) == -2 {
            if self.bfactor(self.nodes[root_index as u64].left_index) > 0 {
                self.nodes[root_index as u64].left_index = self.rotate_left(self.nodes[root_index as u64].left_index);
            }
            new_root_index = self.rotate_right(root_index);
        }

        new_root_index
    }

    fn add_from_root(&mut self, root_index: i32, value: T) -> i32 {
        if self.nodes[root_index as u64].value > value {
            if self.nodes[root_index as u64].left_index == -1 {
                self.nodes[root_index as u64].left_index = self.nodes.add(value);
            } else {
                self.nodes[root_index as u64].left_index = self.add_from_root(self.nodes[root_index as u64].left_index, value);
            }
        } else {
            if self.nodes[root_index as u64].right_index == -1 {
                self.nodes[root_index as u64].right_index = self.nodes.add(value);
            } else {
                self.nodes[root_index as u64].right_index = self.add_from_root(self.nodes[root_index as u64].right_index, value);
            }
        }
        self.balance(root_index)
    }

    pub fn add(&mut self, value: T) {
        if self.nodes.len() == 0 {
            self.root = self.nodes.add(value);
        } else {
            self.root = self.add_from_root(self.root, value);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tree = BalancedTree::<u64>::new();
        assert_eq!(tree.nodes.len(), 0);
        assert_eq!(tree.root, 0);
    }

    #[test]
    fn test_add_root() {
        let mut tree = BalancedTree::<u64>::new();
        tree.add(1);
        assert_eq!(tree.nodes.len(), 1);
        assert_eq!(tree.root, 0);
        assert_eq!(tree.nodes[0].value, 1);
    }

    #[test]
    fn test_add_left() {
        let mut tree = BalancedTree::<u64>::new();
        tree.add(1);
        tree.add(0);
        assert_eq!(tree.nodes.len(), 2);
        assert_eq!(tree.root, 0);
        assert_eq!(tree.nodes[0].value, 1);
        assert_eq!(tree.nodes[0].left_index, 1);
        assert_eq!(tree.nodes[1].value, 0);
    }

    #[test]
    fn test_add_right() {
        let mut tree = BalancedTree::<u64>::new();
        tree.add(1);
        tree.add(2);
        assert_eq!(tree.nodes.len(), 2);
        assert_eq!(tree.root, 0);
        assert_eq!(tree.nodes[0].value, 1);
        assert_eq!(tree.nodes[0].right_index, 1);
        assert_eq!(tree.nodes[1].value, 2);
    }

    #[test]
    fn test_add_left_right() {
        let mut tree = BalancedTree::<u64>::new();
        tree.add(1);
        tree.add(0);
        tree.add(2);
        assert_eq!(tree.nodes.len(), 3);
        assert_eq!(tree.root, 0);
        assert_eq!(tree.nodes[0].value, 1);
        assert_eq!(tree.nodes[0].left_index, 1);
        assert_eq!(tree.nodes[0].right_index, 2);
        assert_eq!(tree.nodes[1].value, 0);
        assert_eq!(tree.nodes[2].value, 2);
    }

    #[test]
    fn test_balance() {
        let mut tree = BalancedTree::<u64>::new();
        tree.add(1);
        tree.add(2);
        tree.add(3);
        assert_eq!(tree.nodes.len(), 3);
        assert_eq!(tree.root, 1);
        assert_eq!(tree.nodes[1].value, 2);
        assert_eq!(tree.nodes[1].left_index, 0);
        assert_eq!(tree.nodes[1].right_index, 2);
    }

    #[test]
    fn test_balance_long() {
        let mut tree = BalancedTree::<u64>::new();
        tree.add(1);
        tree.add(2);
        tree.add(3);
        tree.add(4);
        tree.add(5);
        tree.add(6);
        tree.add(7);

        assert_eq!(tree.nodes.len(), 7);
        assert_eq!(tree.root, 3);
        assert_eq!(tree.nodes[3].value, 4);
    }

    #[test]
    fn test_balance_long2() {
        let mut tree = BalancedTree::<u64>::new();

        tree.add(7);
        tree.add(6);
        tree.add(5);
        tree.add(4);
        tree.add(3);
        tree.add(2);
        tree.add(1);

        assert_eq!(tree.nodes.len(), 7);
        assert_eq!(tree.root, 3);
        assert_eq!(tree.nodes[3].value, 4);
    }
}