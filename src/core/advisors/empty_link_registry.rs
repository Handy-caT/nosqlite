use crate::core::{
    link_struct::PageLink,
    structs::tree::{
        object::{
            balanced_tree::decoratable::Decoratable,
            tree::{Tree, VecFunctions},
        },
        vectors::tree_vec::{Indexes, Levels, TreeVec},
    },
};

pub struct EmptyLinkRegistry<V, M>
where
    V: TreeVec<PageLink> + Sized,
    M: Tree<PageLink> + Sized + VecFunctions<PageLink, V>,
{
    data: Decoratable<PageLink, V, M>,
}

impl<V, M> EmptyLinkRegistry<V, M>
where
    V: TreeVec<PageLink> + Levels + Sized,
    M: Tree<PageLink> + Sized + VecFunctions<PageLink, V>,
{
    pub fn new(data: Decoratable<PageLink, V, M>) -> Self {
        EmptyLinkRegistry { data }
    }

    pub fn add_link(&mut self, link: PageLink) {
        self.data.push(link);
    }

    pub fn remove_link(&mut self, link: PageLink) {
        self.data.remove_by_value(link);
    }

    pub fn pop(&mut self) -> Option<PageLink> {
        todo!()
    }

    pub(in crate::core::advisors) fn get_data(
        &self,
    ) -> &Decoratable<PageLink, V, M> {
        &self.data
    }

    pub(in crate::core::advisors) fn get_data_mut(
        &mut self,
    ) -> &mut Decoratable<PageLink, V, M> {
        &mut self.data
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{
        advisors::empty_link_registry::EmptyLinkRegistry,
        link_struct::PageLink,
        structs::tree::{
            object::{
                BalancedTree,
                balanced_tree::Decoratable,
                tree::Tree,
            },
            vectors::default_tree_vec::DefaultTreeVec,
        },
    };

    #[test]
    fn test_empty_link_registry_new() {
        let nodes = DefaultTreeVec::<PageLink>::new();
        let tree =
            BalancedTree::new_with_compare(nodes, PageLink::compare_by_index);

        let decoratable_tree =
            Decoratable::new(tree, PageLink::compare_by_len);

        let empty_link_registry = EmptyLinkRegistry::new(decoratable_tree);

        assert_eq!(empty_link_registry.data.len(), 0);
    }

    #[test]
    fn test_empty_link_registry_add_link() {
        let nodes = DefaultTreeVec::<PageLink>::new();
        let tree =
            BalancedTree::new_with_compare(nodes, PageLink::compare_by_index);

        let decoratable_tree =
            Decoratable::new(tree, PageLink::compare_by_len);

        let mut empty_link_registry = EmptyLinkRegistry::new(decoratable_tree);

        empty_link_registry.add_link(PageLink::new(0, 0, 20));

        assert_eq!(empty_link_registry.data.len(), 1);

        empty_link_registry.add_link(PageLink::new(0, 20, 10));

        assert_eq!(empty_link_registry.data.len(), 2);
    }
}
