use crate::core::structs::tree::object::b_tree::node::Node;

pub trait NodeLoader<T, const NODE_SIZE: u8> {
    fn load_node(&self, index: usize) -> Node<T, NODE_SIZE>;
    fn preload_nodes(&self) -> Vec<Node<T, NODE_SIZE>>;
}
