use crate::core::structs::tree::nodes::tree_index::TreeIndex;
use queues::{queue, IsQueue, Queue};
use std::cmp::Ordering;

pub fn height_from_root(indexes: &mut Vec<TreeIndex>, root_index: i32) -> u8 {
    if root_index == -1 {
        0
    } else {
        indexes[root_index as usize].height
    }
}

pub fn bfactor(indexes: &mut Vec<TreeIndex>, root_index: i32) -> i8 {
    let node_indexes = indexes[root_index as usize];
    height_from_root(indexes, node_indexes.right_index) as i8
        - height_from_root(indexes, node_indexes.left_index) as i8
}

pub fn fix_height(indexes: &mut Vec<TreeIndex>, root_index: i32) {
    let node_indexes = indexes[root_index as usize];
    let left_height = height_from_root(indexes, node_indexes.left_index);
    let right_height = height_from_root(indexes, node_indexes.right_index);

    let height = if left_height > right_height {
        left_height + 1
    } else {
        right_height + 1
    };

    let node_indexes = &mut indexes[root_index as usize];
    node_indexes.height = height
}

pub fn rotate_right(indexes: &mut Vec<TreeIndex>, root_index: i32) -> i32 {
    let left_index = indexes[root_index as usize].left_index;

    indexes[root_index as usize].left_index = indexes[left_index as usize].right_index;
    indexes[left_index as usize].right_index = root_index;

    fix_height(indexes, root_index);
    fix_height(indexes, left_index);

    left_index
}

pub fn rotate_left(indexes: &mut Vec<TreeIndex>, root_index: i32) -> i32 {
    let right_index = indexes[root_index as usize].right_index;

    indexes[root_index as usize].right_index = indexes[right_index as usize].left_index;
    indexes[right_index as usize].left_index = root_index;

    fix_height(indexes, root_index);
    fix_height(indexes, right_index);

    right_index
}

pub fn balance(indexes: &mut Vec<TreeIndex>, root_index: i32) -> i32 {
    let mut new_root_index = root_index;
    fix_height(indexes, root_index);

    if bfactor(indexes, root_index) == 2 {
        if bfactor(indexes, indexes[root_index as usize].right_index) < 0 {
            indexes[root_index as usize].right_index =
                rotate_right(indexes, indexes[root_index as usize].right_index);
        }
        new_root_index = rotate_left(indexes, root_index);
    }

    if bfactor(indexes, root_index) == -2 {
        if bfactor(indexes, indexes[root_index as usize].left_index) > 0 {
            indexes[root_index as usize].left_index =
                rotate_left(indexes, indexes[root_index as usize].left_index);
        }
        new_root_index = rotate_right(indexes, root_index);
    }

    new_root_index
}

pub fn find_min(indexes: &mut Vec<TreeIndex>, root_index: i32) -> i32 {
    if indexes[root_index as usize].left_index == -1 {
        root_index
    } else {
        find_min(indexes, indexes[root_index as usize].left_index)
    }
}

pub fn remove_min(indexes: &mut Vec<TreeIndex>, root_index: i32) -> i32 {
    if indexes[root_index as usize].left_index == -1 {
        indexes[root_index as usize].right_index
    } else {
        indexes[root_index as usize].left_index =
            remove_min(indexes, indexes[root_index as usize].left_index);
        balance(indexes, root_index)
    }
}

pub fn find_greater_equal<T: Default + PartialOrd + Copy>(
    nodes: &mut Vec<T>,
    indexes: &mut Vec<TreeIndex>,
    compare: fn(&T, &T) -> Ordering,
    root: i32,
    value: T,
) -> Option<(i32, T)> {
    let mut queue: Queue<(i32, String)> = queue![];
    let mut current_index = root;
    let mut last = (-1, "".to_string());
    let mut ind = false;
    let mut turn_count = 0;

    while !ind && current_index != -1 {
        if (compare)(&value, &nodes[current_index as usize]) == Ordering::Less {
            if last.1 == "right" {
                turn_count += 1;
            }

            last = (current_index, "left".to_string());

            if turn_count > 1 {
                while queue.peek().unwrap().1 != "right" {
                    queue.remove();
                }
            }

            queue.add(last.clone());
            current_index = indexes[current_index as usize].left_index;
        } else if (compare)(&value, &nodes[current_index as usize]) == Ordering::Greater {
            if last.1 == "left" {
                turn_count += 1;
            }

            last = (current_index, "right".to_string());

            if turn_count > 1 {
                while queue.peek().unwrap().1 != "left" {
                    queue.remove();
                }
            }

            queue.add(last.clone());
            current_index = indexes[current_index as usize].right_index;
        } else {
            ind = true;
        }
    }

    return if ind {
        Some((
            indexes[current_index as usize].index,
            nodes[current_index as usize],
        ))
    } else {
        if last.1 == "right" {
            if queue.peek().unwrap().1 == "right" {
                None
            } else {
                let mut turn = queue.remove().unwrap();
                while queue.peek().unwrap().1 != "right" {
                    turn = queue.remove().unwrap();
                }

                Some((indexes[turn.0 as usize].index, nodes[turn.0 as usize]))
            }
        } else {
            Some((indexes[last.0 as usize].index, nodes[last.0 as usize]))
        }
    };
}
