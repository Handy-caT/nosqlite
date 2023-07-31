use crate::core::link_struct::PageLink;
use crate::core::structs::tree::object::balanced_tree::decoratable_balanced_tree::DecoratableBalancedTree;
use crate::core::structs::tree::object::tree_object::{TreeObject, TreeObjectVec};
use crate::core::structs::tree::vectors::tree_vec::{TreeVec, TreeVecIndexes, TreeVecLevels};

struct EmptyLinkRegistry<V, M>
    where
        V: TreeVec<PageLink> + Sized,
        M: TreeObject<PageLink> + Sized + TreeObjectVec<PageLink, V>
{
    data: DecoratableBalancedTree<PageLink, V, M>
}

impl <V, M> EmptyLinkRegistry<V, M>
    where
        V: TreeVec<PageLink> + TreeVecLevels + Sized + TreeVecIndexes<PageLink>,
        M: TreeObject<PageLink> + Sized + TreeObjectVec<PageLink, V>
{
    pub fn new(data: DecoratableBalancedTree<PageLink, V, M>) -> Self {
        EmptyLinkRegistry {
            data
        }
    }

    pub fn add_link(&mut self, link: PageLink) {
        self.data.push(link);
    }

    pub fn remove_link(&mut self, link: PageLink) {
        self.data.remove_by_value(link);
    }

    pub fn pop(&mut self) -> Option<PageLink> {
        self.data.remove_by_index(0)
    }
}


#[cfg(test)]
mod tests {
    use crate::core::advisors::empty_link_registry::EmptyLinkRegistry;
    use crate::core::link_struct::PageLink;
    use crate::core::structs::tree::object::balanced_tree::balanced_tree::BalancedTree;
    use crate::core::structs::tree::object::balanced_tree::decoratable_balanced_tree::DecoratableBalancedTree;
    use crate::core::structs::tree::object::tree_object::{TreeObject, TreeObjectVec};
    use crate::core::structs::tree::vectors::default_tree_vec::DefaultTreeVec;

    #[test]
    fn test_empty_link_registry_new() {
        let nodes = DefaultTreeVec::<PageLink>::new();
        let tree = BalancedTree::new_with_compare(nodes, PageLink::compare_by_index);

        let decoratable_tree = DecoratableBalancedTree::new(tree, PageLink::compare_by_len);

        let empty_link_registry = EmptyLinkRegistry::new(decoratable_tree);

        assert_eq!(empty_link_registry.data.len(), 0);
    }

    #[test]
    fn test_empty_link_registry_add_link() {
        let nodes = DefaultTreeVec::<PageLink>::new();
        let tree = BalancedTree::new_with_compare(nodes, PageLink::compare_by_index);

        let decoratable_tree = DecoratableBalancedTree::new(tree, PageLink::compare_by_len);

        let mut empty_link_registry = EmptyLinkRegistry::new(decoratable_tree);

        empty_link_registry.add_link(PageLink::new(0, 0, 20));

        assert_eq!(empty_link_registry.data.len(), 1);

        empty_link_registry.add_link(PageLink::new(0, 20, 10));

        assert_eq!(empty_link_registry.data.len(), 2);
    }
}
