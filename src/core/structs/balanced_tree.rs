use std::cmp::Ordering;
use queues::{IsQueue, Queue, queue};
use crate::core::structs::tree_nodes::tree_node::TreeNode;
use crate::core::structs::tree_vectors::tree_vec::TreeVec;

pub struct BalancedTree<T, M: TreeVec<T> + Sized>
{
    root: i32,
    nodes: M,
    compare: fn(&T, &T) -> Ordering,
}


fn default_compare<T: PartialOrd + Copy>(a: &T, b: &T) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

impl <T: Default + PartialOrd + Copy, M: TreeVec<T> + Sized> BalancedTree<T, M>
{
    pub fn new(vec: M) -> BalancedTree<T, M> {
        BalancedTree {
            root: 0,
            nodes: vec,
            compare: default_compare,
        }
    }

    pub fn new_with_compare(vec: M, compare: fn(&T, &T) -> Ordering) -> BalancedTree<T, M> {
        BalancedTree {
            root: 0,
            nodes: vec,
            compare,
        }
    }

    fn height_from_root(&self, root_index: i32) -> u8 {
        if root_index == -1 {
            0
        } else {
            self.nodes[root_index as u64].height
        }
    }

    fn bfactor(&mut self, root_index: i32) -> i8 {
        let node: &TreeNode<T> = &self.nodes[root_index as u64];
        self.height_from_root(node.right_index) as i8 - self.height_from_root(node.left_index) as i8
    }

    fn fix_height(&mut self, root_index: i32) {
        let node: &TreeNode<T> = &self.nodes[root_index as u64];
        let left_height = self.height_from_root(node.left_index);
        let right_height =  self.height_from_root(node.right_index);

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
        if (self.compare)(&value, &self.nodes[root_index as u64].value) == Ordering::Less {
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

    fn find_min(&self, root_index: i32) -> i32 {
        if self.nodes[root_index as u64].left_index == -1 {
            root_index
        } else {
            self.find_min(self.nodes[root_index as u64].left_index)
        }
    }

    fn remove_min(&mut self, root_index: i32) -> i32 {
        if self.nodes[root_index as u64].left_index == -1 {
            self.nodes[root_index as u64].right_index
        } else {
            self.nodes[root_index as u64].left_index = self.remove_min(self.nodes[root_index as u64].left_index);
            self.balance(root_index)
        }
    }

    fn remove_from_root(&mut self, root_index: i32, value: T) -> i32 {
        if (self.compare)(&value, &self.nodes[root_index as u64].value) == Ordering::Less {
            self.nodes[root_index as u64].left_index = self.remove_from_root(self.nodes[root_index as u64].left_index, value);
        } else if (self.compare)(&value, &self.nodes[root_index as u64].value) == Ordering::Greater {
            self.nodes[root_index as u64].right_index = self.remove_from_root(self.nodes[root_index as u64].right_index, value);
        } else {
            let left_index = self.nodes[root_index as u64].left_index;
            let right_index = self.nodes[root_index as u64].right_index;

            self.nodes.remove(root_index);

            if right_index == -1 {
                return left_index;
            }

            let min_index = self.find_min(right_index);
            self.nodes[min_index as u64].right_index = self.remove_min(right_index);
            self.nodes[min_index as u64].left_index = left_index;

            return self.balance(min_index);
        }
        self.balance(root_index)
    }

    pub fn remove(&mut self, value: T) {
        if self.nodes.len() == 0 {
            return;
        } else if self.nodes.len() == 1 {
            self.nodes.remove(0);
            self.root = -1;
            return;
        }
        self.root = self.remove_from_root(self.root, value);
    }

    pub fn find(&self, value: T) -> Option<T> {
        let mut current_index = self.root;
        while current_index != -1 {
            if (self.compare)(&value, &self.nodes[current_index as u64].value) == Ordering::Less {
                current_index = self.nodes[current_index as u64].left_index;
            } else if (self.compare)(&value, &self.nodes[current_index as u64].value) == Ordering::Greater {
                current_index = self.nodes[current_index as u64].right_index;
            } else {
                return Some(self.nodes[current_index as u64].value);
            }
        }
        None
    }

    pub fn find_more_equal(&self, value: T) -> Option<T> {
        let mut queue: Queue<(i32, String)> = queue![];
        let mut current_index = self.root;
        let mut last = (-1, "".to_string());
        let mut ind = false;
        let mut turn_count = 0;

        while !ind && current_index != -1 {
            if (self.compare)(&value, &self.nodes[current_index as u64].value) == Ordering::Less {
                if last.1 == "right" {
                    turn_count += 1;
                }

                last = (current_index, "left".to_string());

                if turn_count > 1 {
                    while queue.peek().unwrap().1 != "right" {
                        queue.remove();
                    }
                }

                queue.add(last.clone());
                current_index = self.nodes[current_index as u64].left_index;
            } else if (self.compare)(&value, &self.nodes[current_index as u64].value) == Ordering::Greater {
                if last.1 == "left" {
                    turn_count += 1;
                }

                last = (current_index, "right".to_string());

                if turn_count > 1 {
                    while queue.peek().unwrap().1 != "left" {
                        queue.remove();
                    }
                }

                queue.add(last.clone());
                current_index = self.nodes[current_index as u64].right_index;
            } else {
                ind = true;
            }
        }

        return if ind {
            Some(self.nodes[current_index as u64].value)
        } else {
            if last.1 == "right" {
                if queue.peek().unwrap().1 == "right" {
                    None
                } else {
                    let mut turn = queue.remove().unwrap();
                    while queue.peek().unwrap().1 != "right" {
                        turn = queue.remove().unwrap();
                    }

                    Some(self.nodes[turn.0 as u64].value)
                }
            } else {
                return Some(self.nodes[last.0 as u64].value);
            }
        }
    }

    pub fn find_less_equal(&self, value: T) -> Option<T> {
        let mut queue: Queue<(i32, String)> = queue![];
        let mut current_index = self.root;
        let mut last = (-1, "".to_string());
        let mut ind = false;
        let mut turn_count = 0;

        while !ind && current_index != -1 {
            if (self.compare)(&value, &self.nodes[current_index as u64].value) == Ordering::Less {
                if last.1 == "right" {
                    turn_count += 1;
                }

                last = (current_index, "left".to_string());

                if turn_count > 1 {
                    while queue.peek().unwrap().1 != "right" {
                        queue.remove();
                    }
                }

                queue.add(last.clone());
                current_index = self.nodes[current_index as u64].left_index;
            } else if (self.compare)(&value, &self.nodes[current_index as u64].value) == Ordering::Greater {
                if last.1 == "left" {
                    turn_count += 1;
                }

                last = (current_index, "right".to_string());

                if turn_count > 1 {
                    while queue.peek().unwrap().1 != "left" {
                        queue.remove();
                    }
                }

                queue.add(last.clone());
                current_index = self.nodes[current_index as u64].right_index;
            } else {
                ind = true;
            }
        }

        return if ind {
            Some(self.nodes[current_index as u64].value)
        } else {
            if last.1 == "left" {
                if queue.peek().unwrap().1 == "left" {
                    None
                } else {
                    let mut turn = queue.remove().unwrap();
                    while queue.peek().unwrap().1 != "left" {
                        turn = queue.remove().unwrap();
                    }

                    Some(self.nodes[turn.0 as u64].value)
                }
            } else {
                return Some(self.nodes[last.0 as u64].value);
            }
        }

    }

    pub fn height(&self) -> u8 {
        self.nodes[self.root as u64].height
    }

    pub fn size(&self) -> u64 {
        self.nodes.len() as u64
    }

    pub(in crate::core) fn get_by_index(&self, index: u64) -> TreeNode<T> {
        return self.nodes[index]
    }

    pub(in crate::core) fn get_root(&self) -> TreeNode<T> {
        return self.nodes[self.root as u64]
    }

}

#[cfg(test)]
mod tests {
    use crate::core::structs::tree_vectors::default_tree_vector::DefaultTreeVec;
    use super::*;

    #[test]
    fn test_new() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        assert_eq!(tree.nodes.len(), 0);
        assert_eq!(tree.root, 0);
    }

    #[test]
    fn test_add_root() {
        let nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.add(1);
        assert_eq!(tree.nodes.len(), 1);
        assert_eq!(tree.root, 0);
        assert_eq!(tree.nodes[0].value, 1);
    }

    #[test]
    fn test_add_left() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
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
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
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
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
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
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
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
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
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
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
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

    #[test]
    fn test_remove_root() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new( nodes);
        tree.add(1);
        tree.remove(1);
        assert_eq!(tree.nodes.len(), 1);
        assert_eq!(tree.root, -1);
    }

    #[test]
    fn test_remove_left() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.add(1);
        tree.add(0);
        tree.remove(0);
        assert_eq!(tree.nodes.len(), 2);
        assert_eq!(tree.root, 0);
        assert_eq!(tree.nodes[0].value, 1);
    }

    #[test]
    fn test_remove_right() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.add(1);
        tree.add(2);
        tree.remove(2);
        assert_eq!(tree.nodes.len(), 2);
        assert_eq!(tree.root, 0);
        assert_eq!(tree.nodes[0].value, 1);
    }

    #[test]
    fn test_custom_compare() {
        fn compare_reversed(a: &u64, b: &u64) -> Ordering {
            if a < b {
                Ordering::Greater
            } else if a > b {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }

        let mut nodes = DefaultTreeVec::<u64>::new();
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new_with_compare(nodes, compare_reversed);

        tree.add(1);
        tree.add(2);
        tree.add(3);

        assert_eq!(tree.nodes.len(), 3);
        assert_eq!(tree.root, 1);
        assert_eq!(tree.nodes[1].value, 2);
        assert_eq!(tree.nodes[1].left_index, 2);
        assert_eq!(tree.nodes[1].right_index, 0);
        assert_eq!(tree.nodes[2].value, 3);
        assert_eq!(tree.nodes[0].value, 1);
    }

    #[test]
    fn test_remove_from_long() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.add(1);
        tree.add(2);
        tree.add(3);
        tree.add(4);
        tree.add(5);
        tree.add(6);
        tree.add(7);

        tree.remove(4);
    }

    #[test]
    fn test_find() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.add(1);
        tree.add(2);
        tree.add(3);
        tree.add(4);
        tree.add(5);
        tree.add(6);
        tree.add(7);

        assert_eq!(tree.find(4).unwrap(), 4);
        let res = tree.find(8);
        match res {
            None => {assert!(true)}
            Some(..) => {
                panic!("Should not have found 8");
            }
        }
    }

    #[test]
    fn test_find_more_equal() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.add(100);
        tree.add(450);
        tree.add(50);
        tree.add(800);
        tree.add(300);
        tree.add(20);
        tree.add(75);
        tree.add(350);
        tree.add(70);

        assert_eq!(tree.find_more_equal(50).unwrap(), 50);
        assert_eq!(tree.find_more_equal(73).unwrap(), 75);
        assert_eq!(tree.find_more_equal(325).unwrap(), 350);

        assert_eq!(tree.find_more_equal(68).unwrap(), 70);
        assert_eq!(tree.find_more_equal(98).unwrap(), 100);
        assert_eq!(tree.find_more_equal(10).unwrap(), 20);

        let res = tree.find_more_equal(801);
        match res {
            None => {assert!(true)}
            Some(..) => {
                panic!("Should not have found 801");
            }
        }
    }

    #[test]
    fn test_find_less_equal() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.add(100);
        tree.add(450);
        tree.add(50);
        tree.add(800);
        tree.add(300);
        tree.add(20);
        tree.add(75);
        tree.add(350);
        tree.add(70);

        assert_eq!(tree.find_less_equal(50).unwrap(), 50);
        assert_eq!(tree.find_less_equal(73).unwrap(), 70);
        assert_eq!(tree.find_less_equal(325).unwrap(), 300);

        assert_eq!(tree.find_less_equal(68).unwrap(), 50);
        assert_eq!(tree.find_less_equal(98).unwrap(), 75);
        assert_eq!(tree.find_less_equal(30).unwrap(), 20);


        let res = tree.find_less_equal(0);
        match res {
            None => {assert!(true)}
            Some(..) => {
                panic!("Should not have found 0");
            }
        }
    }
}