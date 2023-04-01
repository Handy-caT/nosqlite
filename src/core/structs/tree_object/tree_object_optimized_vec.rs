use crate::core::structs::tree_object::tree_object::TreeObject;
use crate::core::structs::tree_vectors::optimized_tree_vector::OptimizedTreeVec;
use crate::core::structs::tree_vectors::tree_vec::TreeVec;


impl <T: Default + Copy + PartialOrd> TreeObject<T> for OptimizedTreeVec<T> {
    fn pop(&mut self) -> Option<T> {
        let size = TreeVec::len(self);
        return if size == 0 {
            None
        } else {
            let index = size - 1;
            let item = self.get(index as i32);
            TreeVec::remove(self, index as i32);
            Some(item.value)
        }
    }

    fn push(&mut self, item: T) {
        self.add(item);
    }

    fn peek(&self) -> Option<T> {
        let size = TreeVec::len(self);
        return if size == 0 {
            None
        } else {
            let index = size - 1;
            let item = self.peek_node(index as i32);
            match item {
                Some(value) => Some(value.value),
                None => None,
            }
        }
    }

    fn get(&self, index: u64) -> Option<T> {
        let item = self.peek_node(index as i32);
        match item {
            Some(value) => Some(value.value),
            None => None,
        }
    }

    fn find(&self, item: T) -> Option<u64> {
        let mut index = 0;
        let mut found = false;
        while index < TreeVec::len(self) && !found {
            let value = self.get(index as u64);
            match value {
                Some(value) => {
                    if value == item {
                        found = true;
                    } else {
                        index += 1;
                    }
                }
                None => {
                    index += 1;
                }
            }
        }
        if found {
            Some(index as u64)
        } else {
            None
        }
    }

    fn remove(&mut self, index: u64) -> Option<T> {
        if index >= TreeVec::len(self) as u64 {
            return None;
        } else if self.is_empty_index(index as i32) {
            return None;
        }
        let item = self.get(index as i32);
        TreeVec::remove(self, index as i32);

        Some(item.value)
    }

    fn is_empty(&self) -> bool {
        return TreeVec::len(self) == 0;
    }

    fn len(&self) -> u64 {
        TreeVec::len(self) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_object_optimized_push() {
        let mut vec = OptimizedTreeVec::new();

        vec.push(1);
        vec.push(2);

        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.pop(), Some(1));
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn test_tree_object_optimized_pop() {
        let mut vec = OptimizedTreeVec::new();

        vec.push(1);
        vec.push(2);

        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.pop(), Some(1));
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn test_tree_object_optimized_peek() {
        let mut vec = OptimizedTreeVec::new();

        vec.push(1);
        vec.push(2);

        assert_eq!(vec.peek(), Some(2));
        assert_eq!(vec.peek(), Some(2));
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.pop(), Some(1));
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn test_tree_object_optimized_get() {
        let mut vec = OptimizedTreeVec::new();

        vec.push(1);
        vec.push(2);

        assert_eq!(vec.get(0), Some(1));
        assert_eq!(vec.get(1), Some(2));
        assert_eq!(vec.get(2), None);
    }

    #[test]
    fn test_tree_object_optimized_find() {
        let mut vec = OptimizedTreeVec::new();

        vec.push(1);
        vec.push(2);

        assert_eq!(vec.find(1), Some(0));
        assert_eq!(vec.find(2), Some(1));
        assert_eq!(vec.find(3), None);
    }

    #[test]
    fn test_tree_object_optimized_remove() {
        let mut vec = OptimizedTreeVec::new();

        vec.push(1);
        vec.push(2);

        assert_eq!(TreeObject::remove(&mut vec,0), Some(1));
        assert_eq!(TreeObject::remove(&mut vec,0), None);
    }

    #[test]
    fn test_tree_object_optimized_is_empty() {
        let mut vec = OptimizedTreeVec::new();

        assert_eq!(vec.is_empty(), true);
        vec.push(1);
        assert_eq!(vec.is_empty(), false);
        vec.pop();
        assert_eq!(vec.is_empty(), true);
    }
}