use crate::structs::{
    hash_table::HashTable,
    tree::{
        nodes::btree::Node,
        object::{
            b_tree::{node_loader::NodeLoader, BTree},
            tree::Tree,
        },
    },
};

/// B-Tree iterator.
#[derive(Debug)]
pub struct BTreeIter<'a, T, L, M, const NODE_SIZE: u8>
where
    L: NodeLoader<T, NODE_SIZE>,
    T: Ord + Clone,
    M: HashTable<usize, Node<T, NODE_SIZE>>,
{
    tree: &'a BTree<T, NODE_SIZE, L, M>,

    current_node: Node<T, NODE_SIZE>,

    current_index: usize,
}

impl<'a, T, L, M, const NODE_SIZE: u8> BTreeIter<'a, T, L, M, NODE_SIZE>
where
    L: NodeLoader<T, NODE_SIZE>,
    T: Ord + Clone,
    M: HashTable<usize, Node<T, NODE_SIZE>>,
{
    /// Creates a new B-Tree iterator.
    pub fn new(
        tree: &'a BTree<T, NODE_SIZE, L, M>,
    ) -> BTreeIter<'a, T, L, M, NODE_SIZE> {
        let current_node = tree
            .data
            .get_node(tree.root.unwrap())
            .expect("root node must exist");
        let current_index = 0;

        BTreeIter {
            tree,
            current_node,
            current_index,
        }
    }

    /// Creates a new B-Tree iterator from a value.
    pub fn new_from_node(
        tree: &'a BTree<T, NODE_SIZE, L, M>,
        value: &T,
    ) -> BTreeIter<'a, T, L, M, NODE_SIZE> {
        let index = tree.find(value).expect("value must exist");
        let node = tree.data.get_node(index).expect("node must exist");

        match node {
            Node::Leaf(node) => {
                let index = node.get_position_by_value(value);
                BTreeIter {
                    tree,
                    current_node: Node::Leaf(node),
                    current_index: index,
                }
            }
            Node::Internal(_) => {
                panic!("value must be in a leaf node, not in an internal node")
            }
        }
    }
}

impl<'a, T, L, M, const NODE_SIZE: u8> Iterator for BTreeIter<'a, T, L, M, NODE_SIZE>
where
    L: NodeLoader<T, NODE_SIZE>,
    T: Ord + Clone,
    M: HashTable<usize, Node<T, NODE_SIZE>>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let current_node = &self.current_node;

        match current_node {
            Node::Leaf(node) => {
                let value = node.get(self.current_index);
                if let Some(value) = value {
                    self.current_index += 1;
                    Some(value.clone())
                } else {
                    let right = node.index.right;
                    if let Some(right) = right {
                        self.current_node = self.tree.data.get_node(right).expect("node must exist");
                        self.current_index = 0;
                        self.next()
                    } else {
                        None
                    }
                }
            }
            Node::Internal(_) => {
                panic!("value must be in a leaf node, not in an internal node")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::tree::nodes::btree::{Index, Node};
    use crate::structs::tree::object::b_tree::BTree;
    use crate::structs::tree::object::tree::Tree;

    use super::{BTreeIter, NodeLoader};

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
    fn test_iter() {
        let mut tree: BTree<u16, 3, MockNodeLoader> =
            BTree::new(MockNodeLoader {});

        tree.push(1);
        tree.push(2);
        tree.push(3);
        tree.push(4);
        tree.push(5);
        tree.push(6);
        tree.push(7);
        
        let mut iter = BTreeIter::new(&tree);
    }
}