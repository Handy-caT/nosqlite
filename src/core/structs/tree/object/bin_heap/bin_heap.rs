use std::cmp::Ordering;
use crate::core::structs::tree::object::tree_object::{TreeObject, TreeObjectVec};
use crate::core::structs::tree::vectors::normalized_tree_vec::NormalizedTreeVector;
use crate::core::structs::tree::vectors::tree_vec::{TreeVec};

/// Struct that represents a binary heap.
/// It is a tree-based data structure that satisfies the heap property:
/// It compares the parent node with its children and the parent node is always greater than its children.
/// It is implemented using a vector.
/// It is using custom NormalizedTreeVector struct that represents the tree, where childrens are on 2*i + 1 and 2*i + 2 positions.
pub struct BinHeap<T> {
    data: NormalizedTreeVector<T>,
    compare: fn(&T, &T) -> Ordering,
}

/// Implementation of BinHeap struct
impl <T: Default + PartialOrd + Copy> BinHeap<T> {
    /// Creates a new BinHeap struct
    /// With default compare function
    /// So by default BinHeap is max heap
    pub fn new() -> Self {
        BinHeap {
            data: NormalizedTreeVector::new(),
            compare: |a, b| a.partial_cmp(b).unwrap(),
        }
    }

    /// Creates a new BinHeap struct
    /// With custom compare function
    pub fn new_with_compare(compare: fn(&T, &T) -> Ordering) -> Self {
        BinHeap {
            data: NormalizedTreeVector::new(),
            compare,
        }
    }

    /// Function that heapifies the object
    /// It is used when we remove the root node
    fn heapify(&mut self, index: i32) {
        let left_index = index * 2 + 1;
        let right_index = index * 2 + 2;

        let mut largest_index = index;

        if left_index < self.data.len() as i32 && (self.compare)(&self.data.get(left_index).unwrap().value,
                                                                 &self.data.get(index).unwrap().value) == Ordering::Greater {
            largest_index = left_index;
        }

        if right_index < self.data.len() as i32 && (self.compare)(&self.data.get(right_index).unwrap().value,
                                                                  &self.data.get(largest_index).unwrap().value) == Ordering::Greater {
            largest_index = right_index;
        }

        if largest_index != index {
            self.data.swap_indexes(index, largest_index);
            self.heapify(largest_index);
        }
    }

    /// Function that returns the value of the root item
    pub fn peek_max(&mut self) -> Option<T> {
        return if self.data.len() == 0 {
            None
        } else {
            Some(self.data.get(0).unwrap().value)
        }
    }

    /// Function that removes the root item and returns it
    pub fn get_max(&mut self) -> Option<T> {
        return if self.data.len() == 0 {
            None
        } else {
            let max = self.peek_max();
            self.data.swap_indexes(0, self.data.len() as i32 - 1);
            self.data.remove(self.data.len() as i32 - 1);
            self.heapify(0);

            max
        }
    }
}

/// Implementation of TreeObject trait for BinHeap struct
/// It is used for tree operations and to use as part of TreeDecorator
impl <T: Default + PartialOrd + Copy> TreeObject<T> for BinHeap<T> {
    /// Function that pushes new item to the heap
    fn push(&mut self, value: T) -> i32 {
        if self.data.len() == 0 {
            let index = self.data.push(value);
            return index;
        }

        let mut index = self.data.push(value);
        let result_index = index;
        let mut parent_index = NormalizedTreeVector::<T>::get_parent_index(index);

        while index > 0 && (self.compare)(&self.data.get(index).unwrap().value, &self.data.get(parent_index).unwrap().value) == Ordering::Greater {
            self.data.swap_indexes(index, parent_index);
            index = parent_index;
            parent_index = NormalizedTreeVector::<T>::get_parent_index(index);
        }
        result_index
    }

    /// Function that returns the index of the element with the given value
    /// If there is no such element, it returns None
    fn find(&mut self, value: T) -> Option<i32> {
        if self.data.len() == 0 {
            return None;
        } else {
            let mut index = 0;
            let mut found = false;

            while index < self.data.len() as i32 && !found {
                if self.data.get(index).unwrap().value == value {
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

    /// Function that removes the element by the given value
    /// If there is no such element, it returns None
    fn remove_by_value(&mut self, value: T) -> Option<T> {
        if self.data.len() == 0 {
            return None;
        } else {
            let index = self.find(value);
            if index.is_none() {
                return None;
            } else {
                let index = index.unwrap();
                self.remove_by_index(index)
            }
        }
    }

    /// Function that checks if the heap is empty
    fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    /// Function that returns the length of the heap
    fn len(&self) -> usize {
        self.data.len()
    }
}

/// Implementation of TreeObjectVec trait for BinHeap struct
impl <T: Default + PartialOrd + Copy> TreeObjectVec<T, NormalizedTreeVector<T>> for BinHeap<T> {
    /// Function that returns the value of the element by the given index
    /// If the index is incorrect, it returns None
    /// The index represents index in heap not in underlying vector
    fn get(&mut self, index: i32) -> Option<T> {
        if index < self.data.len() as i32 && index >= 0 {
            Some(self.data.get(index).unwrap().value)
        } else {
            None
        }
    }

    /// Function that returns the mutable reference to it's vector
    fn get_nodes_mut(&mut self) -> &mut NormalizedTreeVector<T> {
        &mut self.data
    }

    /// Function that returns the reference to it's vector
    fn get_nodes(&self) -> &NormalizedTreeVector<T> {
        &self.data
    }

    /// Function that returns the index of the root element
    fn get_root_index(&self) -> i32 {
        0
    }

    /// Function that removes the element by the given index
    fn remove_by_index(&mut self, index: i32) -> Option<T> {
        if index < self.data.len() as i32 || index >= 0 {
            let value = self.data.get(index).unwrap().value;
            self.data.swap_indexes(index, self.data.len() as i32 - 1);
            self.data.remove(self.data.len() as i32 - 1);
            self.heapify(index);

            Some(value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::structs::tree::object::bin_heap::bin_heap::BinHeap;
    use crate::core::structs::tree::object::tree_object::{TreeObject, TreeObjectVec};
    use crate::core::structs::tree::vectors::optimized_tree_vec::INITIAL_LEVELS;
    use crate::core::structs::tree::vectors::tree_vec::{TreeVec, TreeVecLevels};

    #[test]
    fn test_bin_heap_new() {
        let heap = BinHeap::<u64>::new();

        assert_eq!(heap.data.len(), 0);
        assert_eq!(heap.data.get_allocated_levels(), INITIAL_LEVELS);
        assert_eq!(heap.is_empty(), true);
    }

    #[test]
    fn test_bin_heap_push() {
        let mut heap = BinHeap::<u64>::new();

        heap.push(1);
        heap.push(2);
        heap.push(3);

        assert_eq!(heap.data.len(), 3);
        assert_eq!(heap.data[0], 1);
        assert_eq!(heap.data[1], 2);
        assert_eq!(heap.data[2], 3);

        assert_eq!(heap.data.get(0).unwrap().indexes.index, 2);
        assert_eq!(heap.data.get(1).unwrap().indexes.index, 0);
        assert_eq!(heap.data.get(2).unwrap().indexes.index, 1);
    }

    #[test]
    fn test_bin_heap_get_max() {
        let mut heap = BinHeap::<u64>::new();

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
        let mut heap = BinHeap::<u64>::new();

        heap.push(1);
        heap.push(2);
        heap.push(3);

        assert_eq!(heap.peek_max().unwrap(), 3);
        assert_eq!(heap.data.len(), 3);
    }

    #[test]
    fn test_bin_heap_find() {
        let mut heap = BinHeap::<u64>::new();

        heap.push(1);
        heap.push(2);
        heap.push(3);

        assert_eq!(heap.find(1).unwrap(), 1);
        assert_eq!(heap.find(2).unwrap(), 2);
        assert_eq!(heap.find(3).unwrap(), 0);
        assert_eq!(heap.find(4), None);
        assert_eq!(heap.find(0), None)
    }

    #[test]
    fn test_bin_heap_remove_by_value() {
        let mut heap = BinHeap::<u64>::new();

        heap.push(1);
        heap.push(2);
        heap.push(3);
        heap.push(4);

        assert_eq!(heap.remove_by_value(2).unwrap(), 2);
        assert_eq!(heap.data.len(), 3);
        assert_eq!(heap.peek_max().is_some(), true);
        assert_eq!(heap.peek_max().unwrap(), 4);
    }

    #[test]
    fn test_bin_heap_remove_by_index() {
        let mut heap = BinHeap::<u64>::new();

        heap.push(1);
        heap.push(2);
        heap.push(3);
        heap.push(4);

        assert_eq!(heap.remove_by_index(2).unwrap(), 2);
        assert_eq!(heap.data.len(), 3);
        assert_eq!(heap.peek_max().is_some(), true);
        assert_eq!(heap.peek_max().unwrap(), 4);
    }

    #[test]
    fn test_bin_heap_get() {
        let mut heap = BinHeap::<u64>::new();

        heap.push(1);
        heap.push(2);
        heap.push(3);
        heap.push(4);

        assert_eq!(heap.get(0).unwrap(), 4);
        assert_eq!(heap.get(1).unwrap(), 3);
        assert_eq!(heap.get(2).unwrap(), 2);
        assert_eq!(heap.get(3).unwrap(), 1);
        assert_eq!(heap.get(4), None);
        assert_eq!(heap.get(-1), None);
    }

     #[test]
    fn test_bin_heap_push_index() {
         let mut heap = BinHeap::<u64>::new();

         assert_eq!(heap.push(1), 0);
         assert_eq!(heap.push(2), 1);
         assert_eq!(heap.push(3), 2);
     }
}