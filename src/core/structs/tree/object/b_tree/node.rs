
/// Node of a B-Tree.
#[derive(Debug)]
pub struct Node<T> {
    /// Vector that stores the keys.
    keys: Vec<T>,
    
    /// Vector that stores the children indexes.
    link_indexes: Vec<u32>,
    
    /// Next node index.
    next: u32,
    
    /// Max size of the node.
    max_size: u16,
}

impl<T> Node<T>
where T: Ord
{
    /// Creates a new node.
    /// # Arguments
    /// * `max_size` - Maximum size of the node.
    /// # Returns
    /// * Node<T> - New node.
    pub fn new(max_size: u16) -> Node<T> {
        Node {
            keys: Vec::new(),
            link_indexes: Vec::new(),
            next: 0,
            max_size,
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
        let node = Node::<i32>::new(3);
        assert_eq!(node.keys.len(), 0);
        assert_eq!(node.link_indexes.len(), 0);
        assert_eq!(node.next, 0);
        assert_eq!(node.max_size, 3);
    }

    #[test]
    fn test_node_add_value() {
        let mut node = Node::<i32>::new(3);
        let res = node.add_value(1);
        
        assert!(res.is_ok());
        assert_eq!(node.keys.len(), 1);
        assert_eq!(node.keys[0], 1);
    }
    
    #[test]
    fn test_node_add_value_invalid() {
        let mut node = Node::<i32>::new(3);
        let _ = node.add_value(2);
        let res = node.add_value(1);
        
        assert!(res.is_err());
        assert_eq!(node.keys.len(), 1);
        assert_eq!(node.keys[0], 2);
    }
    
    #[test]
    fn test_node_add_link_index() {
        let mut node = Node::<i32>::new(3);
        let _ = node.add_value(1);
        let _ = node.add_link_index(0);
        let _ = node.add_link_index(1);
        
        assert_eq!(node.link_indexes.len(), 2);
        assert_eq!(node.link_indexes[0], 0);
        assert_eq!(node.link_indexes[1], 1);
    }

    #[test]
    fn test_node_add_link_index_invalid() {
        let mut node = Node::<i32>::new(3);
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
        let mut node = Node::<i32>::new(3);
        node.add_value(1);
        let value = node.pop_value();
        
        assert_eq!(value.unwrap(), 1);
        assert_eq!(node.keys.len(), 0);
    }
    
    #[test]
    fn test_node_pop_link_index() {
        let mut node = Node::<i32>::new(3);
        node.add_link_index(1);
        let index = node.pop_link_index();
        
        assert_eq!(index.unwrap(), 1);
        assert_eq!(node.link_indexes.len(), 0);
    }
    
    #[test]
    fn test_node_get_index_by_value() {
        let mut node = Node::<i32>::new(3);
        
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
}
