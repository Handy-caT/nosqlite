use std::cmp::Ordering;
use crate::core::structs::tree::tree_node::TreeNode;
use crate::core::structs::tree::vectors::tree_vec::TreeVec;

pub fn height_from_root<T: Default + PartialOrd + Copy, M: TreeVec<T> + Sized>(nodes: &mut M, root_index: i32) -> u8 {
    if root_index == -1 {
        0
    } else {
        nodes[root_index as usize].indexes.height
    }
}

pub fn bfactor<T: Default + PartialOrd + Copy, M: TreeVec<T> + Sized>(nodes: &mut M, root_index: i32) -> i8 {
    let node: TreeNode<T> = nodes[root_index as usize];
    height_from_root(nodes,node.indexes.right_index) as i8 - height_from_root(nodes,node.indexes.left_index) as i8
}

pub fn fix_height<T: Default + PartialOrd + Copy, M: TreeVec<T> + Sized>(nodes: &mut M, root_index: i32) {
    let node: TreeNode<T> = nodes[root_index as usize];
    let left_height = height_from_root(nodes,node.indexes.left_index);
    let right_height =  height_from_root(nodes,node.indexes.right_index);

    let height = if left_height > right_height {
        left_height + 1
    } else {
        right_height + 1
    };

    let node: &mut TreeNode<T> = &mut nodes[root_index as usize];
    node.indexes.height = height
}

pub fn rotate_right<T: Default + PartialOrd + Copy, M: TreeVec<T> + Sized>(nodes: &mut M, root_index: i32) -> i32 {
    let left_index = nodes[root_index as usize].indexes.left_index;

    nodes[root_index as usize].indexes.left_index = nodes[left_index as usize].indexes.right_index;
    nodes[left_index as usize].indexes.right_index = root_index;

    fix_height(nodes,root_index);
    fix_height(nodes,left_index);

    left_index
}

pub fn rotate_left<T: Default + PartialOrd + Copy, M: TreeVec<T> + Sized>(nodes: &mut M, root_index: i32) -> i32 {
    let right_index = nodes[root_index as usize].indexes.right_index;

    nodes[root_index as usize].indexes.right_index = nodes[right_index as usize].indexes.left_index;
    nodes[right_index as usize].indexes.left_index = root_index;

    fix_height(nodes,root_index);
    fix_height(nodes,right_index);

    right_index
}

pub fn balance<T: Default + PartialOrd + Copy, M: TreeVec<T> + Sized >(mut nodes: &mut M, root_index: i32) -> i32{
    let mut new_root_index = root_index;
    fix_height(nodes,root_index);

    if bfactor(nodes,root_index) == 2 {
        if bfactor(nodes,nodes[root_index as usize].indexes.right_index) < 0 {
            nodes[root_index as usize].indexes.right_index = rotate_right(nodes,nodes[root_index as usize].indexes.right_index);
        }
        new_root_index = rotate_left(nodes,root_index);
    }

    if bfactor(nodes,root_index) == -2 {
        if bfactor(nodes,nodes[root_index as usize].indexes.left_index) > 0 {
            nodes[root_index as usize].indexes.left_index = rotate_left(nodes,nodes[root_index as usize].indexes.left_index);
        }
        new_root_index = rotate_right(nodes,root_index);
    }

    new_root_index
}

pub fn add_from_root<T: Default + PartialOrd + Copy, M: TreeVec<T> + Sized>(mut nodes: &mut M, compare: fn(&T, &T) -> Ordering, root_index: i32, value: T) -> i32 {
    if (compare)(&value, &nodes[root_index as usize].value) == Ordering::Less {
        if nodes[root_index as usize].indexes.left_index == -1 {
            nodes[root_index as usize].indexes.left_index = nodes.push(value);
        } else {
            nodes[root_index as usize].indexes.left_index = add_from_root(nodes, compare, nodes[root_index as usize].indexes.left_index, value);
        }
    } else {
        if nodes[root_index as usize].indexes.right_index == -1 {
            nodes[root_index as usize].indexes.right_index = nodes.push(value);
        } else {
            nodes[root_index as usize].indexes.right_index = add_from_root(nodes, compare, nodes[root_index as usize].indexes.right_index, value);
        }
    }
    balance(nodes, root_index)
}

fn find_min<T: Default + PartialOrd + Copy, M: TreeVec<T> + Sized>(mut nodes: &mut M, root_index: i32) -> i32 {
    if nodes[root_index as usize].indexes.left_index == -1 {
        root_index
    } else {
        find_min(nodes,nodes[root_index as usize].indexes.left_index)
    }
}

fn remove_min<T: Default + PartialOrd + Copy, M: TreeVec<T> + Sized>(mut nodes: &mut M, root_index: i32) -> i32 {
    if nodes[root_index as usize].indexes.left_index == -1 {
        nodes[root_index as usize].indexes.right_index
    } else {
        nodes[root_index as usize].indexes.left_index = remove_min(nodes,nodes[root_index as usize].indexes.left_index);
        balance(nodes,root_index)
    }
}

pub fn remove_from_root<T: Default + PartialOrd + Copy, M: TreeVec<T> + Sized>(mut nodes: &mut M, compare: fn(&T, &T) -> Ordering, root_index: i32, value: T) -> i32 {
    if (compare)(&value, &nodes[root_index as usize].value) == Ordering::Less {
        nodes[root_index as usize].indexes.left_index = remove_from_root(nodes,compare,nodes[root_index as usize].indexes.left_index, value);
    } else if (compare)(&value, &nodes[root_index as usize].value) == Ordering::Greater {
        nodes[root_index as usize].indexes.right_index = remove_from_root(nodes, compare,nodes[root_index as usize].indexes.right_index, value);
    } else {
        let left_index = nodes[root_index as usize].indexes.left_index;
        let right_index = nodes[root_index as usize].indexes.right_index;

        nodes.remove(root_index);

        if right_index == -1 {
            return left_index;
        }

        let min_index = find_min(nodes,right_index);
        nodes[min_index as usize].indexes.right_index = remove_min(nodes,right_index);
        nodes[min_index as usize].indexes.left_index = left_index;

        return balance(nodes,min_index);
    }
    balance(nodes,root_index)
}