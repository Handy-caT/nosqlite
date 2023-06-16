
/// Struct that stores statistics about a hash vector.
/// Now implemented only for the maximum length of the buckets.
struct HashVecStatistics {
    /// Maximum length of the buckets.
    max_length: usize,
    /// Buckets with the maximum length.
    max_length_buckets: Vec<bool>
}

impl HashVecStatistics {

    /// Creates a new HashVecStatistics.
    /// # Arguments
    /// * `pool_size` - Size of the bucket pool.
    /// # Returns
    /// * `HashVecStatistics` - New HashVecStatistics.
    /// Default values:
    /// * `max_length` - 0
    /// * `max_length_buckets` - Vec of size `pool_size` filled with false.
    pub fn new(pool_size: usize) -> Self {
        Self {
            max_length: 0,
            max_length_buckets: vec![false; pool_size as usize]
        }
    }

    /// Updates the maximum length of the buckets.
    /// If the new length is greater than the current one, the current one is updated.
    /// Fills the `max_length_buckets` with false.
    /// # Arguments
    /// * `length` - New length.
    /// # Returns
    /// * `usize` - Current maximum length.
    pub fn update(&mut self, length: usize) -> usize {
        if length > self.max_length {
            self.max_length = length;
            self.max_length_buckets.fill_with(|| false);
        }
        self.max_length
    }

    /// Marks a bucket as a bucket with the maximum length.
    /// # Arguments
    /// * `bucket` - Bucket to mark.
    pub fn add_bucket(&mut self, bucket: usize) {
        self.max_length_buckets[bucket] = true;
    }

    /// Removes a bucket from the maximum length buckets.
    /// # Arguments
    /// * `bucket` - Bucket to remove.
    /// # Returns
    /// * `Option<usize>` - Bucket removed. None if the bucket is out of range.
    pub fn remove_bucket(&mut self, bucket: usize) -> Option<usize>{
        if bucket < self.max_length_buckets.len() {
            self.max_length_buckets[bucket] = false;
            return Some(bucket)
        }
        None
    }

    /// Checks if a bucket is a bucket with the maximum length.
    /// # Arguments
    /// * `bucket` - Bucket to check.
    /// # Returns
    /// * `Option<bool>` - True if the bucket is a bucket with the maximum length. None if the bucket is out of range.
    pub fn is_max_length_bucket(&self, bucket: usize) -> Option<bool> {
        if bucket < self.max_length_buckets.len() {
            return Some(self.max_length_buckets[bucket])
        }
        None
    }

    /// Returns the maximum length of the buckets.
    /// # Returns
    /// * `usize` - Maximum length of the buckets.
    pub fn get_max_length(&self) -> usize {
        self.max_length
    }

    /// Returns the buckets with the maximum length.
    /// # Returns
    /// * `&Vec<bool>` - Buckets with the maximum length.
    pub fn get_max_length_buckets(&self) -> &Vec<bool> {
        &self.max_length_buckets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_vec_statistics_new() {
        let hash_vec_statistics = HashVecStatistics::new(10);
        assert_eq!(hash_vec_statistics.max_length, 0);
        assert_eq!(hash_vec_statistics.max_length_buckets, vec![false; 10]);
    }

    #[test]
    fn test_hash_vec_statistics_update() {
        let mut hash_vec_statistics = HashVecStatistics::new(10);

        assert_eq!(hash_vec_statistics.update(5), 5);
        assert_eq!(hash_vec_statistics.max_length, 5);
        assert_eq!(hash_vec_statistics.max_length_buckets, vec![false; 10]);

        assert_eq!(hash_vec_statistics.update(3), 5);
        assert_eq!(hash_vec_statistics.max_length, 5);
        assert_eq!(hash_vec_statistics.max_length_buckets, vec![false; 10]);

        assert_eq!(hash_vec_statistics.update(7), 7);
        assert_eq!(hash_vec_statistics.max_length, 7);
        assert_eq!(hash_vec_statistics.max_length_buckets, vec![false; 10]);

        assert_eq!(hash_vec_statistics.update(3), 7);
        assert_eq!(hash_vec_statistics.max_length, 7);
        assert_eq!(hash_vec_statistics.max_length_buckets, vec![false; 10]);
    }

    #[test]
    fn test_hash_vec_statistics_add_bucket() {
        let mut hash_vec_statistics = HashVecStatistics::new(10);

        hash_vec_statistics.update(5);
        hash_vec_statistics.add_bucket(5);

        assert_eq!(hash_vec_statistics.max_length, 5);
        assert_eq!(hash_vec_statistics.max_length_buckets[5], true);
    }

    #[test]
    fn test_hash_vec_statistics_update_existing() {
        let mut hash_vec_statistics = HashVecStatistics::new(10);

        hash_vec_statistics.update(5);
        hash_vec_statistics.add_bucket(5);

        assert_eq!(hash_vec_statistics.max_length, 5);
        assert_eq!(hash_vec_statistics.max_length_buckets[5], true);

        hash_vec_statistics.update(6);

        assert_eq!(hash_vec_statistics.max_length, 6);
        assert_eq!(hash_vec_statistics.max_length_buckets[5], false);
    }

    #[test]
    fn test_hash_vec_statistics_remove_bucket() {
        let mut hash_vec_statistics = HashVecStatistics::new(10);

        hash_vec_statistics.update(5);
        hash_vec_statistics.add_bucket(5);

        assert_eq!(hash_vec_statistics.max_length, 5);
        assert_eq!(hash_vec_statistics.max_length_buckets[5], true);

        hash_vec_statistics.remove_bucket(5);

        assert_eq!(hash_vec_statistics.max_length, 5);
        assert_eq!(hash_vec_statistics.max_length_buckets[5], false);
    }

    #[test]
    fn test_hash_vec_statistics_remove_bucket_out_of_range() {
        let mut hash_vec_statistics = HashVecStatistics::new(10);

        hash_vec_statistics.update(5);
        hash_vec_statistics.add_bucket(5);

        assert_eq!(hash_vec_statistics.max_length, 5);
        assert_eq!(hash_vec_statistics.max_length_buckets[5], true);

        hash_vec_statistics.remove_bucket(10);

        assert_eq!(hash_vec_statistics.max_length, 5);
        assert_eq!(hash_vec_statistics.max_length_buckets[5], true);
    }
}