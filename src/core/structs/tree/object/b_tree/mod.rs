use crate::core::structs::{
    hash_table::HashTable,
    tree::object::{b_tree::node::Node, tree::Tree},
};
use std::cmp::Ordering;

mod node;
mod node_loader;
mod node_vector;

struct BTree<T, L, M, const NODE_SIZE: u8> {
    preloaded_data: M,

    root: Option<usize>,

    node_loader: L,

    compare: fn(&T, &T) -> Ordering,
}

impl<T, L, M, const NODE_SIZE: u8> BTree<T, L, M, NODE_SIZE>
where
    T: Ord,
    M: HashTable<usize, Node<T, NODE_SIZE>>,
{
    pub fn new(node_loader: L) -> BTree<T, L, M, NODE_SIZE> {
        BTree {
            preloaded_data: M::new(8),
            root: None,
            node_loader,
            compare: |a, b| a.cmp(b),
        }
    }

    pub fn add_from_root(&mut self, root: usize, value: T) -> usize {
        todo!("Add from root");
    }
}

impl<T, L, M, const NODE_SIZE: u8> Tree<T> for BTree<T, L, M, NODE_SIZE>
where
    T: Ord,
    M: HashTable<usize, Node<T, NODE_SIZE>>,
{
    fn new_with_compare(compare: fn(&T, &T) -> Ordering) -> Self {
        todo!()
    }

    fn push(&mut self, value: T) -> usize {
        if let Some(root) = self.root {
            self.add_from_root(root, value)
        } else {
            // let node_index = self.add_node();
            // let leaf_index = self.add_leaf(value);
            //
            // let mut node = self

            todo!("Push to empty tree");
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

        //assert!(tree.preloaded_data.l);
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

        assert_eq!(tree.preloaded_data.len(), 1);
        assert_eq!(tree.root, Some(0));
        assert_eq!(index, 0);
    }
}
