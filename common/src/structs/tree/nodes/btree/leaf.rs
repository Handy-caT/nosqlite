use crate::structs::tree::nodes::btree::Index;

/// Leaf node of a B-Tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Leaf<T, const NODE_SIZE: u8> {
    /// Vector that stores the values.
    keys: Vec<T>,

    /// Index of the node.
    pub index: Index,
}

impl<T, const NODE_SIZE: u8> Leaf<T, NODE_SIZE>
where
    T: Ord,
{
    /// Creates a new node.
    /// # Arguments
    /// * `max_size` - Maximum size of the node.
    /// # Returns
    /// * Node<T> - New node.
    #[must_use]
    pub fn new(index: usize) -> Leaf<T, NODE_SIZE> {
        Leaf {
            keys: Vec::new(),
            index: Index::new(index),
        }
    }

    /// Returns the value at the given index.
    /// # Arguments
    /// * `index` - Index of the value.
    /// # Returns
    /// * Option<&T> - Value at the given index.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.keys.get(index)
    }

    /// Returns position of the value in the node.
    /// # Arguments
    /// * `value` - Value to search.
    /// # Returns
    /// * usize - Position of the value.
    fn get_position_by_value(&self, value: &T) -> usize {
        let find_index = self.keys.binary_search(value);
        if let Ok(index) = find_index {
            index
        } else {
            find_index.unwrap_err()
        }
    }

    /// Adds a value to the node.
    /// # Arguments
    /// * `value` - Value to add.
    /// # Returns
    /// * Result<(), Error> - Result of the operation.
    /// # Errors
    /// * Error::IsFull - The node is full.
    pub fn add_value(&mut self, value: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::IsFull);
        }

        let pos = self.get_position_by_value(&value);
        self.keys.insert(pos, value);

        Ok(())
    }

    /// Pushes a value to the node's back.
    /// # Arguments
    /// * `value` - Value to push.
    /// # Returns
    /// * Result<(), Error> - Result of the operation.
    /// # Errors
    /// * Error::InvalidValue - The value is less than last value.
    pub fn push_value(&mut self, value: T) -> Result<(), Error> {
        if let Some(last) = self.keys.last() {
            if last > &value {
                return Err(Error::InvalidValue);
            }
        }
        self.keys.push(value);
        Ok(())
    }

    /// Pops the last value from the node.
    /// # Returns
    /// * Option<T> - Popped value.
    pub fn pop_value(&mut self) -> Option<T> {
        self.keys.pop()
    }

    /// Checks if the node contains the value.
    /// # Arguments
    /// * `value` - Value to search.
    /// # Returns
    /// * bool - True if the node contains the value, false otherwise.
    pub fn contains_value(&self, value: &T) -> bool {
        let find_index = self.keys.binary_search(value);
        find_index.is_ok()
    }

    /// Splits the node at the given index.
    /// # Arguments
    /// * `index` - Index to split.
    /// # Returns
    /// * Leaf<T, NODE_SIZE> - New node.
    pub fn split(&mut self, index: usize) -> Leaf<T, NODE_SIZE> {
        let mut new_node = Leaf::new(index);
        let mid = self.keys.len() / 2;
        new_node.keys = self.keys.split_off(mid);
        new_node
    }

    /// Returns the maximum value of the node.
    /// # Returns
    /// * &T - Maximum value of the node.
    /// # Panics
    /// If the node is empty.
    pub fn get_max_value(&self) -> &T {
        self.keys.last().expect("node must have at least one value")
    }

    /// Checks if the node is full.
    /// # Returns
    /// * bool - True if the node is full, false otherwise.
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.keys.len() == NODE_SIZE as usize
    }

    /// Returns the length of the node.
    /// # Returns
    /// * usize - Length of the node.
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Checks if the node is empty.
    /// # Returns
    /// * bool - True if the node is empty, false otherwise.
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
}

/// Error type for the node.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The value is invalid.
    InvalidValue,

    /// The node is full.
    IsFull,
}

#[cfg(test)]
mod tests {
    use crate::structs::tree::nodes::btree::Index;

    use super::{Error, Leaf};

    #[test]
    fn test_new() {
        let node = Leaf::<i32, 3>::new(0);
        assert_eq!(node.keys, Vec::new());
        assert_eq!(node.index, Index::new(0));
    }

    #[test]
    fn test_add_value() {
        let mut node = Leaf::<i32, 3>::new(0);
        assert_eq!(node.add_value(1), Ok(()));
        assert_eq!(node.add_value(2), Ok(()));
        assert_eq!(node.add_value(3), Ok(()));
        assert_eq!(node.add_value(4), Err(Error::IsFull));
    }

    #[test]
    fn test_push_value() {
        let mut node = Leaf::<i32, 3>::new(0);
        assert_eq!(node.push_value(1), Ok(()));
        assert_eq!(node.push_value(3), Ok(()));
        assert_eq!(node.push_value(2), Err(Error::InvalidValue));
    }

    #[test]
    fn test_pop_value() {
        let mut node = Leaf::<i32, 3>::new(0);
        node.push_value(1).unwrap();
        node.push_value(2).unwrap();
        assert_eq!(node.pop_value(), Some(2));
        assert_eq!(node.pop_value(), Some(1));
        assert_eq!(node.pop_value(), None);
    }

    #[test]
    fn test_contains_value() {
        let mut node = Leaf::<i32, 3>::new(0);
        node.push_value(1).unwrap();
        node.push_value(2).unwrap();
        assert!(node.contains_value(&1));
        assert!(node.contains_value(&2));
        assert!(!node.contains_value(&3));
    }

    #[test]
    fn test_is_full() {
        let mut node = Leaf::<i32, 3>::new(0);
        assert!(!node.is_full());
        node.push_value(1).unwrap();
        node.push_value(2).unwrap();
        node.push_value(3).unwrap();
        assert!(node.is_full());
    }

    #[test]
    fn test_split() {
        let mut node = Leaf::<i32, 3>::new(0);
        node.push_value(1).unwrap();
        node.push_value(2).unwrap();
        node.push_value(3).unwrap();
        node.push_value(4).unwrap();
        node.push_value(5).unwrap();

        let new_node = node.split(1);

        assert_eq!(node.keys, vec![1, 2]);
        assert_eq!(new_node.keys, vec![3, 4, 5]);

        assert_eq!(node.index.index, 0);
        assert_eq!(new_node.index.index, 1);
    }

    #[test]
    fn test_get_max_value() {
        let mut node = Leaf::<i32, 3>::new(0);
        node.push_value(1).unwrap();
        node.push_value(2).unwrap();
        node.push_value(3).unwrap();

        assert_eq!(node.get_max_value(), &3);
    }
}
