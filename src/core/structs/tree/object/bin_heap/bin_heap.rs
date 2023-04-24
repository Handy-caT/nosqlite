use std::arch::x86_64::_mm256_testc_pd;
use std::cmp::Ordering;
use crate::core::structs::tree::object::tree_object::TreeObject;
use crate::core::structs::tree::vectors::backward_tree_vec::BackwardsTreeVec;
use crate::core::structs::tree::vectors::tree_vec::{BackwardTreeVec, TreeVec};

// pub struct BinHeap<T> {
//     root: i32,
//     data: ,
//     compare: fn(&T, &T) -> Ordering,
// }
//
// impl <T: Default + PartialOrd + Copy> BinHeap<T> {
//     pub fn new() -> Self {
//         BinHeap {
//             root: -1,
//             data: Vec::new(),
//             compare: |a, b| a.partial_cmp(b).unwrap(),
//         }
//     }
//
//     pub fn new_with_compare(compare: fn(&T, &T) -> Ordering) -> Self {
//         BinHeap {
//             root: -1,
//             data: Vec::new(),
//             compare,
//         }
//     }
// }
//
// impl <T: Default + PartialOrd + Copy> TreeObject<T> for BinHeap<T> {
//     fn push(&mut self, value: T) -> i32 {
//         todo!()
//     }
//
//     fn find(&mut self, value: T) -> Option<i32> {
//         todo!()
//     }
//
//     fn remove_by_value(&mut self, value: T) -> Option<T> {
//         todo!()
//     }
//
//     fn is_empty(&self) -> bool {
//         todo!()
//     }
//
//     fn len(&self) -> usize {
//         todo!()
//     }
// }