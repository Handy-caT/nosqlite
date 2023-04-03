use std::cmp::Ordering;
use crate::core::structs::tree::object::balanced_tree_functions::{add_from_root, find_greater_equal, find_less_equal, remove_from_root};
use crate::core::structs::tree::object::tree_object::{TreeObject, TreeObjectFind};
use crate::core::structs::tree::vectors::tree_vec::TreeVec;

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
}

impl <T: Default + PartialOrd + Copy, M: TreeVec<T> + Sized> TreeObject<T> for BalancedTree<T,M> {
    fn push(&mut self, value: T) {
        if self.nodes.len() == 0 {
            self.root = self.nodes.push(value);
        } else {
            self.root = add_from_root(&mut self.nodes, self.compare, self.root, value);
        }
    }

    fn get(&mut self, index: i32) -> Option<T> {
        let item = self.nodes.get(index);
        return if item.is_none() {
            None
        } else {
            Some(item.unwrap().value)
        }
    }

    fn find(&mut self, value: T) -> Option<i32> {
        let mut current_index = self.root;
        while current_index != -1 {
            if (self.compare)(&value, &self.nodes[current_index as usize].value) == Ordering::Less {
                current_index = self.nodes[current_index as usize].indexes.left_index;
            } else if (self.compare)(&value, &self.nodes[current_index as usize].value) == Ordering::Greater {
                current_index = self.nodes[current_index as usize].indexes.right_index;
            } else {
                return Some(self.nodes[current_index as usize].indexes.index);
            }
        }
        None
    }

    fn remove_by_value(&mut self, value: T) -> Option<T> {
        if self.nodes.len() == 0 {
            return None;
        } else if self.nodes.len() == 1 {
            self.nodes.remove(0);
            self.root = -1;
            return None;
        }
        self.root = remove_from_root(&mut self.nodes, self.compare,self.root, value);
        Some(value)
    }

    fn is_empty(&self) -> bool {
        return self.nodes.len() == 0;
    }

    fn len(&self) -> usize {
        return self.nodes.len();
    }
}

impl <T: Default + PartialOrd + Copy, M: TreeVec<T> + Sized> TreeObjectFind<T> for BalancedTree<T,M> {
    fn find_greater_equal(&mut self, value: T) -> Option<(i32,T)> {
        find_greater_equal(&mut self.nodes, self.compare, self.root, value)
    }

    fn find_less_equal(&mut self, value: T) -> Option<(i32,T)> {
        find_less_equal(&mut self.nodes, self.compare, self.root, value)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::structs::tree::vectors::default_tree_vec::DefaultTreeVec;
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
        tree.push(1);
        assert_eq!(tree.nodes.len(), 1);
        assert_eq!(tree.root, 0);
        assert_eq!(tree.nodes[0].value, 1);
    }

    #[test]
    fn test_add_left() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.push(1);
        tree.push(0);
        assert_eq!(tree.nodes.len(), 2);
        assert_eq!(tree.root, 0);
        assert_eq!(tree.nodes[0].value, 1);
        assert_eq!(tree.nodes[0].indexes.left_index, 1);
        assert_eq!(tree.nodes[1].value, 0);
    }

    #[test]
    fn test_add_right() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.push(1);
        tree.push(2);
        assert_eq!(tree.nodes.len(), 2);
        assert_eq!(tree.root, 0);
        assert_eq!(tree.nodes[0].value, 1);
        assert_eq!(tree.nodes[0].indexes.right_index, 1);
        assert_eq!(tree.nodes[1].value, 2);
    }

    #[test]
    fn test_add_left_right() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.push(1);
        tree.push(0);
        tree.push(2);
        assert_eq!(tree.nodes.len(), 3);
        assert_eq!(tree.root, 0);
        assert_eq!(tree.nodes[0].value, 1);
        assert_eq!(tree.nodes[0].indexes.left_index, 1);
        assert_eq!(tree.nodes[0].indexes.right_index, 2);
        assert_eq!(tree.nodes[1].value, 0);
        assert_eq!(tree.nodes[2].value, 2);
    }

    #[test]
    fn test_balance() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.push(1);
        tree.push(2);
        tree.push(3);
        assert_eq!(tree.nodes.len(), 3);
        assert_eq!(tree.root, 1);
        assert_eq!(tree.nodes[1].value, 2);
        assert_eq!(tree.nodes[1].indexes.left_index, 0);
        assert_eq!(tree.nodes[1].indexes.right_index, 2);
    }

    #[test]
    fn test_balance_long() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.push(1);
        tree.push(2);
        tree.push(3);
        tree.push(4);
        tree.push(5);
        tree.push(6);
        tree.push(7);

        assert_eq!(tree.nodes.len(), 7);
        assert_eq!(tree.root, 3);
        assert_eq!(tree.nodes[3].value, 4);
    }

    #[test]
    fn test_balance_long2() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.push(7);
        tree.push(6);
        tree.push(5);
        tree.push(4);
        tree.push(3);
        tree.push(2);
        tree.push(1);

        assert_eq!(tree.nodes.len(), 7);
        assert_eq!(tree.root, 3);
        assert_eq!(tree.nodes[3].value, 4);
    }

    #[test]
    fn test_remove_root() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.push(1);
        tree.remove_by_value(1);
        assert_eq!(tree.nodes.len(), 0);
        assert_eq!(tree.root, -1);
    }

    #[test]
    fn test_remove_left() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.push(1);
        tree.push(0);
        tree.remove_by_value(0);
        assert_eq!(tree.nodes.len(), 1);
        assert_eq!(tree.root, 0);
        assert_eq!(tree.nodes[0].value, 1);
    }

    #[test]
    fn test_remove_right() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.push(1);
        tree.push(2);
        tree.remove_by_value(2);
        assert_eq!(tree.nodes.len(), 1);
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

        tree.push(1);
        tree.push(2);
        tree.push(3);

        assert_eq!(tree.nodes.len(), 3);
        assert_eq!(tree.root, 1);
        assert_eq!(tree.nodes[1].value, 2);
        assert_eq!(tree.nodes[1].indexes.left_index, 2);
        assert_eq!(tree.nodes[1].indexes.right_index, 0);
        assert_eq!(tree.nodes[2].value, 3);
        assert_eq!(tree.nodes[0].value, 1);
    }

    #[test]
    fn test_remove_from_long() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.push(1);
        tree.push(2);
        tree.push(3);
        tree.push(4);
        tree.push(5);
        tree.push(6);
        tree.push(7);

        tree.remove_by_value(4);
    }

    #[test]
    fn test_find() {
        let nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.push(1);
        tree.push(2);
        tree.push(3);
        tree.push(4);
        tree.push(5);
        tree.push(6);
        tree.push(7);

        assert_eq!(tree.find(4).unwrap(), 3);
        let res = tree.find(8);
        match res {
            None => { assert!(true) }
            Some(..) => {
                panic!("Should not have found 8");
            }
        }
    }

    #[test]
    fn test_find_more_equal() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.push(100);
        tree.push(450);
        tree.push(50);
        tree.push(800);
        tree.push(300);
        tree.push(20);
        tree.push(75);
        tree.push(350);
        tree.push(70);

        assert_eq!(tree.find_greater_equal(50).unwrap(), (2,50));
        assert_eq!(tree.find_greater_equal(73).unwrap(), (6,75));
        assert_eq!(tree.find_greater_equal(325).unwrap(), (7,350));

        assert_eq!(tree.find_greater_equal(68).unwrap(), (8,70));
        assert_eq!(tree.find_greater_equal(98).unwrap(), (0,100));
        assert_eq!(tree.find_greater_equal(10).unwrap(), (5,20));

        let res = tree.find_greater_equal(801);
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
        tree.push(100);
        tree.push(450);
        tree.push(50);
        tree.push(800);
        tree.push(300);
        tree.push(20);
        tree.push(75);
        tree.push(350);
        tree.push(70);

        assert_eq!(tree.find_less_equal(50).unwrap(), (2,50));
        assert_eq!(tree.find_less_equal(73).unwrap(), (8,70));
        assert_eq!(tree.find_less_equal(325).unwrap(), (4,300));

        assert_eq!(tree.find_less_equal(68).unwrap(), (2,50));
        assert_eq!(tree.find_less_equal(98).unwrap(), (6,75));
        assert_eq!(tree.find_less_equal(30).unwrap(), (5,20));


        let res = tree.find_less_equal(0);
        match res {
            None => {assert!(true)}
            Some(..) => {
                panic!("Should not have found 0");
            }
        }
    }
}