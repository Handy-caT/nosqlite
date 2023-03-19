use std::cmp::Ordering;
use crate::core::structs::tree_vectors::normalized_tree_vector::NormalizedTreeVector;

struct BinHeap<T> {
    data: NormalizedTreeVector<T>,
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

impl <T: Default + Copy + Ord> BinHeap<T> {
    pub fn new() -> BinHeap<T> {
        BinHeap {
            data: NormalizedTreeVector::new(),
            compare: default_compare,
        }
    }

    pub fn new_with_compare(compare: fn(&T, &T) -> Ordering) -> BinHeap<T> {
        BinHeap {
            data: NormalizedTreeVector::new(),
            compare,
        }
    }

    pub fn add(&mut self, value: T) {
        if self.data.size == 0 {
            self.data.add(value);
            return;
        }

        let mut index = self.data.add(value);
        let mut parent_index = NormalizedTreeVector::<T>::get_parent_index(index);

        while (self.compare)(&self.data.get(index).value, &self.data.get(parent_index).value) == Ordering::Greater && index > 0 {
            self.data.swap(index, parent_index);
            index = parent_index;
            parent_index = NormalizedTreeVector::<T>::get_parent_index(index);
        }

    }

    fn heapify(&mut self, index: i32) {
        let left_index = index * 2 + 1;
        let right_index = index * 2 + 2;

        let mut largest_index = index;

        if left_index < self.data.size as i32 && (self.compare)(&self.data.get(left_index).value, &self.data.get(index).value) == Ordering::Greater {
            largest_index = left_index;
        }

        if right_index < self.data.size as i32 && (self.compare)(&self.data.get(right_index).value, &self.data.get(largest_index).value) == Ordering::Greater {
            largest_index = right_index;
        }

        if largest_index != index {
            self.data.swap(index, largest_index);
            self.heapify(largest_index);
        }
    }

    pub fn peek_max(&mut self) -> T {
        self.data.get(0).value
    }

    pub fn get_max(&mut self) -> T {
        let max = self.data.get(0).value;
        self.data.swap(0, self.data.size as i32 - 1);
        self.data.size -= 1;
        self.heapify(0);

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let heap = BinHeap::<u64>::new();
        assert_eq!(heap.data.size, 0);
    }

    #[test]
    fn test_add() {
        let mut heap = BinHeap::<u64>::new();
        heap.add(1);
        assert_eq!(heap.data.size, 1);
        assert_eq!(heap.data.get(0).value, 1);
    }

    #[test]
    fn test_add_many() {
        let mut heap = BinHeap::<u64>::new();
        heap.add(1);
        heap.add(2);
        heap.add(3);

        assert_eq!(heap.data.size, 3);
        assert_eq!(heap.data.get(0).value, 3);

    }

    #[test]
    fn test_get_max() {
        let mut heap = BinHeap::<u64>::new();
        heap.add(1);
        heap.add(2);
        heap.add(3);

        assert_eq!(heap.peek_max(), 3);

        assert_eq!(heap.get_max(), 3);
        assert_eq!(heap.get_max(), 2);
        assert_eq!(heap.get_max(), 1);
    }

    #[test]
    fn test_get_max_with_compare() {
        let mut heap = BinHeap::<u64>::new_with_compare(|a, b| b.cmp(a));
        heap.add(1);
        heap.add(2);
        heap.add(3);

        assert_eq!(heap.peek_max(), 1);

        assert_eq!(heap.get_max(), 1);
        assert_eq!(heap.get_max(), 2);
        assert_eq!(heap.get_max(), 3);
    }

}