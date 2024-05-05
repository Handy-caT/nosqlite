pub mod internal;
pub mod leaf;

/// [`Node`] index of a B-Tree.
/// It stores the index of the node, the left and right brother indexes and
/// the parent index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Index {
    /// Index of the [`Node`].
    pub index: usize,

    /// Index of the left brother.
    pub left: Option<usize>,

    /// Index of the right brother.
    pub right: Option<usize>,

    /// Index of the parent.
    pub parent: Option<usize>,
}

impl Index {
    /// Creates a new node index.
    /// # Arguments
    /// * `index` - Index of the node.
    /// # Returns
    /// * Index - New node index.
    #[must_use]
    pub fn new(index: usize) -> Index {
        Index {
            index,
            left: None,
            right: None,
            parent: None,
        }
    }
}

/// Node of a B-Tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node<T, const NODE_SIZE: u8> {
    /// Leaf node.
    Leaf(leaf::Leaf<T, NODE_SIZE>),

    /// Internal node.
    Internal(internal::Internal<T, NODE_SIZE>),
}

impl<T, const NODE_SIZE: u8> Node<T, NODE_SIZE> {
    /// Returns the index of the node.
    /// # Returns
    /// * Index - Index of the node.
    pub fn get_index(&self) -> &Index {
        match self {
            Node::Leaf(leaf) => &leaf.index,
            Node::Internal(internal) => &internal.index,
        }
    }
}
