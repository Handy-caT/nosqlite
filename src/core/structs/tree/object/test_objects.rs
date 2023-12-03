#[cfg(test)]
mod tests {
    use crate::core::structs::tree::{
        object::{
            balanced_tree::decoratable::Decoratable, tree::Tree, BinHeap,
        },
        vectors::normalized_tree_vec::NormalizedTreeVector,
    };

    #[test]
    fn test_bin_heap_in_tree() {
        let mut heap = BinHeap::<u64>::new();

        heap.push(1);
        heap.push(2);
        heap.push(3);

        let tree =
            Decoratable::<u64,
                NormalizedTreeVector<u64>,
                BinHeap<u64>
            >::new_with_existing(
                heap,
                |a, b| b.cmp(&a),
            );

        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn test_bin_heap_in_tree_add() {
        let mut heap = BinHeap::<u64>::new();

        heap.push(1);
        heap.push(2);
        heap.push(3);

        let mut tree = Decoratable::<
            u64,
            NormalizedTreeVector<u64>,
            BinHeap<u64>,
        >::new_with_existing(heap, |a, b| b.cmp(&a));

        tree.push(4);

        assert_eq!(tree.len(), 4);
    }

    #[test]
    fn test_bim_heap_in_tree_remove() {
        let mut heap = BinHeap::<u64>::new();

        heap.push(1);
        heap.push(2);
        heap.push(3);

        let mut tree = Decoratable::<
            u64,
            NormalizedTreeVector<u64>,
            BinHeap<u64>,
        >::new_with_existing(heap, |a, b| b.cmp(&a));

        tree.remove_by_value(2);

        assert_eq!(tree.len(), 2);
    }
}
