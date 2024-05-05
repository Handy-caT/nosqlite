use crate::structs::{
    hash_table::{scalable::ScalableHashTable, HashTable},
    tree::{
        nodes::btree::{internal::Internal, leaf::Leaf, Node},
        object::{
            b_tree::{
                node_loader::{BaseLoader, NodeLoader},
                node_vector::BTreeVec,
            },
            tree::Tree,
        },
    },
};
use std::cmp::Ordering;

mod node_loader;
mod node_vector;

#[derive(Debug, Clone)]
pub struct BTree<
    T,
    const NODE_SIZE: u8,
    L = BaseLoader,
    M = ScalableHashTable<usize, Node<T, NODE_SIZE>>,
> {
    data: BTreeVec<T, L, M, NODE_SIZE>,

    root: Option<usize>,

    compare: fn(&T, &T) -> Ordering,

    len: usize,
}
//
impl<T, L, M, const NODE_SIZE: u8> BTree<T, NODE_SIZE, L, M>
where
    L: NodeLoader<T, NODE_SIZE>,
    T: Ord + Clone,
    M: HashTable<usize, Node<T, NODE_SIZE>>,
{
    pub fn new(node_loader: L) -> BTree<T, NODE_SIZE, L, M> {
        BTree {
            data: BTreeVec::new(node_loader),
            root: None,
            compare: Ord::cmp,
            len: 0,
        }
    }
    
    pub fn update_parent(&mut self, index: usize) {
        let node = self.data.get_node(index).unwrap();
        
        if let Node::Internal(node) = node {
            for child in node.get_children() {
                let child_node = self.data.get_node(child).unwrap();
                match child_node {
                    Node::Leaf(mut child_node) => {
                        child_node.index.parent = Some(index);
                        self.data.update_node(child, Node::Leaf(child_node));
                    }
                    Node::Internal(mut child_node) => {
                        child_node.index.parent = Some(index);
                        self.data.update_node(child, Node::Internal(child_node));
                    }
                }
            }
            self.data.update_node(index, Node::Internal(node));
        }
    }

    pub fn split_node(&mut self, index: usize) -> usize {
        let node = self.data.get_node(index).unwrap();
        let new_index = self.data.get_next_index();
        let (node_max, split_max, parent) = match node {
            Node::Leaf(mut node) => {
                let split = node.split(new_index);

                let split_max = split.get_max_value().clone();
                let node_max = node.get_max_value().clone();
                
                self.data.add_node(Node::Leaf(split));
                let parent = node.index.parent;

                (node_max, split_max, parent)
            }
            Node::Internal(mut node) => {
                let split = node.split(new_index);

                let split_max = split.get_max_value().clone();
                let node_max = node.get_max_value().clone();
                
                self.data.add_node(Node::Internal(split));
                self.update_parent(new_index);
                let parent = node.index.parent;

                (node_max, split_max, parent)
            }
        };
        
        if let Some(parent) = parent {
            let parent_node = self.data.get_node(parent).unwrap();
            match parent_node {
                Node::Internal(mut parent_node) => {
                    if parent_node.is_full() {
                        self.split_node(parent)
                    } else {
                        parent_node
                            .add_value(
                                node_max,
                                new_index,
                            )
                            .expect("not full because of check before");
                        self.data.update_node(
                            parent,
                            Node::Internal(parent_node),
                        );

                        parent
                    }
                }
                _ => panic!("parent is not internal"),
            }
        } else {
            // Only if root
            let new_root_index = self.data.get_next_index();
            let mut new_node = Internal::new(new_root_index);

            new_node
                .add_value(node_max, index)
                .expect("not full because first");
            new_node
                .add_value(split_max, new_index)
                .expect("not full because second");

            self.root = Some(new_root_index);
            self.data.add_node(Node::Internal(new_node));
            self.update_parent(new_root_index);

            new_root_index
        }
    }

    pub fn add_from_root(&mut self, root: usize, value: T) -> usize {
        let node = self.data.get_node(root).unwrap();
        match node {
            Node::Leaf(mut node) => {
                if node.is_full() {
                    let index = self.split_node(root);
                    self.add_from_root(index, value)
                } else {
                    node.add_value(value)
                        .expect("not full because of check before");
                    let result = node.index.index;
                    self.data.update_node(root, Node::Leaf(node));
                    result
                }
            }
            Node::Internal(node) => {
                let child = node.get_index_by_value(&value);
                self.add_from_root(child, value)
            }
        }
    }
}

impl<T, L, M, const NODE_SIZE: u8> Tree<T> for BTree<T, NODE_SIZE, L, M>
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
            let res = self.add_from_root(root, value);
            self.len += 1;
            res
        } else {
            let mut node = Leaf::new(0);
            node.push_value(value).expect("first value must be valid");
            let node_index = self.data.add_node(Node::Leaf(node));
            self.root = Some(node_index);

            self.len += 1;

            node_index
        }
    }

    fn find(&self, value: &T) -> Option<usize> {
        if let Some(root) = self.root {
            let mut current = root;
            loop {
                let node = self
                    .data
                    .get_node(current)
                    .expect("exists because index is valid");
                match node {
                    Node::Leaf(node) => {
                        return if node.contains_value(value) {
                            Some(current)
                        } else {
                            None
                        };
                    }
                    Node::Internal(node) => {
                        let child = node.get_index_by_value(value);
                        current = child;
                    }
                }
            }
        } else {
            None
        }
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
        self.len
    }
}

impl<T, L, M, const NODE_SIZE: u8> Default for BTree<T, NODE_SIZE, L, M>
where
    L: NodeLoader<T, NODE_SIZE> + Default,
    T: Ord + Clone,
    M: HashTable<usize, Node<T, NODE_SIZE>>,
{
    fn default() -> Self {
        Self::new(L::default())
    }
}

#[cfg(test)]
mod test {
    use crate::structs::tree::{
        nodes::btree::Node,
        object::{
            b_tree::{node_loader::NodeLoader, BTree},
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
        let tree: BTree<u16, 3, MockNodeLoader> = BTree::new(MockNodeLoader {});

        assert!(tree.root.is_none());
    }

    #[test]
    fn test_btree_push_first() {
        let mut tree: BTree<u16, 3, MockNodeLoader> =
            BTree::new(MockNodeLoader {});

        let index = tree.push(1);

        assert_eq!(tree.root, Some(0));
        assert_eq!(tree.len, 1);
        assert_eq!(index, 0);

        let node = tree.data.get_node(0).unwrap();
        match node {
            Node::Leaf(node) => {
                assert_eq!(node.len(), 1);
                assert_eq!(node.get(0), Some(&1));
            }
            _ => panic!("node is not leaf"),
        }
    }

    #[test]
    fn test_btree_push_second_bigger() {
        let mut tree: BTree<u16, 3, MockNodeLoader> =
            BTree::new(MockNodeLoader {});

        tree.push(1);
        let index = tree.push(2);

        assert_eq!(tree.root, Some(0));
        assert_eq!(tree.len, 2);
        assert_eq!(index, 0);

        let node = tree.data.get_node(0).unwrap();
        match node {
            Node::Leaf(node) => {
                assert_eq!(node.len(), 2);
                assert_eq!(node.get(0), Some(&1));
                assert_eq!(node.get(1), Some(&2));
            }
            _ => panic!("node is not leaf"),
        }
    }

    #[test]
    fn test_btree_push_second_lower() {
        let mut tree: BTree<u16, 3, MockNodeLoader> =
            BTree::new(MockNodeLoader {});

        tree.push(2);
        let index = tree.push(1);

        assert_eq!(tree.root, Some(0));
        assert_eq!(index, 0);

        let node = tree.data.get_node(0).unwrap();
        match node {
            Node::Leaf(node) => {
                assert_eq!(node.len(), 2);
                assert_eq!(node.get(0), Some(&1));
                assert_eq!(node.get(1), Some(&2));
            }
            _ => panic!("node is not leaf"),
        }
    }

    #[test]
    fn test_btree_find() {
        let mut tree: BTree<u16, 3, MockNodeLoader> =
            BTree::new(MockNodeLoader {});

        tree.push(2);
        tree.push(1);

        let index = tree.find(&1);

        assert_eq!(index, Some(0));
    }
    
    #[test]
    fn splits_node() {
        let mut tree: BTree<u16, 3, MockNodeLoader> =
            BTree::new(MockNodeLoader {});

        tree.push(1);
        tree.push(2);
        tree.push(3);
        let index = tree.push(4);
        
        assert_eq!(tree.root, Some(2));
        assert_eq!(index, 1);
    }
}
