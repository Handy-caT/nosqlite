use crate::structs::tree::{
    nodes::tree_index::TreeIndex,
    object::{
        balanced_tree::functions::{balance, find_min, remove_min},
        tree::{Tree, VecFunctions},
    },
    vectors::{
        additional_indexes::additional_index_vec::AdditionalIndexVec,
        tree_vec::{Indexes, Levels, TreeVec},
    },
};
use std::cmp::Ordering;

/// [`Decoratable`] balanced tree is a tree that can be decorated
/// with additional indexes.
/// It is a wrapper around [`BalancedTree`].
/// It can be used to don't duplicate data in memory.
/// So you can create various tree indexes on the
/// same data using custom compare functions.
/// You can also use [`BinHeap`] to decorate it.
#[derive(Debug)]
pub struct Decoratable<T, V, M> {
    /// Base tree object
    base: M,
    /// Root index
    root: Option<usize>,
    /// Additional indexes vector
    indexes: AdditionalIndexVec,
    /// Compare function
    compare: fn(&T, &T) -> Ordering,
    v: std::marker::PhantomData<V>,
}

impl<
        T: Default + Clone,
        V: TreeVec<T> + Levels + Sized,
        M: Tree<T> + Sized + VecFunctions<T, V>,
    > Decoratable<T, V, M>
{
    /// Create new [`Decoratable`] balanced tree with compare function
    /// and tree object.
    /// # Arguments
    /// * `tree` - tree object, can contain data.
    /// * `compare` - compare function.
    /// # Returns
    /// * `Decoratable` - new [`Decoratable`] balanced tree.
    pub fn new_with_existing(
        tree: M,
        compare: fn(&T, &T) -> Ordering,
    ) -> Decoratable<T, V, M> {
        let additional_index_vec =
            AdditionalIndexVec::new_with_existing(tree.get_nodes());

        let mut dec_tree = Decoratable {
            base: tree,
            root: None,
            indexes: additional_index_vec,
            compare,
            v: std::marker::PhantomData,
        };

        dec_tree.fill_indexes();

        dec_tree
    }

    /// Returns the link to the base [`Tree`]
    /// # Returns
    /// * `&M` - link to the base [`Tree`]
    pub fn get_base(&self) -> &M {
        &self.base
    }

    /// Returns the mutable link to the base [`Tree`]
    /// # Returns
    /// * `&mut M` - mutable link to the base [`Tree`]
    pub fn get_base_mut(&mut self) -> &mut M {
        &mut self.base
    }

    /// Function that add value index from root index
    /// # Arguments
    /// * `value` - value to add, used to compare
    /// * `root_index` - root index
    /// * `value_index` - value index
    /// # Returns
    /// * `i32` - new root index
    fn add_from_root(
        &mut self,
        value: T,
        root_index: usize,
        value_index: usize,
    ) -> Option<usize> {
        if let Some(node_value) =
            self.base.get_nodes_mut().get_value_mut(root_index)
        {
            if (self.compare)(&value, node_value) == Ordering::Less {
                if self.indexes[root_index].left_index.is_none() {
                    self.indexes[root_index].left_index = Some(value_index);
                } else {
                    self.indexes[root_index].left_index = self.add_from_root(
                        value,
                        self.indexes[root_index].left_index.unwrap(),
                        value_index,
                    );
                }
            } else if self.indexes[root_index].right_index.is_none() {
                self.indexes[root_index].right_index = Some(value_index);
            } else {
                self.indexes[root_index].right_index = self.add_from_root(
                    value,
                    self.indexes[root_index].right_index.unwrap(),
                    value_index,
                );
            }
            Some(balance(self.indexes.get_indexes_mut(), root_index))
        } else {
            None
        }
    }

    /// Function that remove value from root index
    /// # Arguments
    /// * `value` - value to remove, used to compare
    /// * `root_index` - root index
    /// # Returns
    /// * `i32` - new root index
    fn remove_from_root(
        &mut self,
        value: &T,
        root_index: usize,
    ) -> Option<usize> {
        if let Some(node_value) =
            self.base.get_nodes_mut().get_value_mut(root_index)
        {
            if (self.compare)(value, node_value) == Ordering::Less {
                self.indexes[root_index].left_index = self.remove_from_root(
                    value,
                    self.indexes[root_index].left_index.unwrap(),
                );
            } else if (self.compare)(value, node_value) == Ordering::Greater {
                self.indexes[root_index].right_index = self.remove_from_root(
                    value,
                    self.indexes[root_index].right_index.unwrap(),
                );
            } else {
                let left_index = self.indexes[root_index].left_index;
                let right_index = self.indexes[root_index].right_index;

                if right_index.is_none() {
                    return left_index;
                }

                let min_index = find_min(
                    self.indexes.get_indexes_mut(),
                    right_index.unwrap(),
                );
                self.indexes[root_index].right_index = remove_min(
                    self.indexes.get_indexes_mut(),
                    right_index.unwrap(),
                );
                self.indexes[root_index].left_index = left_index;

                return Some(balance(
                    self.indexes.get_indexes_mut(),
                    min_index,
                ));
            }
            Some(balance(self.indexes.get_indexes_mut(), root_index))
        } else {
            None
        }
    }

    /// Function that fills additional indexes when base tree is not empty
    fn fill_indexes(&mut self) {
        let length = self.base.len();
        if length == 0 {
            return;
        }
        self.indexes.push(TreeIndex::new_with_index(0));
        self.root = Some(0);

        for i in 1..length {
            let item = self.base.get_nodes().get(i);
            if let Some(node) = item {
                self.indexes.push(TreeIndex::new_with_index(i));
                let value =
                    self.base.get_nodes().get(node.indexes.index.unwrap());
                self.root = self.add_from_root(
                    value.unwrap().value,
                    self.root.unwrap(),
                    i,
                );
            } else {
                self.indexes.push(TreeIndex::default());
            }
        }
    }

    fn push_index(&mut self, index: usize) {
        if index >= self.indexes.len() {
            self.indexes.push(TreeIndex::new_with_index(index));
        } else {
            self.indexes[index] = TreeIndex::new_with_index(index);
        }
    }
}

impl<
        T: Default + Clone + Ord,
        V: TreeVec<T> + Levels + Sized,
        M: Tree<T> + Sized + VecFunctions<T, V> + Default,
    > Tree<T> for Decoratable<T, V, M>
{
    fn new_with_compare(compare: fn(&T, &T) -> Ordering) -> Self {
        let additional_index_vec = AdditionalIndexVec::default();

        let mut dec_tree = Decoratable {
            base: M::default(),
            root: None,
            indexes: additional_index_vec,
            compare,
            v: std::marker::PhantomData,
        };

        dec_tree.fill_indexes();

        dec_tree
    }

    fn push(&mut self, value: T) -> usize {
        let index = self.base.push(value.clone());
        self.push_index(index);
        if self.root.is_none() {
            self.root = Some(index);
        } else {
            self.root = self.add_from_root(value, self.root.unwrap(), index);
        }

        index
    }

    fn find(&mut self, value: &T) -> Option<usize> {
        let mut current_index = self.root;
        while current_index.is_some() {
            if let Some(node_value) = self
                .base
                .get_nodes_mut()
                .get_value_mut(current_index.unwrap())
            {
                if (self.compare)(value, node_value) == Ordering::Less {
                    current_index =
                        self.indexes[current_index.unwrap()].left_index;
                } else if (self.compare)(value, node_value) == Ordering::Greater
                {
                    current_index =
                        self.indexes[current_index.unwrap()].right_index;
                } else {
                    return self.indexes[current_index.unwrap()].index;
                }
            } else {
                return None;
            }
        }
        None
    }

    fn remove_by_value(&mut self, value: &T) -> Option<T> {
        if self.len() == 0 {
            return None;
        } else if self.len() == 1 {
            self.base.remove_by_value(value);
            self.indexes[0] = TreeIndex::default();
            self.root = None;
            return Some(value.clone());
        }
        self.root = self.remove_from_root(value, self.root.unwrap());
        self.base.remove_by_value(value);

        Some(value.clone())
    }

    fn pop(&self) -> Option<T> {
        todo!()
    }

    fn is_empty(&self) -> bool {
        self.base.is_empty()
    }

    fn len(&self) -> usize {
        self.base.len()
    }
}

impl<
        T: Default + Clone + Ord,
        V: TreeVec<T> + Indexes<T> + Levels + Sized,
        M: Tree<T> + Sized + VecFunctions<T, V> + Default,
    > VecFunctions<T, V> for Decoratable<T, V, M>
{
    fn get(&mut self, index: usize) -> Option<T> {
        self.base.get(index)
    }

    fn get_nodes_mut(&mut self) -> &mut V {
        self.base.get_nodes_mut()
    }

    fn get_nodes(&self) -> &V {
        self.base.get_nodes()
    }

    fn get_root_index(&self) -> Option<usize> {
        self.root
    }

    fn remove_by_index(&mut self, index: usize) -> Option<T> {
        if self.len() == 0 {
            return None;
        } else if self.len() == 1 {
            let value = self.base.get(index).unwrap();
            self.base.remove_by_index(index);
            self.indexes[0] = TreeIndex::default();
            self.root = None;
            return Some(value);
        }
        let value = self.base.get(index).unwrap();
        self.root = self.remove_from_root(&value, self.root.unwrap());
        self.base.remove_by_index(index);

        Some(value)
    }
}

impl<
        T: Clone,
        V: TreeVec<T> + Sized + Clone,
        M: Tree<T> + Sized + VecFunctions<T, V> + Clone,
    > Clone for Decoratable<T, V, M>
{
    fn clone(&self) -> Self {
        Decoratable {
            base: self.base.clone(),
            root: self.root,
            indexes: self.indexes.clone(),
            compare: self.compare,
            v: std::marker::PhantomData,
        }
    }
}

impl<T, V, M> Default for Decoratable<T, V, M>
where
    T: Default + Clone + Ord,
    V: TreeVec<T> + Levels + Sized,
    M: Tree<T> + Sized + VecFunctions<T, V> + Default,
{
    fn default() -> Self {
        let additional_index_vec = AdditionalIndexVec::default();

        let mut dec_tree = Decoratable {
            base: M::default(),
            root: None,
            indexes: additional_index_vec,
            compare: |a: &T, b: &T| a.cmp(b),
            v: std::marker::PhantomData,
        };

        dec_tree.fill_indexes();

        dec_tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::tree::{
        object::BalancedTree, vectors::default_tree_vec::DefaultTreeVec,
    };

    #[test]
    fn test_decoratable_balanced_tree_new() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::default();

        tree.push(1);
        tree.push(2);
        tree.push(3);

        let dec_tree = Decoratable::<
            u64,
            DefaultTreeVec<u64>,
            BalancedTree<u64, DefaultTreeVec<u64>>,
        >::new_with_existing(tree, |a, b| b.cmp(&a));

        assert_eq!(dec_tree.base.len(), 3);
        assert_eq!(dec_tree.indexes.len(), 3);
        assert_eq!(dec_tree.root, Some(1));
        assert_eq!(dec_tree.indexes[1].left_index, Some(2));
        assert_eq!(dec_tree.indexes[1].right_index, Some(0));
    }

    #[test]
    fn test_decoratable_tree_new_empty() {
        let dec_tree = Decoratable::<
            u64,
            DefaultTreeVec<u64>,
            BalancedTree<u64, DefaultTreeVec<u64>>,
        >::default();

        assert_eq!(dec_tree.base.len(), 0);
        assert_eq!(dec_tree.indexes.len(), 0);
        assert_eq!(dec_tree.root, None);
    }

    #[test]
    fn test_decoratable_balanced_tree_push() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::default();

        tree.push(1);
        tree.push(2);

        let mut dec_tree = Decoratable::<
            u64,
            DefaultTreeVec<u64>,
            BalancedTree<u64, DefaultTreeVec<u64>>,
        >::new_with_existing(tree, |a, b| b.cmp(&a));

        dec_tree.push(3);

        assert_eq!(dec_tree.base.len(), 3);
        assert_eq!(dec_tree.indexes.len(), 3);
        assert_eq!(dec_tree.root, Some(1));
        assert_eq!(dec_tree.indexes[1].left_index, Some(2));
        assert_eq!(dec_tree.indexes[1].right_index, Some(0));

        assert_eq!(dec_tree.base.get_root_index(), Some(1));
        assert_eq!(dec_tree.base.get_nodes().get_index(1).left_index, Some(0));
        assert_eq!(dec_tree.base.get_nodes().get_index(1).right_index, Some(2));
    }

    #[test]
    fn test_decoratable_tree_push_empty() {
        let mut dec_tree = Decoratable::<
            u64,
            DefaultTreeVec<u64>,
            BalancedTree<u64, DefaultTreeVec<u64>>,
        >::new_with_compare(|a, b| b.cmp(&a));

        dec_tree.push(3);

        assert_eq!(dec_tree.base.len(), 1);
        assert_eq!(dec_tree.indexes.len(), 1);
        assert_eq!(dec_tree.root, Some(0));
        assert_eq!(dec_tree.indexes[0].left_index, None);
        assert_eq!(dec_tree.indexes[0].right_index, None);

        assert_eq!(dec_tree.base.get_root_index(), Some(0));

        dec_tree.push(2);

        assert_eq!(dec_tree.base.len(), 2);
        assert_eq!(dec_tree.indexes.len(), 2);
        assert_eq!(dec_tree.root, Some(0));
        assert_eq!(dec_tree.indexes[0].left_index, None);
        assert_eq!(dec_tree.indexes[0].right_index, Some(1));
    }

    #[test]
    fn test_decoratable_balanced_tree_find() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::default();

        tree.push(1);
        tree.push(2);
        tree.push(3);

        let mut dec_tree = Decoratable::<
            u64,
            DefaultTreeVec<u64>,
            BalancedTree<u64, DefaultTreeVec<u64>>,
        >::new_with_existing(tree, |a, b| b.cmp(&a));

        assert_eq!(dec_tree.find(&1), Some(0));
        assert_eq!(dec_tree.find(&2), Some(1));
        assert_eq!(dec_tree.find(&3), Some(2));
    }

    #[test]
    fn test_decoratable_balanced_tree_remove_by_value() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::default();

        tree.push(1);
        tree.push(2);
        tree.push(3);

        let mut dec_tree = Decoratable::<
            u64,
            DefaultTreeVec<u64>,
            BalancedTree<u64, DefaultTreeVec<u64>>,
        >::new_with_existing(tree, |a, b| b.cmp(&a));

        assert_eq!(dec_tree.remove_by_value(&1), Some(1));
        assert_eq!(dec_tree.remove_by_value(&2), Some(2));
        assert_eq!(dec_tree.remove_by_value(&3), Some(3));
    }

    #[test]
    fn test_decoratable_balanced_tree_remove_by_index() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::default();

        tree.push(1);
        tree.push(2);
        tree.push(3);

        let mut dec_tree = Decoratable::<
            u64,
            DefaultTreeVec<u64>,
            BalancedTree<u64, DefaultTreeVec<u64>>,
        >::new_with_existing(tree, |a, b| b.cmp(&a));

        assert_eq!(dec_tree.remove_by_index(0), Some(1));
        assert_eq!(dec_tree.remove_by_index(1), Some(2));
        assert_eq!(dec_tree.remove_by_index(2), Some(3));

        assert_eq!(dec_tree.get(0), None);
    }

    #[test]
    fn test_decoratable_balanced_tree_remove_root_values() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::default();

        tree.push(1);

        let mut dec_tree = Decoratable::<
            u64,
            DefaultTreeVec<u64>,
            BalancedTree<u64, DefaultTreeVec<u64>>,
        >::new_with_existing(tree, |a, b| b.cmp(&a));

        assert_eq!(dec_tree.remove_by_value(&1), Some(1));
        assert_eq!(dec_tree.len(), 0);
        assert_eq!(dec_tree.get(0), None);

        assert_eq!(dec_tree.remove_by_value(&1), None);
    }

    #[test]
    fn test_decoratable_balanced_tree_remove_root_indexes() {
        let mut tree = BalancedTree::<u64, DefaultTreeVec<u64>>::default();

        tree.push(1);

        let mut dec_tree = Decoratable::<
            u64,
            DefaultTreeVec<u64>,
            BalancedTree<u64, DefaultTreeVec<u64>>,
        >::new_with_existing(tree, |a, b| b.cmp(&a));

        assert_eq!(dec_tree.remove_by_index(0), Some(1));
        assert_eq!(dec_tree.len(), 0);
        assert_eq!(dec_tree.find(&1), None);
        assert_eq!(dec_tree.get(0), None);

        assert_eq!(dec_tree.remove_by_index(0), None);
    }
}
