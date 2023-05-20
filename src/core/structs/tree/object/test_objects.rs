#[cfg(test)]
mod tests {
    use crate::core::structs::tree::object::balanced_tree::decoratable_balanced_tree::DecoratableBalancedTree;
    use crate::core::structs::tree::object::bin_heap::bin_heap::BinHeap;
    use crate::core::structs::tree::object::tree_object::TreeObject;
    use crate::core::structs::tree::vectors::normalized_tree_vec::NormalizedTreeVector;

    #[test]
    fn test_bin_heap_in_tree() {
        let mut heap = BinHeap::<u64>::new();

        heap.push(1);
        heap.push(2);
        heap.push(3);

        let tree = DecoratableBalancedTree::<u64, NormalizedTreeVector<u64>, BinHeap<u64>>::new(heap, |a, b| b.cmp(a));

        assert_eq!(tree.len(), 3);
    }
}