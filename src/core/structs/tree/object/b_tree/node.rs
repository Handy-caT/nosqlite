
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
where T: PartialOrd
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
    /// * `index` - Child node index.
    pub fn add_value(&mut self, value: T, index: u32) -> Result<(), Error> {
        if let Some(last) = self.keys.last() {
            if last > &value {
                return Err(Error::InvalidValue);
            }
        }
        
        self.keys.push(value);
        self.link_indexes.push(index);
        
        Ok(())
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
        let res = node.add_value(1, 0);
        
        assert!(res.is_ok());
        assert_eq!(node.keys.len(), 1);
        assert_eq!(node.link_indexes.len(), 1);
        assert_eq!(node.keys[0], 1);
        assert_eq!(node.link_indexes[0], 0);
    }
}
