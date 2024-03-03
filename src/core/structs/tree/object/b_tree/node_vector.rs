use crate::core::structs::{
    hash_table::HashTable,
    tree::object::b_tree::{node::Node, node_loader::NodeLoader},
};

struct BTreeVec<T, L, M, const NODE_SIZE: u8> {
    preloaded_data: M,

    node_loader: L,

    max_index: usize,

    /// Vector that stores the empty indexes.
    empty: Vec<usize>,

    _phantom: std::marker::PhantomData<T>,
}

impl<T, L, M, const NODE_SIZE: u8> BTreeVec<T, L, M, NODE_SIZE>
where
    L: NodeLoader<T, NODE_SIZE>,
    M: HashTable<usize, Node<T, NODE_SIZE>>,
{

}
