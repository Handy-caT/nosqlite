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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Node<T, const NODE_SIZE: u8> {
    /// Vector that stores the keys.
    pub(crate) keys: Vec<T>,

    /// Vector that stores the children indexes.
    pub(crate) link_indexes: Vec<u32>,

    index: Index,
}

impl<T, const NODE_SIZE: u8> Node<T, NODE_SIZE>
where
    T: Ord,
{
    /// Creates a new node.
    /// # Arguments
    /// * `max_size` - Maximum size of the node.
    /// # Returns
    /// * Node<T> - New node.
    pub fn new(index: usize) -> Node<T, NODE_SIZE> {
        Node {
            keys: Vec::new(),
            link_indexes: Vec::new(),
            index: Index::new(index),
        }
    }

    /// Adds a value to the node with a child node index.
    /// # Arguments
    /// * `value` - Value to add.
    pub fn add_value(&mut self, value: T) -> Result<(), Error> {
        if let Some(last) = self.keys.last() {
            if last > &value {
                return Err(Error::InvalidValue);
            }
        }
        self.keys.push(value);
        Ok(())
    }

    /// Adds a child node index to the node.
    /// # Arguments
    /// * `index` - Child node index.
    pub fn add_link_index(&mut self, index: u32) -> Result<(), Error> {
        if self.link_indexes.len() <= self.keys.len() {
            self.link_indexes.push(index);
        } else {
            return Err(Error::InvalidValue);
        }

        Ok(())
    }

    /// Pops the last value from the node.
    /// # Returns
    /// * Option<T> - Popped value.
    pub fn pop_value(&mut self) -> Option<T> {
        self.keys.pop()
    }

    /// Pops the last child node index from the node.
    /// # Returns
    /// * Option<u32> - Popped index.
    pub fn pop_link_index(&mut self) -> Option<u32> {
        self.link_indexes.pop()
    }

    /// Gets the index to child by provided value.
    /// # Arguments
    /// * `value` - Value to search.
    /// # Returns
    /// * u32 - Index to child.
    pub fn get_index_by_value(&self, value: T) -> u32 {
        let find_index = self.keys.binary_search(&value);
        let index = if let Ok(index) = find_index {
            index
        } else {
            find_index.unwrap_err()
        };

        *self.link_indexes.get(index).unwrap()
    }

    /// Checks if the node is a leaf.
    /// # Returns
    /// * bool - True if the node is a leaf, false otherwise.
    pub fn is_leaf(&self) -> bool {
        self.link_indexes.is_empty()
    }
}

/// Error type for the node.
#[derive(Debug)]
pub enum Error {
    InvalidValue,
}

#[cfg(test)]
mod tests {
    use crate::core::structs::tree::object::b_tree::node::Node;

    #[test]
    fn test_node_new() {
        let node = Node::<i32, 3>::new(0);
        assert_eq!(node.keys.len(), 0);
        assert_eq!(node.link_indexes.len(), 0);

        let index = node.index;

        assert_eq!(index.index, 0);
        assert_eq!(index.left, None);
        assert_eq!(index.right, None);
        assert_eq!(index.parent, None);
    }

    #[test]
    fn test_node_add_value() {
        let mut node = Node::<i32, 3>::new(0);
        let res = node.add_value(1);

        assert!(res.is_ok());
        assert_eq!(node.keys.len(), 1);
        assert_eq!(node.keys[0], 1);
    }

    #[test]
    fn test_node_add_value_invalid() {
        let mut node = Node::<i32, 3>::new(0);
        let _ = node.add_value(2);
        let res = node.add_value(1);

        assert!(res.is_err());
        assert_eq!(node.keys.len(), 1);
        assert_eq!(node.keys[0], 2);
    }

    #[test]
    fn test_node_add_link_index() {
        let mut node = Node::<i32, 3>::new(0);
        let _ = node.add_value(1);
        let _ = node.add_link_index(0);
        let _ = node.add_link_index(1);

        assert_eq!(node.link_indexes.len(), 2);
        assert_eq!(node.link_indexes[0], 0);
        assert_eq!(node.link_indexes[1], 1);
    }

    #[test]
    fn test_node_add_link_index_invalid() {
        let mut node = Node::<i32, 3>::new(0);
        let _ = node.add_value(1);
        let _ = node.add_link_index(0);
        let _ = node.add_link_index(1);

        assert_eq!(node.link_indexes.len(), 2);
        assert_eq!(node.link_indexes[0], 0);
        assert_eq!(node.link_indexes[1], 1);

        let res = node.add_link_index(2);

        assert!(res.is_err());
        assert_eq!(node.link_indexes.len(), 2);
    }

    #[test]
    fn test_node_pop_value() {
        let mut node = Node::<i32, 3>::new(0);
        node.add_value(1);
        let value = node.pop_value();

        assert_eq!(value.unwrap(), 1);
        assert_eq!(node.keys.len(), 0);
    }

    #[test]
    fn test_node_pop_link_index() {
        let mut node = Node::<i32, 3>::new(0);
        node.add_link_index(1);
        let index = node.pop_link_index();

        assert_eq!(index.unwrap(), 1);
        assert_eq!(node.link_indexes.len(), 0);
    }

    #[test]
    fn test_node_get_index_by_value() {
        let mut node = Node::<i32, 3>::new(0);

        let _ = node.add_value(1);
        let _ = node.add_link_index(0);
        let _ = node.add_link_index(1);
        let _ = node.add_value(7);
        let _ = node.add_link_index(2);
        let _ = node.add_value(10);
        let _ = node.add_link_index(3);

        let index = node.get_index_by_value(0);
        assert_eq!(index, 0);

        let index = node.get_index_by_value(2);
        assert_eq!(index, 1);

        let index = node.get_index_by_value(8);
        assert_eq!(index, 2);

        let index = node.get_index_by_value(11);
        assert_eq!(index, 3);
    }

    #[test]
    fn test_node_is_leaf() {
        let mut node = Node::<i32, 3>::new(0);
        assert!(node.is_leaf());

        let _ = node.add_value(1);
        assert!(node.is_leaf());

        let _ = node.add_link_index(0);
        assert!(!node.is_leaf());
    }
}
