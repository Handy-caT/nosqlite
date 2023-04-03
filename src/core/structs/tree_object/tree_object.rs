use std::cmp::Ordering;
use crate::core::structs::tree_object::index_vector::TreeIndex;
use crate::core::structs::tree_vectors::optimized_tree_vector::OptimizedTreeVec;

pub trait TreeObject<T> {
    fn pop(&mut self) -> Option<T>;
    fn push(&mut self, item: T);
    fn peek(&self) -> Option<T>;

    fn get(&self, index: u64) -> Option<T>;
    fn find(&self, item: T) -> Option<u64>;
    fn remove(&mut self, index: u64) -> Option<T>;

    fn is_empty(&self) -> bool;
    fn len(&self) -> u64;
}

pub struct DecoratableTreeObject<T, M: TreeObject<T> + Sized> {
    base: M,
    root: i32,
    indexes: OptimizedTreeVec<TreeIndex>,
    compare: fn(&T, &T) -> Ordering,
}

impl<T, M: TreeObject<T> + Sized> DecoratableTreeObject<T, M> {
    pub fn new(base: M, compare: fn(&T, &T) -> Ordering) -> Self {
        Self {
            base,
            root: -1,
            indexes: OptimizedTreeVec::new(),
            compare,
        }
    }

    pub fn get_base(&self) -> &M {
        &self.base
    }

    pub fn get_base_mut(&mut self) -> &mut M {
        &mut self.base
    }

    pub fn get_indexes(&self) -> &OptimizedTreeVec<TreeIndex> {
        &self.indexes
    }

    pub fn get_indexes_mut(&mut self) -> &mut OptimizedTreeVec<TreeIndex> {
        &mut self.indexes
    }

    pub fn get_compare(&self) -> fn(&T, &T) -> Ordering {
        self.compare
    }

    pub fn get_compare_mut(&mut self) -> &mut fn(&T, &T) -> Ordering {
        &mut self.compare
    }
}

impl <T: Copy + Default, M: TreeObject<T> + Sized> TreeObject<T> for DecoratableTreeObject<T, M> {
    fn pop(&mut self) -> Option<T> {
        todo!()
    }

    fn push(&mut self, item: T) {
        todo!()
    }

    fn peek(&self) -> Option<T> {
        todo!()
    }

    fn get(&self, index: u64) -> Option<T> {
        todo!()
    }

    fn find(&self, item: T) -> Option<u64> {
        todo!()
    }

    fn remove(&mut self, index: u64) -> Option<T> {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn len(&self) -> u64 {
        todo!()
    }
}