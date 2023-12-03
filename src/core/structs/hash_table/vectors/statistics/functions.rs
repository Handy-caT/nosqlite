use crate::core::structs::hash_table::vectors::hash_vec::{
    HashVec, InternalStatistics,
};

/// Function that updates statistics after push action.
/// It checks if bucket length is bigger than max length and updates statistics.
/// If bucket length is equal to max length,
/// it adds bucket to max length buckets.
/// # Arguments
/// * `hash_vec` - [`HashVec`] to update
/// * `index` - index of bucket
pub(in crate::core::structs::hash_table) fn statistics_add_actions<K, V, M>(
    hash_vec: &mut M,
    index: usize,
) where
    M: HashVec<K, V> + InternalStatistics<K, V>,
{
    let bucket_len = hash_vec.get_bucket_len(index);
    if let Some(len) = bucket_len {
        if len > hash_vec.get_max_len() {
            hash_vec.get_statistics_mut().update(len);
            hash_vec.get_statistics_mut().add_bucket(index);
        } else if len == hash_vec.get_statistics().max_length {
            hash_vec.get_statistics_mut().add_bucket(index);
        }
    }
    hash_vec.get_statistics_mut().size += 1;
}

/// Function that updates statistics after remove action.
/// It checks if bucket length is equal to max length and updates statistics.
/// If bucket length is equal to max length,
/// it removes bucket from max length buckets.
/// # Arguments
/// * `hash_vec` - [`HashVec`] to update
/// * `index` - index of bucket
pub(in crate::core::structs::hash_table) fn statistics_remove_actions<K, V, M>(
    hash_vec: &mut M,
    index: usize,
) where
    M: HashVec<K, V> + InternalStatistics<K, V>,
{
    hash_vec.get_statistics_mut().size -= 1;
    let is_max = hash_vec.get_statistics().is_max_length_bucket(index);
    if let Some(true) = is_max {
        hash_vec.get_statistics_mut().remove_bucket(index);
        if hash_vec.get_statistics().get_count() == 0 {
            let bucket_len = hash_vec.get_bucket_len(index);
            if let Some(len) = bucket_len {
                hash_vec.get_statistics_mut().update(len - 1);
                hash_vec.get_statistics_mut().add_bucket(index);
            }
        }
    }
}
