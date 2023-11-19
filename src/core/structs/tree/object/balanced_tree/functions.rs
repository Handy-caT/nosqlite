use crate::core::structs::tree::nodes::tree_index::TreeIndex;
use queues::{queue, IsQueue, Queue};
use std::cmp::Ordering;

pub fn height_from_root(
    indexes: &mut [TreeIndex],
    root_index: Option<usize>,
) -> u8 {
    if let Some(index) = root_index {
        indexes[index].height
    } else {
        0
    }
}

pub fn bfactor(indexes: &mut [TreeIndex], root_index: usize) -> i8 {
    let node_indexes = indexes[root_index];
    i8::try_from(height_from_root(indexes, node_indexes.right_index)).unwrap()
        - i8::try_from(height_from_root(indexes, node_indexes.left_index)).unwrap()
}

pub fn fix_height(indexes: &mut [TreeIndex], root_index: usize) {
    let node_indexes = indexes[root_index];

    let left_height = height_from_root(indexes, node_indexes.left_index);
    let right_height = height_from_root(indexes, node_indexes.right_index);

    let height = if left_height > right_height {
        left_height + 1
    } else {
        right_height + 1
    };

    let node_indexes = &mut indexes[root_index];
    node_indexes.height = height;
}

pub fn rotate_right(indexes: &mut [TreeIndex], root_index: usize) -> usize {
    let left_index = indexes[root_index].left_index.unwrap();

    indexes[root_index].left_index = indexes[left_index].right_index;
    indexes[left_index].right_index = Some(root_index);

    fix_height(indexes, root_index);
    fix_height(indexes, left_index);

    left_index
}

pub fn rotate_left(indexes: &mut [TreeIndex], root_index: usize) -> usize {
    let right_index = indexes[root_index].right_index.unwrap();

    indexes[root_index].right_index = indexes[right_index].left_index;
    indexes[right_index].left_index = Some(root_index);

    fix_height(indexes, root_index);
    fix_height(indexes, right_index);

    right_index
}

pub fn balance(indexes: &mut [TreeIndex], root_index: usize) -> usize {
    let mut new_root_index = root_index;
    fix_height(indexes, root_index);

    if bfactor(indexes, root_index) == 2 {
        if bfactor(indexes, indexes[root_index].right_index.unwrap()) < 0 {
            indexes[root_index].right_index = Some(rotate_right(
                indexes,
                indexes[root_index].right_index.unwrap(),
            ));
        }
        new_root_index = rotate_left(indexes, root_index);
    }

    if bfactor(indexes, root_index) == -2 {
        if bfactor(indexes, indexes[root_index].left_index.unwrap()) > 0 {
            indexes[root_index].left_index = Some(rotate_left(
                indexes,
                indexes[root_index].left_index.unwrap(),
            ));
        }
        new_root_index = rotate_right(indexes, root_index);
    }

    new_root_index
}

pub fn find_min(indexes: &mut Vec<TreeIndex>, root_index: usize) -> usize {
    if indexes[root_index].left_index.is_none() {
        root_index
    } else {
        find_min(indexes, indexes[root_index].left_index.unwrap())
    }
}

pub fn remove_min(
    indexes: &mut Vec<TreeIndex>,
    root_index: usize,
) -> Option<usize> {
    if indexes[root_index].left_index.is_none() {
        indexes[root_index].right_index
    } else {
        indexes[root_index].left_index =
            remove_min(indexes, indexes[root_index].left_index.unwrap());
        Some(balance(indexes, root_index))
    }
}

pub fn find_greater_equal<T: Default + PartialOrd + Copy>(
    nodes: &mut [T],
    indexes: &mut [TreeIndex],
    compare: fn(&T, &T) -> Ordering,
    root: usize,
    value: T,
) -> Option<(usize, T)> {
    let mut queue: Queue<(Option<usize>, String)> = queue![];
    let mut current_index = Some(root);
    let mut last = (None, String::new());
    let mut ind = false;
    let mut turn_count = 0;

    while !ind && current_index.is_some() {
        if (compare)(&value, &nodes[current_index.unwrap()]) == Ordering::Less {
            if last.1 == "right" {
                turn_count += 1;
            }

            last = (current_index, "left".to_string());

            if turn_count > 1 {
                while queue.peek().unwrap().1 != "right" {
                    let _ = queue.remove();
                }
            }

            let _ = queue.add(last.clone());
            current_index = indexes[current_index.unwrap()].left_index;
        } else if (compare)(&value, &nodes[current_index.unwrap()])
            == Ordering::Greater
        {
            if last.1 == "left" {
                turn_count += 1;
            }

            last = (current_index, "right".to_string());

            if turn_count > 1 {
                while queue.peek().unwrap().1 != "left" {
                    let _ = queue.remove();
                }
            }

            let _ = queue.add(last.clone());
            current_index = indexes[current_index.unwrap()].right_index;
        } else {
            ind = true;
        }
    }

    return if ind {
        Some((
            indexes[current_index.unwrap()].index.unwrap(),
            nodes[current_index.unwrap()],
        ))
    } else if last.1 == "right" {
        if queue.peek().unwrap().1 == "right" {
            None
        } else {
            let mut turn = queue.remove().unwrap();
            while queue.peek().unwrap().1 != "right" {
                turn = queue.remove().unwrap();
            }

            Some((
                indexes[turn.0.unwrap()].index.unwrap(),
                nodes[turn.0.unwrap()],
            ))
        }
    } else {
        Some((
            indexes[last.0.unwrap()].index.unwrap(),
            nodes[last.0.unwrap()],
        ))
    };
}
