use crate::core::tree::normalized_tree_vector::NormalizedTreeVector;

struct BinHeap<T> {
    data: NormalizedTreeVector<T>,
}

impl <T: Default + Copy + Ord> BinHeap<T> {
    pub fn new() -> BinHeap<T> {
        BinHeap {
            data: NormalizedTreeVector::new(),
        }
    }

    pub fn add(&mut self, value: T) {
        if self.data.size == 0 {
            self.data.add(value);
            return;
        }

        let mut index = self.data.add(value);
        let mut parent_index = NormalizedTreeVector::<T>::get_parent_index(index);

        while self.data.get(index).value > self.data.get(parent_index).value && index > 0 {
            self.data.swap(index, parent_index);
            index = parent_index;
            parent_index = NormalizedTreeVector::<T>::get_parent_index(index);
        }

    }

    fn heapify(&mut self, index: i32) {
        let left_index = index * 2 + 1;
        let right_index = index * 2 + 2;

        let mut largest_index = index;

        if left_index < self.data.size as i32 && self.data.get(left_index).value > self.data.get(index).value {
            largest_index = left_index;
        }

        if right_index < self.data.size as i32 && self.data.get(right_index).value > self.data.get(largest_index).value {
            largest_index = right_index;
        }

        if largest_index != index {
            self.data.swap(index, largest_index);
            self.heapify(largest_index);
        }
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
}