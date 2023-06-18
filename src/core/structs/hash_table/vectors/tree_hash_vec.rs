use crate::core::structs::hash_table::vectors::statistics::hash_vec_statistics::HashVecStatistics;
use crate::core::structs::tree::object::balanced_tree::balanced_tree::BalancedTree;
use crate::core::structs::tree::vectors::optimized_tree_vec::OptimizedTreeVec;

/// A hsh vector that uses a tree to store the values.
/// * `V` - Type of the value
/// * `N` - Number of buckets, must be a power of 2, if it is not, it will be rounded up to the next power of 2
struct TreeHashVec<V: Copy + Default, const N: u64> {
    /// The data of the hash vector as a vector of trees.
    /// OptimizedTreeVec is used as the underlying data structure for the trees.
    data: Vec<BalancedTree<V, OptimizedTreeVec<V>>>,
    /// Statistics of the hash vector
    statistics: HashVecStatistics,
}

impl <V: Copy + Default + PartialOrd, const N: u64> TreeHashVec<V, N> {
    /// Creates a new TreeHashVec
    /// # Returns
    /// * `TreeHashVec<V, N>` - New TreeHashVec
    pub fn new() -> TreeHashVec<V, N> {
        let mut vec = TreeHashVec {
            data: Vec::new(),
            statistics: HashVecStatistics::new(N as usize),
        };

        vec.data.reserve(N as usize);

        for _ in 0..N {
            let nodes = OptimizedTreeVec::new();
            vec.data.push(BalancedTree::<V, OptimizedTreeVec<V>>::new(nodes));
        }

        vec
    }
}