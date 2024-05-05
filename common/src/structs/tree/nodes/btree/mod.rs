mod leaf;
mod node;

use crate::structs::tree::object::b_tree::node::Node;

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

/// Index of B-Tree node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndexType {
    /// Leaf node index.
    Leaf(usize),

    /// Node index.
    Node(usize),
}

impl Default for IndexType {
    fn default() -> Self {
        IndexType::Leaf(0)
    }
}
