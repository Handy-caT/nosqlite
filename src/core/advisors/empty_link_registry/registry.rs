use crate::core::{
    link_struct::PageLink,
    structs::tree::{
        object::{
            balanced_tree::Decoratable,
            tree::{Tree, VecFunctions},
        },
        vectors::tree_vec::{Levels, TreeVec},
    },
};

/// Trait for empty link registry.
pub trait EmptyLinkStorage {
    /// Adds a link to the registry.
    /// # Arguments
    /// * `link` - Link to add.
    fn add_link(&mut self, link: PageLink);

    /// Removes a link from the registry.
    /// # Arguments
    /// * `link` - Link to remove.
    fn remove_link(&mut self, link: PageLink);

    /// Pops a link from the registry. It deletes the link
    /// from the registry without any order.
    /// # Returns
    /// * `Option<PageLink>` - Link that was popped.
    fn pop(&mut self) -> Option<PageLink>;
}

pub struct Registry<V, M>
where
    V: TreeVec<PageLink> + Sized,
    M: Tree<PageLink> + Sized + VecFunctions<PageLink, V>,
{
    data: Decoratable<PageLink, V, M>,
}

impl<V, M> Registry<V, M>
where
    V: TreeVec<PageLink> + Levels + Sized,
    M: Tree<PageLink> + Sized + VecFunctions<PageLink, V>,
{
    pub(in crate::core::advisors::empty_link_registry) fn new(
        data: Decoratable<PageLink, V, M>,
    ) -> Self {
        Registry { data }
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

    /// Gets the length of the registry.
    /// # Returns
    /// * `usize` - Length of the registry.
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<V, M> EmptyLinkStorage for Registry<V, M>
where
    V: TreeVec<PageLink> + Levels + Sized,
    M: Tree<PageLink> + Sized + VecFunctions<PageLink, V>,
{
    fn add_link(&mut self, link: PageLink) {
        self.data.push(link);
    }

    fn remove_link(&mut self, link: PageLink) {
        self.data.remove_by_value(link);
    }

    fn pop(&mut self) -> Option<PageLink> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{
        advisors::empty_link_registry::registry::Registry,
        link_struct::PageLink,
        structs::tree::{
            object::{balanced_tree::Decoratable, tree::Tree, BalancedTree},
            vectors::default_tree_vec::DefaultTreeVec,
        },
    };
    use crate::core::advisors::empty_link_registry::registry::EmptyLinkStorage;

    #[test]
    fn test_empty_link_registry_new() {
        let tree = BalancedTree::<_, DefaultTreeVec<_>>::new_with_compare(
            PageLink::compare_by_index,
        );

        let decoratable_tree =
            Decoratable::new_with_existing(tree, PageLink::compare_by_len);

        let empty_link_registry = Registry::new(decoratable_tree);

        assert_eq!(empty_link_registry.data.len(), 0);
    }

    #[test]
    fn test_empty_link_registry_add_link() {
        let tree = BalancedTree::<_, DefaultTreeVec<_>>::new_with_compare(
            PageLink::compare_by_index,
        );

        let decoratable_tree =
            Decoratable::new_with_existing(tree, PageLink::compare_by_len);

        let mut empty_link_registry = Registry::new(decoratable_tree);

        empty_link_registry.add_link(PageLink::new(0, 0, 20));

        assert_eq!(empty_link_registry.data.len(), 1);

        empty_link_registry.add_link(PageLink::new(0, 20, 10));

        assert_eq!(empty_link_registry.data.len(), 2);
    }
}
