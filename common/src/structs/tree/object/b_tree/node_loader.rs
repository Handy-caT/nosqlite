use crate::structs::tree::object::b_tree::node::Node;

pub trait NodeLoader<T, const NODE_SIZE: u8> {
    fn load_node(&self, index: usize) -> Node<T, NODE_SIZE>;
    fn preload_nodes(&self) -> Vec<Node<T, NODE_SIZE>>;
}

#[derive(Debug, Default)]
pub struct BaseLoader;

impl<T, const NODE_SIZE: u8> NodeLoader<T, NODE_SIZE> for BaseLoader {
    fn load_node(&self, _: usize) -> Node<T, NODE_SIZE> {
        unimplemented!()
    }

    fn preload_nodes(&self) -> Vec<Node<T, NODE_SIZE>> {
        unimplemented!()
    }
}
