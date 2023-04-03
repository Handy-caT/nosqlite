use crate::core::structs::balanced_tree::BalancedTree;
use crate::core::structs::tree_object::tree_object::TreeObject;
use crate::core::structs::tree_vectors::tree_vec::TreeVec;

impl <T: Copy + Default + PartialOrd, M: TreeVec<T> + Sized> TreeObject<T> for BalancedTree<T, M> {
    fn pop(&mut self) -> Option<T> {
        return if self.size() == 0 {
            None
        } else {
            let index = self.size() - 1;
            let item = self.get_by_index(index);
            self.remove(item.value);

            Some(item.value)
        }
    }

    fn push(&mut self, item: T) {
        self.add(item);
    }

    fn peek(&self) -> Option<T> {
        None
    }

    fn get(&self, index: u64) -> Option<T> {
        None
    }

    fn find(&self, item: T) -> Option<u64> {
        None
    }

    fn remove(&mut self, index: u64) -> Option<T> {
        None
    }

    fn is_empty(&self) -> bool {
        true
    }

    fn len(&self) -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::core::structs::balanced_tree::BalancedTree;
    use crate::core::structs::tree_object::tree_object::TreeObject;
    use crate::core::structs::tree_vectors::optimized_tree_vector::OptimizedTreeVec;

    #[test]
    fn test_pop() {
        let nodes = OptimizedTreeVec::<u64>::new();

        let mut tree = BalancedTree::<u64, OptimizedTreeVec<u64>>::new(nodes);
        tree.push(1);
        tree.push(2);
        tree.push(3);

        assert_eq!(tree.pop(), Some(3));
        assert_eq!(tree.pop(), Some(2));
        assert_eq!(tree.pop(), Some(1));
        assert_eq!(tree.pop(), None);
    }
}