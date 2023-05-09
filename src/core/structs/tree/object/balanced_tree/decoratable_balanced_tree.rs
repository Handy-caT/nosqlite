use std::cmp::Ordering;
use crate::core::structs::tree::nodes::tree_index::TreeIndex;
use crate::core::structs::tree::object::balanced_tree::balanced_tree_functions::{balance, find_min, remove_min};
use crate::core::structs::tree::object::tree_object::{TreeObject, TreeObjectVec};
use crate::core::structs::tree::vectors::additional_indexes::additional_index_vec::AdditionalIndexVec;
use crate::core::structs::tree::vectors::tree_vec::{TreeVec, TreeVecIndexes, TreeVecLevels};

pub struct DecoratableBalancedTree<T, V: TreeVec<T> + Sized, M: TreeObject<T> + Sized + TreeObjectVec<T, V>> {
    base: M,
    root: i32,
    indexes: AdditionalIndexVec,
    compare: fn(&T, &T) -> Ordering,
    v: std::marker::PhantomData<V>,
}

impl <T: Default + Copy, V: TreeVec<T> + TreeVecIndexes<T> + TreeVecLevels + Sized, M: TreeObject<T> + Sized + TreeObjectVec<T, V>> DecoratableBalancedTree<T, V, M> {
    pub fn new(tree: M, compare: fn(&T, &T) -> Ordering) -> DecoratableBalancedTree<T, V, M> {
        let additional_index_vec = AdditionalIndexVec::new(tree.get_nodes());

        let mut dec_tree = DecoratableBalancedTree {
            base: tree,
            root: -1,
            indexes: additional_index_vec,
            compare,
            v: std::marker::PhantomData,
        };

        dec_tree.fill_indexes();

        dec_tree
    }

    fn add_from_root(&mut self, value: T, root_index: i32, value_index: i32) -> i32 {
        if (self.compare)(&value, self.base.get_nodes_mut().get_value_mut(root_index)) == Ordering::Less {
            if self.indexes[root_index as usize].left_index == -1 {
                self.indexes[root_index as usize].left_index = value_index;
            } else {
                self.indexes[root_index as usize].left_index = self.add_from_root(value, self.indexes[root_index as usize].left_index, value_index);
            }
        } else {
            if self.indexes[root_index as usize].right_index == -1 {
                self.indexes[root_index as usize].right_index = value_index;
            } else {
                self.indexes[root_index as usize].right_index = self.add_from_root(value, self.indexes[root_index as usize].right_index, value_index);
            }
        }
        balance(self.indexes.get_indexes_mut(), root_index)
    }

    fn remove_from_root(&mut self, value: T, root_index: i32) -> i32 {
        if (self.compare)(&value, self.base.get_nodes_mut().get_value_mut(root_index)) == Ordering::Less {
            self.indexes[root_index as usize].left_index = self.remove_from_root(value, self.indexes[root_index as usize].left_index);
        } else if (self.compare)(&value, self.base.get_nodes_mut().get_value_mut(root_index)) == Ordering::Greater {
            self.indexes[root_index as usize].right_index = self.remove_from_root(value, self.indexes[root_index as usize].right_index);
        } else {
            let left_index = self.indexes[root_index as usize].left_index;
            let right_index = self.indexes[root_index as usize].right_index;

            if right_index == -1 {
                return left_index;
            }

            let min_index = find_min(self.indexes.get_indexes_mut(),right_index);
            self.indexes[root_index as usize].right_index = remove_min(self.indexes.get_indexes_mut(),right_index);
            self.indexes[root_index as usize].left_index = left_index;

            return balance(self.indexes.get_indexes_mut(),min_index);
        }
        balance(self.indexes.get_indexes_mut(),root_index)
    }

    fn fill_indexes(&mut self) {
        let length = self.base.len();
        self.indexes.push(TreeIndex::new_with_index(0));
        self.root = 0;

        for i in 1..length {
            let item = self.base.get(i as i32);
            if item.is_none() {
                self.indexes.push(TreeIndex::default());
            } else {
                self.indexes.push(TreeIndex::new_with_index(i as i32));
                self.root = self.add_from_root(item.unwrap(), self.root, i as i32);
            }
        }
    }

    fn push_index(&mut self, index: i32) {
        if index >= self.indexes.len() as i32 {
            self.indexes.push(TreeIndex::new_with_index(index));
        } else {
            self.indexes[index as usize] = TreeIndex::new_with_index(index);
        }
    }
}

impl <T: Default + Copy, V: TreeVec<T> + TreeVecIndexes<T> + TreeVecLevels + Sized, M: TreeObject<T> + Sized + TreeObjectVec<T, V>> TreeObject<T> for DecoratableBalancedTree<T, V, M>  {
    fn push(&mut self, value: T) -> i32 {
        let index = self.base.push(value);
        self.push_index(index);
        self.root = self.add_from_root(value, self.root, index);

        index
    }

    fn find(&mut self, value: T) -> Option<i32> {
        let mut current_index = self.root;
        while current_index != -1 {
            if (self.compare)(&value, self.base.get_nodes_mut().get_value_mut(current_index)) == Ordering::Less {
                current_index =  self.indexes[current_index as usize].left_index;
            } else if (self.compare)(&value, self.base.get_nodes_mut().get_value_mut(current_index)) == Ordering::Greater {
                current_index = self.indexes[current_index as usize].right_index;
            } else {
                return Some(self.indexes[current_index as usize].index);
            }
        }
        None
    }

    fn remove_by_value(&mut self, value: T) -> Option<T> {
        if self.len() == 0 {
            return None;
        } else if self.len() == 1 {
            self.base.remove_by_value(value);
            self.indexes[0] = TreeIndex::default();
            self.root = -1;
            return Some(value);
        }
        self.root = self.remove_from_root(value, self.root);
        self.base.remove_by_value(value);

        Some(value)
    }

    fn is_empty(&self) -> bool {
        self.base.is_empty()
    }

    fn len(&self) -> usize {
        self.base.len()
    }
}

impl <T: Default + Copy, V: TreeVec<T> + TreeVecIndexes<T> + TreeVecLevels + Sized, M: TreeObject<T> + Sized + TreeObjectVec<T, V>> TreeObjectVec<T, V> for DecoratableBalancedTree<T, V, M>  {
    fn get(&mut self, index: i32) -> Option<T> {
        self.base.get(index)
    }

    fn get_nodes_mut(&mut self) -> &mut V {
        self.base.get_nodes_mut()
    }

    fn get_nodes(&self) -> &V {
        self.base.get_nodes()
    }

    fn get_root_index(&self) -> i32 {
        self.root
    }

    fn remove_by_index(&mut self, index: i32) -> Option<T> {
        if self.len() == 0 {
            return None;
        } else if self.len() == 1 {
            let value = self.base.get(index).unwrap();
            self.base.remove_by_index(index);
            self.indexes[0] = TreeIndex::default();
            self.root = -1;
            return Some(value);
        }
        let value = self.base.get(index).unwrap();
        self.root = self.remove_from_root(value, self.root);
        self.base.remove_by_index(index);

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::structs::tree::object::balanced_tree::balanced_tree::BalancedTree;
    use crate::core::structs::tree::vectors::default_tree_vec::DefaultTreeVec;
    use super::*;

    #[test]
    fn test_decoratable_balanced_tree_new() {
        let nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);

        tree.push(1);
        tree.push(2);
        tree.push(3);

        let dec_tree = DecoratableBalancedTree::<u64, DefaultTreeVec<u64>, BalancedTree<u64, DefaultTreeVec<u64>>>::new(tree, |a, b| b.cmp(a));

        assert_eq!(dec_tree.base.len(), 3);
        assert_eq!(dec_tree.indexes.len(), 3);
        assert_eq!(dec_tree.root, 1);
        assert_eq!(dec_tree.indexes[1].left_index, 2);
        assert_eq!(dec_tree.indexes[1].right_index, 0);
    }

    #[test]
    fn test_decoratable_balanced_tree_push() {
        let nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);

        tree.push(1);
        tree.push(2);

        let mut dec_tree = DecoratableBalancedTree::<u64, DefaultTreeVec<u64>, BalancedTree<u64, DefaultTreeVec<u64>>>::new(tree, |a, b| b.cmp(a));

        dec_tree.push(3);

        assert_eq!(dec_tree.base.len(), 3);
        assert_eq!(dec_tree.indexes.len(), 3);
        assert_eq!(dec_tree.root, 1);
        assert_eq!(dec_tree.indexes[1].left_index, 2);
        assert_eq!(dec_tree.indexes[1].right_index, 0);

        assert_eq!(dec_tree.base.get_root_index(), 1);
        assert_eq!(dec_tree.base.get_nodes().get_index(1).left_index, 0);
        assert_eq!(dec_tree.base.get_nodes().get_index(1).right_index, 2);
    }

    #[test]
    fn test_decoratable_balanced_tree_find() {
        let nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);

        tree.push(1);
        tree.push(2);
        tree.push(3);

        let mut dec_tree = DecoratableBalancedTree::<u64, DefaultTreeVec<u64>, BalancedTree<u64, DefaultTreeVec<u64>>>::new(tree, |a, b| b.cmp(a));

        assert_eq!(dec_tree.find(1), Some(0));
        assert_eq!(dec_tree.find(2), Some(1));
        assert_eq!(dec_tree.find(3), Some(2));
    }

    #[test]
    fn test_decoratable_balanced_tree_remove_by_value() {
        let nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);

        tree.push(1);
        tree.push(2);
        tree.push(3);

        let mut dec_tree = DecoratableBalancedTree::<u64, DefaultTreeVec<u64>, BalancedTree<u64, DefaultTreeVec<u64>>>::new(tree, |a, b| b.cmp(a));

        assert_eq!(dec_tree.remove_by_value(1), Some(1));
        assert_eq!(dec_tree.remove_by_value(2), Some(2));
        assert_eq!(dec_tree.remove_by_value(3), Some(3));
    }

    #[test]
    fn test_decoratable_balanced_tree_remove_by_index() {
        let nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);

        tree.push(1);
        tree.push(2);
        tree.push(3);

        let mut dec_tree = DecoratableBalancedTree::<u64, DefaultTreeVec<u64>, BalancedTree<u64, DefaultTreeVec<u64>>>::new(tree, |a, b| b.cmp(a));

        assert_eq!(dec_tree.remove_by_index(0), Some(1));
        assert_eq!(dec_tree.remove_by_index(1), Some(2));
        assert_eq!(dec_tree.remove_by_index(2), Some(3));

        assert_eq!(dec_tree.get(0), None);
    }

    #[test]
    fn test_decoratable_balanced_tree_remove_root_values() {
        let nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);

        tree.push(1);

        let mut dec_tree = DecoratableBalancedTree::<u64, DefaultTreeVec<u64>, BalancedTree<u64, DefaultTreeVec<u64>>>::new(tree, |a, b| b.cmp(a));

        assert_eq!(dec_tree.remove_by_value(1), Some(1));
        assert_eq!(dec_tree.len(), 0);
        assert_eq!(dec_tree.get(0), None);

        assert_eq!(dec_tree.remove_by_value(1), None);
    }

    #[test]
    fn test_decoratable_balanced_tree_remove_root_indexes() {
        let nodes = DefaultTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::new(nodes);

        tree.push(1);

        let mut dec_tree = DecoratableBalancedTree::<u64, DefaultTreeVec<u64>, BalancedTree<u64, DefaultTreeVec<u64>>>::new(tree, |a, b| b.cmp(a));

        assert_eq!(dec_tree.remove_by_index(0), Some(1));
        assert_eq!(dec_tree.len(), 0);
        assert_eq!(dec_tree.find(1), None);
        assert_eq!(dec_tree.get(0), None);

        assert_eq!(dec_tree.remove_by_index(0), None);
    }

}