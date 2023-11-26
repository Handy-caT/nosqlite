use crate::core::structs::hash_table::vectors::{
    hash_vec::{HashVec, Indexes, InternalStatistics},
    key_value::KeyValue,
};
use std::{marker::PhantomData, path::Iter};

/// [`HashVecIterator`] is an iterator for [`HashVec`]
/// * `K` - key type
/// * `V` - value type
/// * `H` - [`HashVec`] implementation
pub struct HashVecIterator<'a, K, V, H>
where
    H: HashVec<K, V>,
{
    table: &'a mut H,
    index: usize,
    size: usize,
    bucket: usize,
    v: PhantomData<V>,
    k: PhantomData<K>,
}

impl<'a, K, V, H> HashVecIterator<'a, K, V, H>
where
    H: HashVec<K, V>,
{
    /// Creates a new [`HashVecIterator`]
    /// # Arguments
    /// * `table` - [`HashVec`] implementation
    /// # Returns
    /// * `Self` - [`HashVecIterator`]
    pub fn new(table: &'a mut H) -> Self {
        let size = table.size();
        HashVecIterator {
            table,
            index: 0,
            size,
            bucket: 0,
            v: PhantomData,
            k: PhantomData,
        }
    }
}

impl<'a, K, V, H> Iterator for HashVecIterator<'a, K, V, H>
where
    H: HashVec<K, V> + Indexes<K, V>,
{
    type Item = KeyValue<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        let bucket = self.bucket;
        let index = self.index;
        let value = self.table.get_by_index(bucket, index);

        if value.is_some() {
            self.index += 1;
            value
        } else if self.bucket >= self.size {
            None
        } else {
            self.bucket += 1;
            self.index = 0;
            self.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::structs::hash_table::vectors::{
        hash_vec::HashVec, hash_vec_iterator::HashVecIterator,
        static_hash_vec::StaticHashVec,
    };

    #[test]
    fn test_hash_vec_iterator_new() {
        let mut hash_vec: StaticHashVec<u64, u64> = StaticHashVec::new(8);

        let hash_vec_iterator = HashVecIterator::new(&mut hash_vec);

        assert_eq!(hash_vec_iterator.index, 0);
        assert_eq!(hash_vec_iterator.size, 8);
        assert_eq!(hash_vec_iterator.bucket, 0);
    }

    #[test]
    fn test_hash_vec_iterator_next() {
        let mut hash_vec: StaticHashVec<u64, u64> = StaticHashVec::new(8);

        hash_vec.push(1, 1, 1);
        hash_vec.push(2, 2, 2);
        hash_vec.push(3, 3, 3);
        hash_vec.push(4, 4, 4);
        hash_vec.push(5, 5, 5);
        hash_vec.push(6, 6, 6);
        hash_vec.push(7, 7, 7);

        let mut hash_vec_iterator = HashVecIterator::new(&mut hash_vec);

        assert_eq!(hash_vec_iterator.next().unwrap().value, 1);
        assert_eq!(hash_vec_iterator.next().unwrap().value, 2);
        assert_eq!(hash_vec_iterator.next().unwrap().value, 3);
        assert_eq!(hash_vec_iterator.next().unwrap().value, 4);
        assert_eq!(hash_vec_iterator.next().unwrap().value, 5);
        assert_eq!(hash_vec_iterator.next().unwrap().value, 6);
        assert_eq!(hash_vec_iterator.next().unwrap().value, 7);
        assert_eq!(hash_vec_iterator.next(), None);
    }
}
