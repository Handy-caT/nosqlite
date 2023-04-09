use std::cmp::Ordering;
use std::ops::IndexMut;
use crate::core::structs::tree::object::balanced_tree::balanced_tree_functions::balance;
use crate::core::structs::tree::object::tree_object::{TreeObject, TreeObjectVec};
use crate::core::structs::tree::tree_index::TreeIndex;
use crate::core::structs::tree::vectors::additional_index_vec::AdditionalIndexVec;
use crate::core::structs::tree::vectors::tree_vec::{TreeVec, TreeVecLevels};

pub struct DecoratableBalancedTree<T, V: TreeVec<T> + Sized, M: TreeObject<T> + Sized + TreeObjectVec<T, V>> {
    base: M,
    root_index: i32,
    indexes: AdditionalIndexVec,
    compare: fn(&T, &T) -> Ordering,
    v: std::marker::PhantomData<V>,
}

impl <T: Default + Copy, V: TreeVec<T> + TreeVecLevels + Sized, M: TreeObject<T> + Sized + TreeObjectVec<T, V>> DecoratableBalancedTree<T, V, M> {
    pub fn new(tree: M, compare: fn(&T, &T) -> Ordering) -> DecoratableBalancedTree<T, V, M> {
        let additional_index_vec = AdditionalIndexVec::new(tree.get_nodes());

        let mut dec_tree = DecoratableBalancedTree {
            base: tree,
            root_index: -1,
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

    fn fill_indexes(&mut self) {
        let length = self.base.len();
        self.indexes.push(TreeIndex::new_with_index(0));
        self.root_index = 0;

        for i in 1..length {
            let item = self.base.get(i as i32);
            if item.is_none() {
                self.indexes.push(TreeIndex::default());
            } else {
                self.indexes.push(TreeIndex::new_with_index(i as i32));
                self.root_index = self.add_from_root(item.unwrap(), self.root_index, i as i32);
            }
        }
    }
}

impl <T: Default + Copy, V: TreeVec<T> + Sized, M: TreeObject<T> + Sized + TreeObjectVec<T, V>> TreeObject<T> for DecoratableBalancedTree<T, V, M>  {
    fn push(&mut self, value: T) {
        todo!()
    }

    fn find(&mut self, value: T) -> Option<i32> {
        todo!()
    }

    fn remove_by_value(&mut self, value: T) -> Option<T> {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
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
        assert_eq!(dec_tree.root_index, 1);
        assert_eq!(dec_tree.indexes[1].left_index, 2);
        assert_eq!(dec_tree.indexes[1].right_index, 0);
    }
}