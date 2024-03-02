use crate::core::structs::tree::object::b_tree::node::Node;

trait NodeLoader<T> {
    fn load_node(&self, index: usize) -> Node<T>;
    fn preload_nodes(&self) -> Vec<Node<T>>;
}
