pub mod decoratable;
mod functions;

use crate::core::structs::tree::{
    object::{
        balanced_tree::functions::{balance, find_min, remove_min},
        tree::{FindFunctions, Tree, VecFunctions},
    },
    vectors::tree_vec::{Indexes, Levels, TreeVec},
};
use queues::{queue, IsQueue, Queue};
use std::cmp::Ordering;

pub use decoratable::Decoratable;

/// Balanced tree object
/// This structure represents balanced tree that is stored in vector
/// Vector must implement [`TreeVec`] trait
/// In balanced tree all nodes have no more than 2 children
/// It also can be customized with compare function
pub struct BalancedTree<T, M: TreeVec<T> + Sized> {
    /// Index of the root node
    root: Option<usize>,

    /// Vector of nodes
    nodes: M,

    /// Length of the tree
    len: usize,

    /// Compare function
    compare: fn(T, T) -> Ordering,
}

/// Default comparator for the balanced tree
fn default_compare<T: PartialOrd + Copy>(a: T, b: T) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

/// Functions for the balanced tree
impl<
        T: Default + PartialOrd + Copy,
        M: TreeVec<T> + Levels + Indexes<T> + Sized,
    > BalancedTree<T, M>
{
    /// Function to add value to the tree from it's root
    /// It returns index of the root in case the root was changed
    /// # Arguments
    /// * `value` - Value to be added
    /// * `root_index` - Index of the root
    /// # Returns
    /// * `(usize, usize)` - Index of the new root and index of the new node
    fn add_from_root(
        &mut self,
        value: T,
        root_index: usize,
    ) -> Option<(usize, usize)> {
        let mut pushed_index;
        if let Some(node_value) = self.nodes.get_value_mut(root_index) {
            if (self.compare)(value, *node_value) == Ordering::Less {
                if self.nodes.get_index_mut(root_index).left_index.is_none() {
                    self.nodes.get_index_mut(root_index).left_index =
                        Some(self.nodes.push(value));
                    pushed_index = self
                        .nodes
                        .get_index_mut(root_index)
                        .left_index
                        .unwrap();
                } else {
                    let balanced = self
                        .add_from_root(
                            value,
                            self.nodes
                                .get_index(root_index)
                                .left_index
                                .unwrap(),
                        )
                        .unwrap();
                    self.nodes.get_index_mut(root_index).left_index =
                        Some(balanced.0);
                    pushed_index = balanced.1;
                }
            } else if self.nodes.get_index_mut(root_index).right_index.is_none()
            {
                self.nodes.get_index_mut(root_index).right_index =
                    Some(self.nodes.push(value));
                pushed_index =
                    self.nodes.get_index_mut(root_index).right_index.unwrap();
            } else {
                let balanced = self
                    .add_from_root(
                        value,
                        self.nodes.get_index(root_index).right_index.unwrap(),
                    )
                    .unwrap();
                self.nodes.get_index_mut(root_index).right_index =
                    Some(balanced.0);
                pushed_index = balanced.1;
            }
            Some((balance(self.nodes.get_indexes(), root_index), pushed_index))
        } else {
            None
        }
    }

    /// Function to remove value from the tree from it's root
    /// It returns index of the root in case the root was changed
    /// # Arguments
    /// * `value` - Value to be removed
    /// * `root_index` - Index of the root
    /// # Returns
    /// * `usize` - Index of the new root
    fn remove_from_root(
        &mut self,
        value: T,
        root_index: usize,
    ) -> Option<usize> {
        if let Some(node_value) = self.nodes.get_value_mut(root_index) {
            if (self.compare)(value, *node_value) == Ordering::Less {
                self.nodes.get_index_mut(root_index).left_index = self
                    .remove_from_root(
                        value,
                        self.nodes.get_index(root_index).left_index.unwrap(),
                    );
            } else if (self.compare)(value, *node_value) == Ordering::Greater {
                self.nodes.get_index_mut(root_index).right_index = self
                    .remove_from_root(
                        value,
                        self.nodes.get_index(root_index).right_index.unwrap(),
                    );
            } else {
                let left_index =
                    self.nodes.get_index_mut(root_index).left_index;
                let right_index =
                    self.nodes.get_index_mut(root_index).right_index;

                self.nodes.remove(root_index);

                if right_index.is_none() {
                    return left_index;
                }

                let min_index =
                    find_min(self.nodes.get_indexes(), right_index.unwrap());
                self.nodes.get_index_mut(min_index).right_index =
                    remove_min(self.nodes.get_indexes(), right_index.unwrap());
                self.nodes.get_index_mut(min_index).left_index = left_index;

                return Some(balance(self.nodes.get_indexes(), min_index));
            }
            Some(balance(self.nodes.get_indexes(), root_index))
        } else {
            None
        }
    }
}

/// [`Tree`] trait implementation for the balanced tree
/// It implemented to use [`BalancedTree`] as [`Tree`] and
/// to use in [`Decoratable`]
impl<
        T: Default + PartialOrd + Copy,
        M: TreeVec<T> + Indexes<T> + Levels + Sized,
    > Tree<T> for BalancedTree<T, M>
{
    fn new() -> BalancedTree<T, M> {
        BalancedTree {
            root: None,
            nodes: M::new(),
            compare: default_compare,
            len: 0,
        }
    }

    fn new_with_compare(compare: fn(T, T) -> Ordering) -> BalancedTree<T, M> {
        BalancedTree {
            root: None,
            nodes: M::new(),
            compare,
            len: 0,
        }
    }

    fn push(&mut self, value: T) -> usize {
        if self.nodes.len() == 0 {
            self.root = Some(self.nodes.push(value));
            self.len += 1;
            self.root.unwrap()
        } else {
            let balanced =
                self.add_from_root(value, self.root.unwrap()).unwrap();
            self.root = Some(balanced.0);
            self.len += 1;
            balanced.1
        }
    }

    fn find(&mut self, value: T) -> Option<usize> {
        let mut current_index = self.root;
        while current_index.is_some() {
            if let Some(node_value) =
                self.nodes.get_value_mut(current_index.unwrap())
            {
                if (self.compare)(value, *node_value) == Ordering::Less {
                    current_index = self
                        .nodes
                        .get_index_mut(current_index.unwrap())
                        .left_index;
                } else if (self.compare)(value, *node_value)
                    == Ordering::Greater
                {
                    current_index = self
                        .nodes
                        .get_index_mut(current_index.unwrap())
                        .right_index;
                } else {
                    return self
                        .nodes
                        .get_index_mut(current_index.unwrap())
                        .index;
                }
            } else {
                return None;
            }
        }
        None
    }

    fn remove_by_value(&mut self, value: T) -> Option<T> {
        if self.nodes.len() == 0 {
            return None;
        } else if self.nodes.len() == 1 {
            let item = self.nodes.remove(0);
            self.len -= 1;
            return match item {
                Some(item) => {
                    self.root = None;
                    Some(item.value)
                }
                None => None,
            };
        }
        self.len -= 1;
        self.root = self.remove_from_root(value, self.root.unwrap());
        Some(value)
    }

    fn pop(&self) -> Option<T> {
        todo!()
    }

    fn is_empty(&self) -> bool {
        self.nodes.len() == 0
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl<
        T: Default + PartialOrd + Copy,
        M: TreeVec<T> + Indexes<T> + Levels + Sized,
    > VecFunctions<T, M> for BalancedTree<T, M>
{
    fn get(&mut self, index: usize) -> Option<T> {
        let item = self.nodes.get(index);
        item.map(|node| node.value)
    }

    fn get_nodes_mut(&mut self) -> &mut M {
        &mut self.nodes
    }

    fn get_nodes(&self) -> &M {
        &self.nodes
    }

    fn get_root_index(&self) -> Option<usize> {
        self.root
    }

    fn remove_by_index(&mut self, index: usize) -> Option<T> {
        let item = self.nodes.get(index);
        if let Some(node) = item {
            self.remove_by_value(node.value)
        } else {
            None
        }
    }
}

impl<T: Default + PartialOrd + Copy, M: TreeVec<T> + Indexes<T> + Sized>
    FindFunctions<T> for BalancedTree<T, M>
{
    fn find_greater_equal(&mut self, value: T) -> Option<(usize, T)> {
        let mut queue: Queue<(Option<usize>, String)> = queue![];
        let mut current_index = self.root;
        let mut last = (None, String::new());
        let mut ind = false;
        let mut turn_count = 0;

        while !ind && current_index.is_some() {
            if let Some(node_value) =
                self.nodes.get_value_mut(current_index.unwrap())
            {
                if (self.compare)(value, *node_value) == Ordering::Less {
                    if last.1 == "right" {
                        turn_count += 1;
                    }

                    last = (current_index, "left".to_string());

                    if turn_count > 1 {
                        while queue.peek().unwrap().1 != "right" {
                            let _ = queue.remove();
                        }
                    }

                    let _ = queue.add(last.clone());
                    current_index = self
                        .nodes
                        .get_index_mut(current_index.unwrap())
                        .left_index;
                } else if (self.compare)(value, *node_value)
                    == Ordering::Greater
                {
                    if last.1 == "left" {
                        turn_count += 1;
                    }

                    last = (current_index, "right".to_string());

                    if turn_count > 1 {
                        while queue.peek().unwrap().1 != "left" {
                            let _ = queue.remove();
                        }
                    }

                    let _ = queue.add(last.clone());
                    current_index = self
                        .nodes
                        .get_index_mut(current_index.unwrap())
                        .right_index;
                } else {
                    ind = true;
                }
            } else {
                current_index = None;
            }
        }

        return if ind {
            Some((
                self.nodes
                    .get_index_mut(current_index.unwrap())
                    .index
                    .unwrap(),
                self.nodes[current_index.unwrap()],
            ))
        } else if last.1 == "right" {
            if queue.peek().unwrap().1 == "right" {
                None
            } else {
                let mut turn = queue.remove().unwrap();
                while queue.peek().unwrap().1 != "right" {
                    turn = queue.remove().unwrap();
                }

                Some((
                    self.nodes.get_index_mut(turn.0.unwrap()).index.unwrap(),
                    self.nodes[turn.0.unwrap()],
                ))
            }
        } else {
            Some((
                self.nodes.get_index_mut(last.0.unwrap()).index.unwrap(),
                self.nodes[last.0.unwrap()],
            ))
        };
    }

    fn find_less_equal(&mut self, value: T) -> Option<(usize, T)> {
        let mut queue: Queue<(Option<usize>, String)> = queue![];
        let mut current_index = self.root;
        let mut last = (None, String::new());
        let mut ind = false;
        let mut turn_count = 0;

        while !ind && current_index.is_some() {
            if let Some(node_value) =
                self.nodes.get_value_mut(current_index.unwrap())
            {
                if (self.compare)(value, *node_value) == Ordering::Less {
                    if last.1 == "right" {
                        turn_count += 1;
                    }

                    last = (current_index, "left".to_string());

                    if turn_count > 1 {
                        while queue.peek().unwrap().1 != "right" {
                            let _ = queue.remove();
                        }
                    }

                    let _ = queue.add(last.clone());
                    current_index = self
                        .nodes
                        .get_index_mut(current_index.unwrap())
                        .left_index;
                } else if (self.compare)(value, *node_value)
                    == Ordering::Greater
                {
                    if last.1 == "left" {
                        turn_count += 1;
                    }

                    last = (current_index, "right".to_string());

                    if turn_count > 1 {
                        while queue.peek().unwrap().1 != "left" {
                            let _ = queue.remove();
                        }
                    }

                    let _ = queue.add(last.clone());
                    current_index = self
                        .nodes
                        .get_index_mut(current_index.unwrap())
                        .right_index;
                } else {
                    ind = true;
                }
            } else {
                current_index = None;
            }
        }

        return if ind {
            Some((
                self.nodes
                    .get_index_mut(current_index.unwrap())
                    .index
                    .unwrap(),
                self.nodes[current_index.unwrap()],
            ))
        } else if last.1 == "left" {
            if queue.peek().unwrap().1 == "left" {
                None
            } else {
                let mut turn = queue.remove().unwrap();
                while queue.peek().unwrap().1 != "left" {
                    turn = queue.remove().unwrap();
                }

                Some((
                    self.nodes.get_index_mut(turn.0.unwrap()).index.unwrap(),
                    self.nodes[turn.0.unwrap()],
                ))
            }
        } else {
            Some((
                self.nodes.get_index_mut(last.0.unwrap()).index.unwrap(),
                self.nodes[last.0.unwrap()],
            ))
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::structs::tree::vectors::default_tree_vec::DefaultTreeVec;

    #[test]
    fn test_new() {
        let tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

        assert_eq!(tree.nodes.len(), 0);
        assert_eq!(tree.root, None);
    }

    #[test]
    fn test_add_from_root() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

        tree.push(1);

        let balanced = tree.add_from_root(0, 0);
        assert!(balanced.is_some());
        let balanced = balanced.unwrap();

        assert_eq!(balanced.0, 0);
        assert_eq!(balanced.1, 1);
    }

    #[test]
    fn test_add_root() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

        tree.push(1);

        assert_eq!(tree.nodes.len(), 1);
        assert_eq!(tree.root, Some(0));
        assert_eq!(tree.nodes[0], 1);
    }

    #[test]
    fn test_add_root_after_remove() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

        tree.push(1);
        tree.push(2);
        tree.push(3);

        tree.remove_by_value(2);

        assert!(tree.root.is_some());
        let root = tree.root.unwrap();

        let balanced = tree.add_from_root(2, root);
        assert!(balanced.is_some());
        let balanced = balanced.unwrap();

        assert_eq!(balanced.0, 1);
        assert_eq!(balanced.1, 1);
    }

    #[test]
    fn test_add_left() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

        tree.push(1);
        tree.push(0);

        assert_eq!(tree.nodes.len(), 2);
        assert_eq!(tree.root, Some(0));
        assert_eq!(tree.nodes[0], 1);
        assert_eq!(tree.nodes.get_indexes()[0].left_index, Some(1));
        assert_eq!(tree.nodes[1], 0);
    }

    #[test]
    fn test_add_right() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

        tree.push(1);
        tree.push(2);

        assert_eq!(tree.nodes.len(), 2);
        assert_eq!(tree.root, Some(0));
        assert_eq!(tree.nodes[0], 1);
        assert_eq!(tree.nodes.get_indexes()[0].right_index, Some(1));
        assert_eq!(tree.nodes[1], 2);
    }

    #[test]
    fn test_add_left_right() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

        tree.push(1);
        tree.push(0);
        tree.push(2);

        assert_eq!(tree.nodes.len(), 3);
        assert_eq!(tree.root, Some(0));
        assert_eq!(tree.nodes[0], 1);
        assert_eq!(tree.nodes.get_indexes()[0].left_index, Some(1));
        assert_eq!(tree.nodes.get_indexes()[0].right_index, Some(2));
        assert_eq!(tree.nodes[1], 0);
        assert_eq!(tree.nodes[2], 2);
    }

    #[test]
    fn test_balance() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

        tree.push(1);
        tree.push(2);
        tree.push(3);

        assert_eq!(tree.nodes.len(), 3);
        assert_eq!(tree.root, Some(1));
        assert_eq!(tree.nodes[1], 2);
        assert_eq!(tree.nodes.get_indexes()[1].left_index, Some(0));
        assert_eq!(tree.nodes.get_indexes()[1].right_index, Some(2));
    }

    #[test]
    fn test_balance_long() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

        tree.push(1);
        tree.push(2);
        tree.push(3);
        tree.push(4);
        tree.push(5);
        tree.push(6);
        tree.push(7);

        assert_eq!(tree.nodes.len(), 7);
        assert_eq!(tree.root, Some(3));
        assert_eq!(tree.nodes[3], 4);
    }

    #[test]
    fn test_balance_long2() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

        tree.push(7);
        tree.push(6);
        tree.push(5);
        tree.push(4);
        tree.push(3);
        tree.push(2);
        tree.push(1);

        assert_eq!(tree.nodes.len(), 7);
        assert_eq!(tree.root, Some(3));
        assert_eq!(tree.nodes[3], 4);
    }

    #[test]
    fn test_remove_root() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

        tree.push(1);
        tree.remove_by_value(1);

        assert_eq!(tree.nodes.len(), 0);
        assert!(tree.root.is_none());
    }

    #[test]
    fn test_remove_left() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

        tree.push(1);
        tree.push(0);
        tree.remove_by_value(0);

        assert_eq!(tree.nodes.len(), 1);
        assert_eq!(tree.root, Some(0));
        assert_eq!(tree.nodes[0], 1);
    }

    #[test]
    fn test_remove_right() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

        tree.push(1);
        tree.push(2);
        tree.remove_by_value(2);

        assert_eq!(tree.nodes.len(), 1);
        assert_eq!(tree.root, Some(0));
        assert_eq!(tree.nodes[0], 1);
    }

    #[test]
    fn test_custom_compare() {
        fn compare_reversed(a: u64, b: u64) -> Ordering {
            b.cmp(&a)
        }

        let mut tree =
            BalancedTree::<u64, DefaultTreeVec<u64>>::new_with_compare(
                compare_reversed,
            );

        tree.push(1);
        tree.push(2);
        tree.push(3);

        assert_eq!(tree.nodes.len(), 3);
        assert_eq!(tree.root, Some(1));
        assert_eq!(tree.nodes[1], 2);
        assert_eq!(tree.nodes.get_indexes()[1].left_index, Some(2));
        assert_eq!(tree.nodes.get_indexes()[1].right_index, Some(0));
        assert_eq!(tree.nodes[2], 3);
        assert_eq!(tree.nodes[0], 1);
    }

    #[test]
    fn test_remove() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

        tree.push(1);
        tree.push(2);

        tree.remove_by_value(2);
        assert_eq!(tree.nodes.len(), 1);

        assert_eq!(tree.remove_by_value(1), Some(1));
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn test_remove_from_long() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

        tree.push(1);
        tree.push(2);
        tree.push(3);
        tree.push(4);
        tree.push(5);
        tree.push(6);
        tree.push(7);

        tree.remove_by_value(4);

        assert_eq!(tree.nodes.len(), 7);

        tree.remove_by_value(7);

        assert_eq!(tree.nodes.len(), 6);
    }

    #[test]
    fn test_find() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

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
            None => {}
            Some(..) => {
                panic!("Should not have found 8");
            }
        }
    }

    #[test]
    fn test_find_more_equal() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

        tree.push(100);
        tree.push(450);
        tree.push(50);
        tree.push(800);
        tree.push(300);
        tree.push(20);
        tree.push(75);
        tree.push(350);
        tree.push(70);

        assert_eq!(tree.find_greater_equal(50).unwrap(), (2, 50));
        assert_eq!(tree.find_greater_equal(73).unwrap(), (6, 75));
        assert_eq!(tree.find_greater_equal(325).unwrap(), (7, 350));

        assert_eq!(tree.find_greater_equal(68).unwrap(), (8, 70));
        assert_eq!(tree.find_greater_equal(98).unwrap(), (0, 100));
        assert_eq!(tree.find_greater_equal(10).unwrap(), (5, 20));

        let res = tree.find_greater_equal(801);
        match res {
            None => {}
            Some(..) => {
                panic!("Should not have found 801");
            }
        }
    }

    #[test]
    fn test_find_less_equal() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

        tree.push(100);
        tree.push(450);
        tree.push(50);
        tree.push(800);
        tree.push(300);
        tree.push(20);
        tree.push(75);
        tree.push(350);
        tree.push(70);

        assert_eq!(tree.find_less_equal(50).unwrap(), (2, 50));
        assert_eq!(tree.find_less_equal(73).unwrap(), (8, 70));
        assert_eq!(tree.find_less_equal(325).unwrap(), (4, 300));

        assert_eq!(tree.find_less_equal(68).unwrap(), (2, 50));
        assert_eq!(tree.find_less_equal(98).unwrap(), (6, 75));
        assert_eq!(tree.find_less_equal(30).unwrap(), (5, 20));

        let res = tree.find_less_equal(0);
        match res {
            None => {}
            Some(..) => {
                panic!("Should not have found 0");
            }
        }
    }

    #[test]
    fn test_remove_by_index() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new();

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
        assert_eq!(tree.root, Some(0));
        assert_eq!(tree.find(300), None);
        assert_eq!(tree.find(350), Some(7));
    }
}
