use crate::core::structs::{
    hash_table::HashTable,
    tree::object::{b_tree::node::Node, tree::Tree},
};
use std::cmp::Ordering;
use crate::core::structs::tree::object::b_tree::node_loader::NodeLoader;
use crate::core::structs::tree::object::b_tree::node_vector::BTreeVec;

mod node;
mod node_loader;
mod node_vector;

struct BTree<T, L, M, const NODE_SIZE: u8> {
    data: BTreeVec<T, L, M, NODE_SIZE>,

    root: Option<usize>,

    compare: fn(&T, &T) -> Ordering,
}

impl<T, L, M, const NODE_SIZE: u8> BTree<T, L, M, NODE_SIZE>
where
    L: NodeLoader<T, NODE_SIZE>,
    T: Ord,
    M: HashTable<usize, Node<T, NODE_SIZE>>,
{
    pub fn new(node_loader: L) -> BTree<T, L, M, NODE_SIZE> {
        BTree {
            data: BTreeVec::new(node_loader),
            root: None,
            compare: |a, b| a.cmp(b),
        }
    }

    pub fn add_from_root(&mut self, root: usize, value: T) -> usize {
        let mut node = self.data.get_node(root).unwrap();
        if node.is_leaf() {
            if node.is_full() {
                todo!()
            } else {
                node.add_value(value);
                node.index.index
            }
        } else {
            let child = node.get_index_by_value(&value);
            if let Some(child) = child {
                self.add_from_root(child, value)
            } else if node.is_full() {
                todo!()
            } else {
                let leaf_index = self.data.add_leaf(value);
                
                node.add_link_index(leaf_index).unwrap();
                self.data.update_node(root, node);
                leaf_index
            }
        }
    }
}

impl<T, L, M, const NODE_SIZE: u8> Tree<T> for BTree<T, L, M, NODE_SIZE>
where
    L: NodeLoader<T, NODE_SIZE>,
    T: Ord + Clone,
    M: HashTable<usize, Node<T, NODE_SIZE>>,
{
    fn new_with_compare(compare: fn(&T, &T) -> Ordering) -> Self {
        todo!()
    }

    fn push(&mut self, value: T) -> usize {
        if let Some(root) = self.root {
            self.add_from_root(root, value)
        } else {
            let node_index = self.data.add_node();
            self.root = Some(node_index);
            
            let leaf_index = self.data.add_leaf(value.clone());
            
            let mut node = self.data.get_node(node_index).unwrap();
            node.add_value(value).unwrap();
            node.add_link_index(leaf_index).unwrap();
            self.data.update_node(node_index, node);

            leaf_index
        }
    }

    fn find(&mut self, value: &T) -> Option<usize> {
        todo!()
    }

    fn remove_by_value(&mut self, value: &T) -> Option<T> {
        todo!()
    }

    fn pop(&self) -> Option<T> {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::core::structs::{
        hash_table::{scalable_hash_table::ScalableHashTable, HashTable},
        tree::object::{
            b_tree::{node::Node, node_loader::NodeLoader, BTree},
            tree::Tree,
        },
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
    fn test_btree_new() {
        let tree: BTree<
            u16,
            MockNodeLoader,
            ScalableHashTable<usize, Node<u16, 3>>,
            3,
        > = BTree::new(MockNodeLoader {});
        
        assert!(tree.root.is_none());
    }

    #[test]
    fn test_btree_push_first() {
        let mut tree: BTree<
            u16,
            MockNodeLoader,
            ScalableHashTable<usize, Node<u16, 3>>,
            3,
        > = BTree::new(MockNodeLoader {});

        let index = tree.push(1);
        
        assert_eq!(tree.root, Some(0));
        assert_eq!(index, 1);
        
        let node = tree.data.get_node(0).unwrap();
        assert_eq!(node.keys.len(), 1);
        assert_eq!(node.keys[0], 1);
        assert_eq!(node.link_indexes.len(), 1);
        assert_eq!(node.link_indexes[0], 1);
        assert!(!node.is_leaf());
        
        let leaf = tree.data.get_node(1).unwrap();
        assert_eq!(leaf.keys.len(), 1);
        assert_eq!(leaf.keys[0], 1);
        assert!(leaf.is_leaf());
    }
    
    #[test]
    fn test_btree_push_second() {
        let mut tree: BTree<
            u16,
            MockNodeLoader,
            ScalableHashTable<usize, Node<u16, 3>>,
            3,
        > = BTree::new(MockNodeLoader {});

        tree.push(1);
        let index = tree.push(2);
        
        assert_eq!(tree.root, Some(0));
        assert_eq!(index, 2);
        
        let node = tree.data.get_node(0).unwrap();
        assert_eq!(node.keys.len(), 1);
        assert_eq!(node.keys[0], 1);
        assert_eq!(node.link_indexes.len(), 2);
        assert_eq!(node.link_indexes[0], 1);
        assert_eq!(node.link_indexes[1], 2);
        assert!(!node.is_leaf());
        
        let leaf = tree.data.get_node(1).unwrap();
        assert_eq!(leaf.keys.len(), 1);
        assert_eq!(leaf.keys[0], 1);
        assert!(leaf.is_leaf());
        
        let leaf = tree.data.get_node(2).unwrap();
        assert_eq!(leaf.keys.len(), 1);
        assert_eq!(leaf.keys[0], 2);
        assert!(leaf.is_leaf());
    }
}
