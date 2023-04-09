use crate::core::structs::tree::vectors::tree_vec::TreeVec;

pub trait TreeObject<T> {
    fn push(&mut self, value: T) -> i32;
    fn find(&mut self, value: T) -> Option<i32>;
    fn remove_by_value(&mut self, value: T) -> Option<T>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
}

pub trait TreeObjectVec<T, M: TreeVec<T> + Sized> {
    fn get(&mut self, index: i32) -> Option<T>;
    fn get_nodes_mut(&mut self) -> &mut M;
    fn get_nodes(&self) -> &M;
}

pub trait TreeObjectFind<T> {
    fn find_greater_equal(&mut self, value: T) -> Option<(i32,T)>;
    fn find_less_equal(&mut self, value: T) -> Option<(i32,T)>;
}
