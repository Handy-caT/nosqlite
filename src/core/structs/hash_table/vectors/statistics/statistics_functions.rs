use crate::core::structs::hash_table::vectors::hash_vec::{HashVec, HashVecStatisticsInternal};

pub(in crate::core::structs::hash_table) fn statistics_add_actions<V: Default + Eq, const N: u64,M: HashVec<V, N> + HashVecStatisticsInternal<V, N>>(hash_vec: &mut M, index: u64) {
    let bucket_len = hash_vec.get_bucket_len(index);
    match bucket_len {
        Some(len) => {
            if len > hash_vec.get_max_len() {
                hash_vec.get_statistics_mut().update(len);
                hash_vec.get_statistics_mut().add_bucket(index as usize);
            } else if len == hash_vec.get_statistics().max_length {
                hash_vec.get_statistics_mut().add_bucket(index as usize);
            }
        },
        None => {}
    }
    hash_vec.get_statistics_mut().size += 1;
}

pub(in crate::core::structs::hash_table) fn statistics_remove_actions<V: Default + Eq, const N: u64,M: HashVec<V, N> + HashVecStatisticsInternal<V, N>>(hash_vec: &mut M, index: u64) {
    hash_vec.get_statistics_mut().size -= 1;
    let is_max = hash_vec.get_statistics().is_max_length_bucket(index as usize);
    match is_max {
        Some(true) => {
            hash_vec.get_statistics_mut().remove_bucket(index as usize);
            if hash_vec.get_statistics().get_count() == 0 {
                let bucket_len = hash_vec.get_bucket_len(index);
                match bucket_len {
                    Some(len) => {
                        hash_vec.get_statistics_mut().update(len - 1);
                        hash_vec.get_statistics_mut().add_bucket(index as usize);
                    },
                    None => {}
                }
            }
        },
        _ => {}
    }
}