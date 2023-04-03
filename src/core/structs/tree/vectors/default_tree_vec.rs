use std::ops::{Index, IndexMut};
use crate::core::structs::tree::tree_node::TreeNode;
use crate::core::structs::tree::vectors::tree_vec::TreeVec;

pub struct DefaultTreeVec<T: Sized> {
    data: Vec<TreeNode<T>>,
    empty: Vec<u64>,
    length: u64,
}

impl <T: Default + Copy> DefaultTreeVec<T> {
    pub fn new() -> DefaultTreeVec<T> {
        DefaultTreeVec {
            data: Vec::new(),
            empty: Vec::new(),
            length: 0,
        }
    }
}

impl <T> Index<usize> for DefaultTreeVec<T> {
    type Output = TreeNode<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl <T> IndexMut<usize> for DefaultTreeVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}

impl <T: Default + Copy> TreeVec<T> for DefaultTreeVec<T> {
    fn push(&mut self, value: T) -> i32 {
        let index = if self.empty.len() > 0 {
            self.empty.pop().unwrap()
        } else {
            self.length += 1;
            self.data.len() as u64
        };

        let node = TreeNode::new_with_index(value, index as i32);

        if index == self.data.len() as u64 {
            self.data.push(node);
        } else {
            self.data[index as usize] = node;
        }

        index as i32
    }

    fn get(&mut self, index: i32) -> Option<TreeNode<T>> {
        let item = self.data.get(index as usize);
        return if item.is_none() {
            None
        } else {
            let item = item.unwrap();
            if item.indexes.index == -1 {
                None
            } else {
                Some(*item)
            }
        }
    }

    fn remove(&mut self, index: i32) -> Option<TreeNode<T>> {
        let item = self.data.get(index as usize);
        if item.is_none() {
            return None;
        }

        let item = *item.unwrap();
        if item.indexes.index == -1 {
            return None;
        }

        self.data[index as usize] = TreeNode::default();
        self.empty.push(index as u64);

        if index as u64 == self.length - 1 {
            self.length -= 1;
        }

        Some(item)
    }

    fn len(&self) -> usize {
        self.length as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_vec_new() {
        let tree_vec = DefaultTreeVec::<u64>::new();
        assert_eq!(tree_vec.data.len(), 0);
        assert_eq!(tree_vec.empty.len(), 0);
    }

    #[test]
    fn test_default_vec_add() {
        let mut tree_vec = DefaultTreeVec::<u64>::new();
        tree_vec.push(1);
        assert_eq!(tree_vec.data.len(), 1);
        assert_eq!(tree_vec.empty.len(), 0);
        assert_eq!(tree_vec.data[0].value, 1);
    }

    #[test]
    fn test_default_vec_get() {
        let mut tree_vec = DefaultTreeVec::<u64>::new();
        tree_vec.push(1);

        let item = tree_vec.get(0);

        assert_eq!(item.is_some(), true);
        assert_eq!(item.unwrap().value, 1)
    }


    #[test]
    fn test_default_vec_add_remove() {
        let mut tree_vec = DefaultTreeVec::<u64>::new();
        tree_vec.push(1);
        tree_vec.push(2);
        tree_vec.push(3);
        tree_vec.remove(1);

        assert_eq!(tree_vec.data.len(), 3);
        assert_eq!(tree_vec.empty.len(), 1);
        assert_eq!(tree_vec.empty[0], 1);

        tree_vec.push(6);

        assert_eq!(tree_vec.data.len(), 3);
        assert_eq!(tree_vec.empty.len(), 0);

        assert_eq!(tree_vec.data[0].value, 1);
        assert_eq!(tree_vec.data[1].value, 6);
        assert_eq!(tree_vec.data[2].value, 3);
    }

    #[test]
    fn test_default_vec_get_removed() {
        let mut vec = DefaultTreeVec::<i32>::new();
        let index = vec.push(1);

        vec.push(2);
        vec.push(3);

        assert_eq!(vec.remove(index).is_some(), true);
        assert_eq!(vec.get(index).is_none(), true);
    }

    #[test]
    fn test_default_vec_remove() {
        let mut vec = DefaultTreeVec::<i32>::new();
        let index = vec.push(1);

        vec.push(2);
        vec.push(3);

        assert_eq!(vec.remove(index).is_some(), true);

        assert_eq!(vec.data.len(), 3);
        assert_eq!(vec.empty.len(), 1);
        assert_eq!(vec.empty[0], 0);

        assert_eq!(vec.remove(index).is_none(), true);
    }

    #[test]
    fn test_default_vec_remove_last() {
        let mut vec = DefaultTreeVec::<i32>::new();

        vec.push(1);
        vec.push(2);
        let index = vec.push(3);

        assert_eq!(vec.remove(index).is_some(), true);
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn test_default_vec_get_out_of_bounds() {
        let mut vec = DefaultTreeVec::<i32>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.get(-1).is_none(), true);
        assert_eq!(vec.get(5).is_none(), true);
    }

    #[test]
    fn test_default_vec_remove_out_of_bounds() {
        let mut vec = DefaultTreeVec::<i32>::new();

        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.remove(-1).is_none(), true);
        assert_eq!(vec.remove(5).is_none(), true);
        assert_eq!(vec.empty.len(), 0);
    }

}