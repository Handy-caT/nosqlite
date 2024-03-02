use crate::core::structs::tree::object::b_tree::node::Node;

mod node;
mod node_loader;

struct BTree<T, L> {
    preloaded_data: Vec<Node<T>>,

    node_loader: L,
}

impl<T, L> BTree<T, L> {
    pub fn new(node_loader: L) -> BTree<T, L> {
        BTree {
            preloaded_data: Vec::new(),
            node_loader,
        }
    }
}
