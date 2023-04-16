use std::cmp::Ordering;
use queues::{IsQueue, Queue, queue};
use crate::core::structs::tree::object::balanced_tree::balanced_tree_functions::{balance, find_min, remove_min};
use crate::core::structs::tree::object::tree_object::{TreeObject, TreeObjectFind, TreeObjectVec};
use crate::core::structs::tree::vectors::tree_vec::{TreeVec, TreeVecIndexes, TreeVecLevels};

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

impl <T: Default + PartialOrd + Copy, M: TreeVec<T> + TreeVecLevels + TreeVecIndexes<T> + Sized> BalancedTree<T, M>
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

    fn add_from_root(&mut self, value: T, root_index: i32) -> (i32, i32) {
        let mut pushed_index = -1;
        if (self.compare)(&value, self.nodes.get_value_mut(root_index)) == Ordering::Less {
            if self.nodes.get_index_mut(root_index).left_index == -1 {
                self.nodes.get_index_mut(root_index).left_index = self.nodes.push(value);
                pushed_index = self.nodes.get_index_mut(root_index).left_index;
            } else {
                let balanced = self.add_from_root(value, self.nodes.get_index(root_index).left_index);
                self.nodes.get_index_mut(root_index).left_index = balanced.0;
                pushed_index = balanced.1;
            }
        } else {
            if self.nodes.get_index_mut(root_index).right_index == -1 {
                self.nodes.get_index_mut(root_index).right_index = self.nodes.push(value);
                pushed_index = self.nodes.get_index_mut(root_index).right_index;
            } else {
                let balanced = self.add_from_root(value, self.nodes.get_index(root_index).right_index);
                self.nodes.get_index_mut(root_index).right_index = balanced.0;
                pushed_index = balanced.1;
            }
        }
        (balance(self.nodes.get_indexes(), root_index), pushed_index)
    }

    fn remove_from_root(&mut self, value: T, root_index: i32) -> i32 {
        if (self.compare)(&value, self.nodes.get_value_mut(root_index)) == Ordering::Less {
            self.nodes.get_index_mut(root_index).left_index = self.remove_from_root(value, self.nodes.get_index(root_index).left_index);
        } else if (self.compare)(&value, self.nodes.get_value_mut(root_index)) == Ordering::Greater {
            self.nodes.get_index_mut(root_index).right_index = self.remove_from_root(value, self.nodes.get_index(root_index).right_index);
        } else {
            let left_index = self.nodes.get_index_mut(root_index).left_index;
            let right_index = self.nodes.get_index_mut(root_index).right_index;

            self.nodes.remove(root_index);

            if right_index == -1 {
                return left_index;
            }

            let min_index = find_min(self.nodes.get_indexes(),right_index);
            self.nodes.get_index_mut(min_index).right_index = remove_min(self.nodes.get_indexes(),right_index);
            self.nodes.get_index_mut(min_index).left_index = left_index;

            return balance(self.nodes.get_indexes(),min_index);
        }
        balance(self.nodes.get_indexes(),root_index)
    }
}

impl <T: Default + PartialOrd + Copy, M: TreeVec<T> + TreeVecIndexes<T> + TreeVecLevels + Sized> TreeObject<T> for BalancedTree<T,M> {
    fn push(&mut self, value: T) -> i32{
        return if self.nodes.len() == 0 {
            self.root = self.nodes.push(value);
            self.root
        } else {
            let balanced = self.add_from_root(value, self.root);
            self.root = balanced.0;
            balanced.1
        }
    }

    fn find(&mut self, value: T) -> Option<i32> {
        let mut current_index = self.root;
        while current_index != -1 {
            if (self.compare)(&value, self.nodes.get_value_mut(current_index)) == Ordering::Less {
                current_index = self.nodes.get_index_mut(current_index).left_index;
            } else if (self.compare)(&value, self.nodes.get_value_mut(current_index)) == Ordering::Greater {
                current_index = self.nodes.get_index_mut(current_index).right_index;
            } else {
                return Some(self.nodes.get_index_mut(current_index).index);
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
        self.root = self.remove_from_root(value, self.root);
        Some(value)
    }

    fn is_empty(&self) -> bool {
        return self.nodes.len() == 0;
    }

    fn len(&self) -> usize {
        return self.nodes.len();
    }
}


impl <T: Default + PartialOrd + Copy, M: TreeVec<T> + TreeVecIndexes<T> + TreeVecLevels + Sized> TreeObjectVec<T, M> for BalancedTree<T,M> {
    fn get(&mut self, index: i32) -> Option<T> {
        let item = self.nodes.get(index);
        return if item.is_none() {
            None
        } else {
            Some(item.unwrap().value)
        }
    }

    fn get_nodes_mut(&mut self) -> &mut M {
        &mut self.nodes
    }

    fn get_nodes(&self) -> &M {
        &self.nodes
    }

    fn get_root_index(&self) -> i32 {
        self.root
    }

    fn remove_by_index(&mut self, index: i32) -> Option<T> {
        let item = self.nodes.get(index);
        return if item.is_none() {
            None
        } else {
            self.remove_by_value(item.unwrap().value)
        }
    }
}



impl <T: Default + PartialOrd + Copy, M: TreeVec<T> + TreeVecIndexes<T> + Sized> TreeObjectFind<T> for BalancedTree<T,M> {
    fn find_greater_equal(&mut self, value: T) -> Option<(i32,T)> {
        let mut queue: Queue<(i32, String)> = queue![];
        let mut current_index = self.root;
        let mut last = (-1, "".to_string());
        let mut ind = false;
        let mut turn_count = 0;

        while !ind && current_index != -1 {
            if (self.compare)(&value, self.nodes.get_value_mut(current_index)) == Ordering::Less {
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
                current_index = self.nodes.get_index_mut(current_index).left_index;
            } else if (self.compare)(&value, self.nodes.get_value_mut(current_index)) == Ordering::Greater {
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
                current_index = self.nodes.get_index_mut(current_index).right_index;
            } else {
                ind = true;
            }
        }

        return if ind {
            Some((self.nodes.get_index_mut(current_index).index,self.nodes[current_index]))
        } else {
            if last.1 == "right" {
                if queue.peek().unwrap().1 == "right" {
                    None
                } else {
                    let mut turn = queue.remove().unwrap();
                    while queue.peek().unwrap().1 != "right" {
                        turn = queue.remove().unwrap();
                    }

                    Some((self.nodes.get_index_mut(turn.0).index,self.nodes[turn.0]))
                }
            } else {
                Some((self.nodes.get_index_mut(last.0).index,self.nodes[last.0]))
            }
        }

    }

    fn find_less_equal(&mut self, value: T) -> Option<(i32,T)> {
        let mut queue: Queue<(i32, String)> = queue![];
        let mut current_index = self.root;
        let mut last = (-1, "".to_string());
        let mut ind = false;
        let mut turn_count = 0;

        while !ind && current_index != -1 {
            if (self.compare)(&value, self.nodes.get_value_mut(current_index)) == Ordering::Less {
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
                current_index = self.nodes.get_index_mut(current_index).left_index;
            } else if (self.compare)(&value, self.nodes.get_value_mut(current_index)) == Ordering::Greater {
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
                current_index = self.nodes.get_index_mut(current_index).right_index;
            } else {
                ind = true;
            }
        }

        return if ind {
            Some((self.nodes.get_index_mut(current_index).index, self.nodes[current_index]))
        } else {
            if last.1 == "left" {
                if queue.peek().unwrap().1 == "left" {
                    None
                } else {
                    let mut turn = queue.remove().unwrap();
                    while queue.peek().unwrap().1 != "left" {
                        turn = queue.remove().unwrap();
                    }

                    Some((self.nodes.get_index_mut(turn.0).index, self.nodes[turn.0]))
                }
            } else {
                Some((self.nodes.get_index_mut(last.0).index, self.nodes[last.0]))
            }
        }

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
    fn test_add_from_root() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.push(1);

        let balanced = tree.add_from_root(0, 0);
        assert_eq!(balanced.0, 0);
        assert_eq!(balanced.1, 1);
    }

    #[test]
    fn test_add_root() {
        let nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.push(1);
        assert_eq!(tree.nodes.len(), 1);
        assert_eq!(tree.root, 0);
        assert_eq!(tree.nodes[0], 1);
    }

    #[test]
    fn test_add_root_after_remove() {
        let nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.push(1);
        tree.push(2);
        tree.push(3);

        tree.remove_by_value(2);

        let balanced = tree.add_from_root(2, tree.root);
        assert_eq!(balanced.0, 1);
        assert_eq!(balanced.1, 1);
    }

    #[test]
    fn test_add_left() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.push(1);
        tree.push(0);
        assert_eq!(tree.nodes.len(), 2);
        assert_eq!(tree.root, 0);
        assert_eq!(tree.nodes[0], 1);
        assert_eq!(tree.nodes.get_indexes()[0].left_index, 1);
        assert_eq!(tree.nodes[1], 0);
    }

    #[test]
    fn test_add_right() {
        let mut nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);
        tree.push(1);
        tree.push(2);
        assert_eq!(tree.nodes.len(), 2);
        assert_eq!(tree.root, 0);
        assert_eq!(tree.nodes[0], 1);
        assert_eq!(tree.nodes.get_indexes()[0].right_index, 1);
        assert_eq!(tree.nodes[1], 2);
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
        assert_eq!(tree.nodes[0], 1);
        assert_eq!(tree.nodes.get_indexes()[0].left_index, 1);
        assert_eq!(tree.nodes.get_indexes()[0].right_index, 2);
        assert_eq!(tree.nodes[1], 0);
        assert_eq!(tree.nodes[2], 2);
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
        assert_eq!(tree.nodes[1], 2);
        assert_eq!(tree.nodes.get_indexes()[1].left_index, 0);
        assert_eq!(tree.nodes.get_indexes()[1].right_index, 2);
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
        assert_eq!(tree.nodes[3], 4);
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
        assert_eq!(tree.nodes[3], 4);
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
        assert_eq!(tree.nodes[0], 1);
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
        assert_eq!(tree.nodes[0], 1);
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
        assert_eq!(tree.nodes[1], 2);
        assert_eq!(tree.nodes.get_indexes()[1].left_index, 2);
        assert_eq!(tree.nodes.get_indexes()[1].right_index, 0);
        assert_eq!(tree.nodes[2], 3);
        assert_eq!(tree.nodes[0], 1);
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

    #[test]
    fn test_remove_by_index() {
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

        tree.remove_by_index(4);
        assert_eq!(tree.nodes.len(), 9);
        assert_eq!(tree.root, 0);
        assert_eq!(tree.find(300), None);
        assert_eq!(tree.find(350), Some(7));
    }
}