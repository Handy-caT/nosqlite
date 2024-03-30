use crate::structs::{
    hash_table::HashTable,
    tree::object::b_tree::{node::Node, node_loader::NodeLoader},
};

#[derive(Debug)]
pub struct BTreeVec<T, L, M, const NODE_SIZE: u8> {
    /// Preloaded data.
    preloaded_data: M,

    /// Node loader.
    node_loader: L,

    /// Current max index.
    max_index: usize,

    /// Vector that stores the empty indexes.
    empty: Vec<usize>,

    _phantom: std::marker::PhantomData<T>,
}

impl<T, L, M, const NODE_SIZE: u8> BTreeVec<T, L, M, NODE_SIZE>
where
    T: Ord,
    L: NodeLoader<T, NODE_SIZE>,
    M: HashTable<usize, Node<T, NODE_SIZE>>,
{
    /// Creates a new B-Tree vector.
    /// # Arguments
    /// * `node_loader` - Node loader.
    /// # Returns
    /// * BTreeVec<T, L, M, NODE_SIZE> - New B-Tree vector.
    pub fn new(node_loader: L) -> BTreeVec<T, L, M, NODE_SIZE> {
        BTreeVec {
            preloaded_data: M::new(8),
            node_loader,
            max_index: 0,
            empty: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Adds a new node to the tree.
    /// # Returns
    /// * usize - Index of the new node.
    pub fn add_node(&mut self) -> usize {
        let index = if let Some(index) = self.empty.pop() {
            index
        } else {
            self.max_index += 1;
            self.max_index - 1
        };

        let node = Node::new(index);
        self.preloaded_data.insert(index, node);

        index
    }

    /// Adds a new leaf to the tree.
    /// # Arguments
    /// * `value` - Value to add.
    /// # Returns
    /// * usize - Index of the new leaf.
    pub fn add_leaf(&mut self, value: T) -> usize {
        let index = if let Some(index) = self.empty.pop() {
            index
        } else {
            self.max_index += 1;
            self.max_index - 1
        };

        let mut node = Node::new(index);
        node.add_value(value, 0).unwrap();
        self.preloaded_data.insert(index, node);

        index
    }

    /// Returns node by index.
    /// # Arguments
    /// * `index` - Index of the node.
    /// # Returns
    /// * Option<Node<T, NODE_SIZE>> - Node by index.
    pub fn get_node(&mut self, index: usize) -> Option<Node<T, NODE_SIZE>> {
        self.preloaded_data.get(&index)
    }

    /// Updates node by index.
    /// # Arguments
    /// * `index` - Index of the node.
    /// * `node` - New node.
    pub fn update_node(&mut self, index: usize, node: Node<T, NODE_SIZE>) {
        self.preloaded_data.insert(index, node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::{
        hash_table::scalable_hash_table::ScalableHashTable,
        tree::object::b_tree::node_loader::NodeLoader,
    };

    struct MockNodeLoader {}

    impl<T, const NODE_SIZE: u8> NodeLoader<T, NODE_SIZE> for MockNodeLoader {
        fn load_node(&self, index: usize) -> Node<T, NODE_SIZE> {
            todo!()
        }

        fn preload_nodes(&self) -> Vec<Node<T, NODE_SIZE>> {
            todo!()
        }
    }

    #[test]
    fn new() {
        let b_tree_vec = BTreeVec::<
            usize,
            MockNodeLoader,
            ScalableHashTable<usize, Node<usize, 4>>,
            4,
        >::new(MockNodeLoader {});

        assert_eq!(b_tree_vec.max_index, 0);
        assert_eq!(b_tree_vec.empty, Vec::new());
    }

    #[test]
    fn add_node() {
        let mut b_tree_vec = BTreeVec::<
            usize,
            MockNodeLoader,
            ScalableHashTable<usize, Node<usize, 4>>,
            4,
        >::new(MockNodeLoader {});

        let index = b_tree_vec.add_node();

        assert_eq!(index, 0);
        assert_eq!(b_tree_vec.max_index, 1);
        assert_eq!(b_tree_vec.empty, Vec::new());
    }

    #[test]
    fn add_leaf() {
        let mut b_tree_vec = BTreeVec::<
            usize,
            MockNodeLoader,
            ScalableHashTable<usize, Node<usize, 4>>,
            4,
        >::new(MockNodeLoader {});

        let index = b_tree_vec.add_leaf(1);

        assert_eq!(index, 0);
        assert_eq!(b_tree_vec.max_index, 1);
        assert_eq!(b_tree_vec.empty, Vec::new());
    }

    #[test]
    fn get_node() {
        let mut b_tree_vec = BTreeVec::<
            usize,
            MockNodeLoader,
            ScalableHashTable<usize, Node<usize, 4>>,
            4,
        >::new(MockNodeLoader {});

        let index = b_tree_vec.add_node();
        let node = b_tree_vec.get_node(index);

        assert_eq!(node, Some(Node::new(index)));
    }

    #[test]
    fn update_node() {
        let mut b_tree_vec = BTreeVec::<
            usize,
            MockNodeLoader,
            ScalableHashTable<usize, Node<usize, 4>>,
            4,
        >::new(MockNodeLoader {});

        let index = b_tree_vec.add_node();
        let node = Node::new(index);
        b_tree_vec.update_node(index, node);

        let node = b_tree_vec.get_node(index);

        assert_eq!(node, Some(Node::new(index)));
    }
}
