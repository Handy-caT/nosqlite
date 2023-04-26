use std::cmp::Ordering;
use crate::core::structs::tree::object::tree_object::TreeObject;
use crate::core::structs::tree::vectors::normalized_tree_vec::NormalizedTreeVector;
use crate::core::structs::tree::vectors::tree_vec::{NormalizedTreeVecIndexes, TreeVec};

pub struct BinHeap<T> {
    data: NormalizedTreeVector<T>,
    compare: fn(&T, &T) -> Ordering,
}

impl <T: Default + PartialOrd + Copy> BinHeap<T> {
    pub fn new() -> Self {
        BinHeap {
            data: NormalizedTreeVector::new(),
            compare: |a, b| a.partial_cmp(b).unwrap(),
        }
    }

    pub fn new_with_compare(compare: fn(&T, &T) -> Ordering) -> Self {
        BinHeap {
            data: NormalizedTreeVector::new(),
            compare,
        }
    }

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

    pub fn peek_max(&mut self) -> Option<T> {
        return if self.data.len() == 0 {
            None
        } else {
            Some(self.data.get(0).unwrap().value)
        }
    }

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

impl <T: Default + PartialOrd + Copy> TreeObject<T> for BinHeap<T> {
    fn push(&mut self, value: T) -> i32 {
        if self.data.len() == 0 {
            let index = self.data.push(value);
            return index;
        }

        let mut index = self.data.push(value);
        let mut parent_index = NormalizedTreeVector::<T>::get_parent_index(index);

        while index > 0 && (self.compare)(&self.data.get(index).unwrap().value, &self.data.get(parent_index).unwrap().value) == Ordering::Greater {
            self.data.swap_indexes(index, parent_index);
            index = parent_index;
            parent_index = NormalizedTreeVector::<T>::get_parent_index(index);
        }
        parent_index
    }

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

    fn remove_by_value(&mut self, value: T) -> Option<T> {
        if self.data.len() == 0 {
            return None;
        } else {
            let index = self.find(value);
            if index.is_none() {
                return None;
            } else {
                let index = index.unwrap();
                let value = self.data.get(index).unwrap().value;
                self.data.swap_indexes(index, self.data.len() as i32 - 1);
                self.data.remove(self.data.len() as i32 - 1);
                self.heapify(index);

                Some(value)
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::structs::tree::object::bin_heap::bin_heap::BinHeap;
    use crate::core::structs::tree::object::tree_object::TreeObject;
    use crate::core::structs::tree::vectors::optimized_tree_vec::INITIAL_LEVELS;
    use crate::core::structs::tree::vectors::tree_vec::{TreeVec, TreeVecLevels};

    #[test]
    fn test_bin_heap_new() {
        let heap = BinHeap::<u64>::new();

        assert_eq!(heap.data.len(), 0);
        assert_eq!(heap.data.get_allocated_levels(), INITIAL_LEVELS);
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
    }
}