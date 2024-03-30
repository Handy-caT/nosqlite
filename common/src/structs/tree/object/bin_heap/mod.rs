use crate::structs::tree::{
    object::tree::{Tree, VecFunctions},
    vectors::{normalized_tree_vec::NormalizedTreeVector, tree_vec::TreeVec},
};
use std::cmp::Ordering;

/// Struct that represents a binary heap.
/// It is a tree-based data structure that satisfies the heap property:
/// It compares the parent node with its children and the parent
/// node is always greater than its children.
/// It is implemented using a vector.
/// It is using custom [`NormalizedTreeVector`] struct that represents the tree,
/// where childrens are on 2*i + 1 and 2*i + 2 positions.
#[derive(Debug)]
pub struct BinHeap<T> {
    /// [`NormalizedTreeVector`] that represents the tree
    /// It is used to store the data
    data: NormalizedTreeVector<T>,
    /// Compare function that is used to compare the nodes
    compare: fn(&T, &T) -> Ordering,
}

/// Implementation of [`BinHeap`] struct
impl<T: Default + PartialOrd + Clone> BinHeap<T> {
    /// Function that heapifies the object
    /// It is used when we remove the root node
    /// # Arguments
    /// * `index` - Index of the node that we will start heapifying
    fn heapify(&mut self, index: usize) {
        let left_index = index * 2 + 1;
        let right_index = index * 2 + 2;

        let mut largest_index = index;

        if left_index < self.data.len()
            && (self.compare)(
                &self.data.get(left_index).unwrap().value,
                &self.data.get(index).unwrap().value,
            ) == Ordering::Greater
        {
            largest_index = left_index;
        }

        if right_index < self.data.len()
            && (self.compare)(
                &self.data.get(right_index).unwrap().value,
                &self.data.get(largest_index).unwrap().value,
            ) == Ordering::Greater
        {
            largest_index = right_index;
        }

        if largest_index != index {
            self.data.swap_indexes(index, largest_index);
            self.heapify(largest_index);
        }
    }

    /// Function that returns the value of the root item
    /// It does not remove the root item
    /// If there is no root item, it returns None
    /// # Returns
    /// * `Option<T>` - Value of the root item
    pub fn peek_max(&mut self) -> Option<T> {
        if self.data.len() == 0 {
            None
        } else {
            Some(self.data.get(0).unwrap().value)
        }
    }

    /// Function that removes the root item and returns it
    /// If there is no root item, it returns None
    /// # Returns
    /// * `Option<T>` - Value of the root item
    pub fn get_max(&mut self) -> Option<T> {
        if self.data.len() == 0 {
            None
        } else {
            let max = self.peek_max();
            self.data.swap_indexes(0, self.data.len() - 1);
            self.data.remove(self.data.len() - 1);
            self.heapify(0);

            max
        }
    }
}

/// Implementation of [`Tree`] trait for [`BinHeap`] struct
/// It is used for tree operations and to use as part of [`TreeDecorator`]
impl<T: Default + PartialOrd + Clone> Tree<T> for BinHeap<T> {
    fn new_with_compare(compare: fn(&T, &T) -> Ordering) -> Self {
        BinHeap {
            data: NormalizedTreeVector::new(),
            compare,
        }
    }

    fn push(&mut self, value: T) -> usize {
        if self.data.len() == 0 {
            let index = self.data.push(value);
            return index;
        }

        let mut index = self.data.push(value);
        let result_index = index;
        // We can unwrap because index is never 0
        let mut parent_index =
            NormalizedTreeVector::<T>::get_parent_index(index);

        while parent_index.is_some()
            && (self.compare)(
                &self.data.get(index).unwrap().value,
                &self.data.get(parent_index.unwrap()).unwrap().value,
            ) == Ordering::Greater
        {
            self.data.swap_indexes(index, parent_index.unwrap());
            index = parent_index.unwrap();
            // We can unwrap because index is never 0
            parent_index = NormalizedTreeVector::<T>::get_parent_index(index);
        }
        result_index
    }

    fn find(&mut self, value: &T) -> Option<usize> {
        if self.data.len() == 0 {
            None
        } else {
            let mut index = 0;
            let mut found = false;

            while index < self.data.len() && !found {
                if &self.data.get(index).unwrap().value == value {
                    found = true;
                } else {
                    index += 1;
                }
            }

            if found {
                Some(index)
            } else {
                None
            }
        }
    }

    fn remove_by_value(&mut self, value: &T) -> Option<T> {
        if self.data.len() == 0 {
            None
        } else {
            let index = self.find(value);
            if let Some(index) = index {
                self.remove_by_index(index)
            } else {
                None
            }
        }
    }

    fn pop(&self) -> Option<T> {
        todo!()
    }

    fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

/// Implementation of [`TreeObjectVec`] trait for [`BinHeap`] struct
impl<T: Default + PartialOrd + Clone> VecFunctions<T, NormalizedTreeVector<T>>
    for BinHeap<T>
{
    fn get(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            Some(self.data.get(index).unwrap().value)
        } else {
            None
        }
    }

    fn get_nodes_mut(&mut self) -> &mut NormalizedTreeVector<T> {
        &mut self.data
    }

    fn get_nodes(&self) -> &NormalizedTreeVector<T> {
        &self.data
    }

    fn get_root_index(&self) -> Option<usize> {
        Some(0)
    }

    fn remove_by_index(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            let value = self.data.get(index).unwrap().value;
            self.data.swap_indexes(index, self.data.len() - 1);
            self.data.remove(self.data.len() - 1);
            self.heapify(index);

            Some(value)
        } else {
            None
        }
    }
}

impl<T: Clone> Clone for BinHeap<T> {
    fn clone(&self) -> Self {
        BinHeap {
            data: self.data.clone(),
            compare: self.compare,
        }
    }
}

impl<T: Clone + PartialOrd + Default> Default for BinHeap<T> {
    fn default() -> Self {
        BinHeap {
            data: NormalizedTreeVector::new(),
            compare: |a, b| a.partial_cmp(&b).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::tree::{
        object::{
            bin_heap::BinHeap,
            tree::{Tree, VecFunctions},
        },
        vectors::{
            optimized_tree_vec::INITIAL_LEVELS,
            tree_vec::{Levels, TreeVec},
        },
    };

    #[test]
    fn test_bin_heap_new() {
        let heap = BinHeap::<u64>::default();

        assert_eq!(heap.data.len(), 0);
        assert_eq!(heap.data.get_allocated_levels(), INITIAL_LEVELS);
        assert!(heap.is_empty());
    }

    #[test]
    fn test_bin_heap_push() {
        let mut heap = BinHeap::<u64>::default();

        heap.push(1);
        heap.push(2);
        heap.push(3);

        assert_eq!(heap.data.len(), 3);
        assert_eq!(heap.data[0], 1);
        assert_eq!(heap.data[1], 2);
        assert_eq!(heap.data[2], 3);

        assert_eq!(heap.data.get(0).unwrap().indexes.index, Some(2));
        assert_eq!(heap.data.get(1).unwrap().indexes.index, Some(0));
        assert_eq!(heap.data.get(2).unwrap().indexes.index, Some(1));
    }

    #[test]
    fn test_bin_heap_get_max() {
        let mut heap = BinHeap::<u64>::default();

        heap.push(1);
        heap.push(2);
        heap.push(3);

        assert_eq!(heap.get_max().unwrap(), 3);
        assert_eq!(heap.data.len(), 2);

        assert_eq!(heap.get_max().unwrap(), 2);
        assert_eq!(heap.data.len(), 1);

        assert_eq!(heap.get_max().unwrap(), 1);
        assert_eq!(heap.data.len(), 0);

        assert_eq!(heap.get_max(), None);
    }

    #[test]
    fn test_bin_heap_peek_max() {
        let mut heap = BinHeap::<u64>::default();

        heap.push(1);
        heap.push(2);
        heap.push(3);

        assert_eq!(heap.peek_max().unwrap(), 3);
        assert_eq!(heap.data.len(), 3);
    }

    #[test]
    fn test_bin_heap_find() {
        let mut heap = BinHeap::<u64>::default();

        heap.push(1);
        heap.push(2);
        heap.push(3);

        assert_eq!(heap.find(&1).unwrap(), 1);
        assert_eq!(heap.find(&2).unwrap(), 2);
        assert_eq!(heap.find(&3).unwrap(), 0);
        assert_eq!(heap.find(&4), None);
        assert_eq!(heap.find(&0), None)
    }

    #[test]
    fn test_bin_heap_remove_by_value() {
        let mut heap = BinHeap::<u64>::default();

        heap.push(1);
        heap.push(2);
        heap.push(3);
        heap.push(4);

        assert_eq!(heap.remove_by_value(&2).unwrap(), 2);
        assert_eq!(heap.data.len(), 3);
        assert!(heap.peek_max().is_some());
        assert_eq!(heap.peek_max().unwrap(), 4);
    }

    #[test]
    fn test_bin_heap_remove_by_index() {
        let mut heap = BinHeap::<u64>::default();

        heap.push(1);
        heap.push(2);
        heap.push(3);
        heap.push(4);

        assert_eq!(heap.remove_by_index(2).unwrap(), 2);
        assert_eq!(heap.data.len(), 3);
        assert!(heap.peek_max().is_some());
        assert_eq!(heap.peek_max().unwrap(), 4);
    }

    #[test]
    fn test_bin_heap_get() {
        let mut heap = BinHeap::<u64>::default();

        heap.push(1);
        heap.push(2);
        heap.push(3);
        heap.push(4);

        assert_eq!(heap.get(0).unwrap(), 4);
        assert_eq!(heap.get(1).unwrap(), 3);
        assert_eq!(heap.get(2).unwrap(), 2);
        assert_eq!(heap.get(3).unwrap(), 1);
        assert_eq!(heap.get(4), None);
    }

    #[test]
    fn test_bin_heap_push_index() {
        let mut heap = BinHeap::<u64>::default();

        assert_eq!(heap.push(1), 0);
        assert_eq!(heap.push(2), 1);
        assert_eq!(heap.push(3), 2);
    }
}
