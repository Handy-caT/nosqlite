/// Struct that stores statistics about a hash vector.
/// Now implemented only for the maximum length of the buckets.
pub struct HashVecStatistics {
    /// Size of the hash vector.
    pub size: usize,

    /// Maximum length of the buckets.
    pub max_length: usize,
    /// Buckets with the maximum length.
    max_length_buckets: Vec<bool>,
    /// Number of buckets with the maximum length.
    count: usize,
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
            size: 0,
            max_length: 0,
            max_length_buckets: vec![false; pool_size],
            count: 0,
        }
    }

    /// Updates the maximum length of the buckets.
    /// If the new length is greater than the current one,
    /// the current one is updated.
    /// Fills the `max_length_buckets` with false.
    /// # Arguments
    /// * `length` - New length.
    /// # Returns
    /// * `usize` - Current maximum length.
    pub fn update(&mut self, length: usize) -> usize {
        self.max_length = length;
        self.max_length_buckets.fill_with(|| false);
        self.count = 0;

        self.max_length
    }

    /// Marks a bucket as a bucket with the maximum length.
    /// # Arguments
    /// * `bucket` - Bucket to mark.
    pub fn add_bucket(&mut self, bucket: usize) {
        if !self.max_length_buckets[bucket] && self.max_length > 0 {
            self.max_length_buckets[bucket] = true;
            self.count += 1;
        }
    }

    /// Removes a bucket from the maximum length buckets.
    /// # Arguments
    /// * `bucket` - Bucket to remove.
    /// # Returns
    /// * `Option<usize>` - Bucket removed. None if the bucket is out of range.
    pub fn remove_bucket(&mut self, bucket: usize) -> Option<usize> {
        if bucket < self.max_length_buckets.len()
            && self.max_length_buckets[bucket]
        {
            self.max_length_buckets[bucket] = false;
            self.count -= 1;
            return Some(bucket);
        }
        None
    }

    /// Checks if a bucket is a bucket with the maximum length.
    /// # Arguments
    /// * `bucket` - Bucket to check.
    /// # Returns
    /// * `Option<bool>` - True if the bucket is
    /// a bucket with the maximum length.
    /// None if the bucket is out of range.
    pub fn is_max_length_bucket(&self, bucket: usize) -> Option<bool> {
        if bucket < self.max_length_buckets.len() {
            return Some(self.max_length_buckets[bucket]);
        }
        None
    }

    /// Returns the number of buckets with the maximum length.
    /// # Returns
    /// * `usize` - Number of buckets with the maximum length.
    pub fn get_count(&self) -> usize {
        self.count
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

        assert_eq!(hash_vec_statistics.update(7), 7);
        assert_eq!(hash_vec_statistics.max_length, 7);
        assert_eq!(hash_vec_statistics.max_length_buckets, vec![false; 10]);

        assert_eq!(hash_vec_statistics.update(3), 3);
        assert_eq!(hash_vec_statistics.max_length, 3);
        assert_eq!(hash_vec_statistics.max_length_buckets, vec![false; 10]);
    }

    #[test]
    fn test_hash_vec_statistics_add_bucket() {
        let mut hash_vec_statistics = HashVecStatistics::new(10);

        hash_vec_statistics.add_bucket(5);
        assert_eq!(hash_vec_statistics.max_length, 0);
        assert_eq!(hash_vec_statistics.count, 0);

        hash_vec_statistics.update(5);
        hash_vec_statistics.add_bucket(5);

        assert_eq!(hash_vec_statistics.max_length, 5);
        assert!(hash_vec_statistics.max_length_buckets[5]);
        assert_eq!(hash_vec_statistics.count, 1);
    }

    #[test]
    fn test_hash_vec_statistics_update_existing() {
        let mut hash_vec_statistics = HashVecStatistics::new(10);

        hash_vec_statistics.update(5);
        hash_vec_statistics.add_bucket(5);

        assert_eq!(hash_vec_statistics.max_length, 5);
        assert!(hash_vec_statistics.max_length_buckets[5]);
        assert_eq!(hash_vec_statistics.count, 1);

        hash_vec_statistics.update(6);

        assert_eq!(hash_vec_statistics.max_length, 6);
        assert!(!hash_vec_statistics.max_length_buckets[5]);
        assert_eq!(hash_vec_statistics.count, 0);
    }

    #[test]
    fn test_hash_vec_statistics_remove_bucket() {
        let mut hash_vec_statistics = HashVecStatistics::new(10);

        hash_vec_statistics.update(5);
        hash_vec_statistics.add_bucket(5);

        assert_eq!(hash_vec_statistics.max_length, 5);
        assert!(hash_vec_statistics.max_length_buckets[5]);

        hash_vec_statistics.remove_bucket(5);

        assert_eq!(hash_vec_statistics.max_length, 5);
        assert!(!hash_vec_statistics.max_length_buckets[5]);
        assert_eq!(hash_vec_statistics.count, 0);
    }

    #[test]
    fn test_hash_vec_statistics_remove_bucket_out_of_range() {
        let mut hash_vec_statistics = HashVecStatistics::new(10);

        hash_vec_statistics.update(5);
        hash_vec_statistics.add_bucket(5);

        assert_eq!(hash_vec_statistics.max_length, 5);
        assert!(hash_vec_statistics.max_length_buckets[5]);

        hash_vec_statistics.remove_bucket(10);

        assert_eq!(hash_vec_statistics.max_length, 5);
        assert!(hash_vec_statistics.max_length_buckets[5]);
    }
}
