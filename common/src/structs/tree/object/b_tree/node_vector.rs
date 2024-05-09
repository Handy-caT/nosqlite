use crate::structs::{
    hash_table::HashTable,
    tree::{nodes::btree::Node, object::b_tree::node_loader::NodeLoader},
};

#[derive(Debug, Clone)]
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
    /// * `BTreeVec`<T, L, M, `NODE_SIZE`> - New B-Tree vector.
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
    /// # Arguments
    /// * `node` - Node to add.
    /// # Returns
    /// * usize - Index of the new node.
    pub fn add_node(&mut self, node: Node<T, NODE_SIZE>) -> usize {
        let index = if let Some(index) = self.empty.pop() {
            index
        } else {
            self.max_index += 1;
            self.max_index - 1
        };
        self.preloaded_data.insert(node.get_index().index, node);

        index
    }

    /// Returns node by index.
    /// # Arguments
    /// * `index` - Index of the node.
    /// # Returns
    /// * Option<Node<T, `NODE_SIZE`>> - Node by index.
    pub fn get_node(&self, index: usize) -> Option<Node<T, NODE_SIZE>> {
        self.preloaded_data.get(&index)
    }

    /// Updates node by index.
    /// # Arguments
    /// * `index` - Index of the node.
    /// * `node` - New node.
    pub fn update_node(&mut self, index: usize, node: Node<T, NODE_SIZE>) {
        self.preloaded_data.insert(index, node);
    }

    /// Returns the next index.
    /// # Returns
    /// * usize - Next index.
    pub fn get_next_index(&self) -> usize {
        self.max_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::{
        hash_table::scalable::ScalableHashTable,
        tree::{nodes::btree::leaf, object::b_tree::node_loader::NodeLoader},
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

        let node = Node::Leaf(leaf::Leaf::new(0));
        let index = b_tree_vec.add_node(node);

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

        let node = Node::Leaf(leaf::Leaf::new(0));
        let index = b_tree_vec.add_node(node.clone());
        let res = b_tree_vec.get_node(index);

        assert_eq!(res, Some(node));
    }

    #[test]
    fn update_node() {
        let mut b_tree_vec = BTreeVec::<
            usize,
            MockNodeLoader,
            ScalableHashTable<usize, Node<usize, 4>>,
            4,
        >::new(MockNodeLoader {});

        let node = Node::Leaf(leaf::Leaf::new(0));
        let index = b_tree_vec.add_node(node);
        let node = Node::Leaf(leaf::Leaf::new(1));
        b_tree_vec.update_node(index, node.clone());

        let res = b_tree_vec.get_node(index);

        assert_eq!(res, Some(node));
    }
}
