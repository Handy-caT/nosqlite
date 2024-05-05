use crate::structs::tree::nodes::btree::Index;

/// Node of a B-Tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Internal<T, const NODE_SIZE: u8> {
    /// Vector that stores the values with index of child node.
    keys: Vec<(T, usize)>,

    /// Index of the node.
    pub index: Index,
}

impl<T, const NODE_SIZE: u8> Internal<T, NODE_SIZE>
where
    T: Ord,
{
    /// Creates a new node.
    /// # Arguments
    /// * `max_size` - Maximum size of the node.
    /// # Returns
    /// * Node<T> - New node.
    #[must_use]
    pub fn new(index: usize) -> Internal<T, NODE_SIZE> {
        Internal {
            keys: Vec::new(),
            index: Index::new(index),
        }
    }

    /// Returns the value at the given index.
    /// # Arguments
    /// * `index` - Index of the value.
    /// # Returns
    /// * Option<&(T, usize)> - Value at the given index.
    pub fn get(&self, index: usize) -> Option<&(T, usize)> {
        self.keys.get(index)
    }

    /// Returns position of the value in the node.
    /// # Arguments
    /// * `value` - Value to search.
    /// # Returns
    /// * usize - Position of the value.
    fn get_position_by_value(&self, value: &T) -> usize {
        let find_index = self.keys.binary_search_by(|(v, _)| v.cmp(value));
        if let Ok(index) = find_index {
            index
        } else {
            find_index.unwrap_err()
        }
    }

    /// Adds a value with index to the node.
    /// # Arguments
    /// * `value` - Value to add.
    /// * `index` - Index of the child node.
    /// # Returns
    /// * Result<(), Error> - Result of the operation.
    /// # Errors
    /// * Error::IsFull - The node is full.
    pub fn add_value(&mut self, value: T, index: usize) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::IsFull);
        }

        let pos = self.get_position_by_value(&value);
        self.keys.insert(pos, (value, index));

        Ok(())
    }

    /// Pushes a value to the node's back.
    /// # Arguments
    /// * `value` - Value to push.
    /// * `index` - Index of the child node.
    /// # Returns
    /// * Result<(), Error> - Result of the operation.
    /// # Errors
    /// * Error::InvalidValue - The value is invalid.
    pub fn push_value(&mut self, value: T, index: usize) -> Result<(), Error> {
        if let Some(last) = self.keys.last() {
            if last.0 > value {
                return Err(Error::InvalidValue);
            }
        }
        self.keys.push((value, index));
        Ok(())
    }

    /// Pops the last value from the node.
    /// # Returns
    /// * Option<(T, IndexType)> - Popped value.
    pub fn pop_value(&mut self) -> Option<(T, usize)> {
        self.keys.pop()
    }

    /// Checks if the node contains the value.
    /// # Arguments
    /// * `value` - Value to search.
    /// # Returns
    /// * bool - True if the node contains the value, false otherwise.
    pub fn contains_value(&self, value: &T) -> bool {
        let find_index = self.keys.binary_search_by(|(v, _)| v.cmp(value));
        find_index.is_ok()
    }

    /// Returns the index of the value.
    /// # Arguments
    /// * `value` - Value to search.
    /// # Returns
    /// * Option<usize> - Index of the child node.
    pub fn get_index_by_value(&self, value: &T) -> usize {
        let find_index = self.keys.binary_search_by(|(v, _)| v.cmp(value));
        if let Ok(index) = find_index {
            self.keys[index].1
        } else {
            let index = find_index.unwrap_err();
            if index == self.keys.len() {
                self.keys[index - 1].1
            } else {
                self.keys[index].1
            }
        }
    }

    /// Splits the node at the given index.
    /// # Arguments
    /// * `index` - Index to split.
    /// # Returns
    /// * Leaf<T, NODE_SIZE> - New node.
    pub fn split(&mut self, index: usize) -> Internal<T, NODE_SIZE> {
        let mut new_node = Internal::new(index);
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
        self.keys.last().map(|(v, _)| v).expect("node is not empty")
    }
    
    /// Returns the children of the node.
    /// # Returns
    /// * impl Iterator<Item = usize> - Iterator of the children.
    pub fn get_children(&self) -> impl Iterator<Item = usize> + '_ {
        self.keys.iter().map(|(_, index)| *index)
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

    use super::{Error, Internal};

    #[test]
    fn test_new() {
        let node = Internal::<i32, 3>::new(0);
        assert_eq!(node.keys, Vec::new());
        assert_eq!(node.index, Index::new(0));
    }

    #[test]
    fn test_add_value() {
        let mut node = Internal::<i32, 3>::new(0);
        assert_eq!(node.add_value(1, 1), Ok(()));
        assert_eq!(node.add_value(2, 2), Ok(()));
        assert_eq!(node.add_value(3, 3), Ok(()));
        assert_eq!(node.add_value(4, 4), Err(Error::IsFull));
    }

    #[test]
    fn test_push_value() {
        let mut node = Internal::<i32, 3>::new(0);
        assert_eq!(node.push_value(1, 1), Ok(()));
        assert_eq!(node.push_value(3, 3), Ok(()));
        assert_eq!(node.push_value(2, 2), Err(Error::InvalidValue));
    }

    #[test]
    fn test_pop_value() {
        let mut node = Internal::<i32, 3>::new(0);
        node.push_value(1, 1).unwrap();
        node.push_value(2, 2).unwrap();
        assert_eq!(node.pop_value(), Some((2, 2)));
        assert_eq!(node.pop_value(), Some((1, 1)));
        assert_eq!(node.pop_value(), None);
    }

    #[test]
    fn test_contains_value() {
        let mut node = Internal::<i32, 3>::new(0);
        node.push_value(1, 1).unwrap();
        node.push_value(2, 2).unwrap();
        assert!(node.contains_value(&1));
        assert!(node.contains_value(&2));
        assert!(!node.contains_value(&3));
    }

    #[test]
    fn test_get_index_by_value() {
        let mut node = Internal::<i32, 3>::new(0);
        node.push_value(1, 0).unwrap();
        node.push_value(10, 1).unwrap();
        assert_eq!(node.get_index_by_value(&-1), 0);
        assert_eq!(node.get_index_by_value(&8), 1);
        assert_eq!(node.get_index_by_value(&11), 1);
    }

    #[test]
    fn test_is_full() {
        let mut node = Internal::<i32, 3>::new(0);
        assert!(!node.is_full());
        node.push_value(1, 1).unwrap();
        node.push_value(2, 2).unwrap();
        node.push_value(3, 3).unwrap();
        assert!(node.is_full());
    }

    #[test]
    fn test_split() {
        let mut node = Internal::<i32, 3>::new(0);
        node.push_value(1, 1).unwrap();
        node.push_value(2, 2).unwrap();
        node.push_value(3, 3).unwrap();
        node.push_value(4, 4).unwrap();
        node.push_value(5, 5).unwrap();

        let new_node = node.split(1);
        assert_eq!(node.keys, vec![(1, 1), (2, 2)]);
        assert_eq!(new_node.keys, vec![(3, 3), (4, 4), (5, 5)]);
    }

    #[test]
    fn test_get_max_value() {
        let mut node = Internal::<i32, 3>::new(0);
        node.push_value(1, 1).unwrap();
        node.push_value(2, 2).unwrap();
        node.push_value(3, 3).unwrap();
        assert_eq!(node.get_max_value(), &3);
    }
}
