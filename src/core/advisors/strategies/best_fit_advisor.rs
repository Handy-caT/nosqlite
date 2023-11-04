use crate::core::advisors::empty_link_registry::EmptyLinkRegistry;
use crate::core::advisors::strategies::place_advisor_strategy::PlaceAdvisorStrategy;
use crate::core::link_struct::PageLink;
use crate::core::structs::tree::object::balanced_tree::balanced_tree::BalancedTree;
use crate::core::structs::tree::object::tree_object::{TreeObject, TreeObjectFind};
use crate::core::structs::tree::vectors::tree_vec::{TreeVec, TreeVecIndexes, TreeVecLevels};

/// BestFitAdvisor is a strategy that finds the best fit for a given size.
/// It uses EmptyLinkRegistry with BalancedTree as a data structure. So the getting the best fit is O(log n).
pub struct BestFitAdvisor<'a, V>
where
    V: TreeVec<PageLink> + Sized + TreeVecIndexes<PageLink> + TreeVecLevels,
{
    /// Link to the EmptyLinkRegistry
    empty_link_registry: &'a mut EmptyLinkRegistry<V, BalancedTree<PageLink, V>>,
}

impl<'a, V> BestFitAdvisor<'a, V>
where
    V: TreeVec<PageLink> + Sized + TreeVecIndexes<PageLink> + TreeVecLevels,
{
    /// Creates a new BestFitAdvisor
    /// # Arguments
    /// * `empty_link_registry` - Link to the EmptyLinkRegistry
    /// # Returns
    /// * `BestFitAdvisor` - New BestFitAdvisor
    pub fn new(
        empty_link_registry: &'a mut EmptyLinkRegistry<V, BalancedTree<PageLink, V>>,
    ) -> Self {
        BestFitAdvisor {
            empty_link_registry,
        }
    }

    pub fn get_empty_link_registry(&self) -> &EmptyLinkRegistry<V, BalancedTree<PageLink, V>> {
        &self.empty_link_registry
    }

    pub fn get_empty_link_registry_mut(
        &mut self,
    ) -> &mut EmptyLinkRegistry<V, BalancedTree<PageLink, V>> {
        &mut self.empty_link_registry
    }
}

impl<'a, V> PlaceAdvisorStrategy for BestFitAdvisor<'a, V>
where
    V: TreeVec<PageLink> + Sized + TreeVecIndexes<PageLink> + TreeVecLevels,
{
    fn provide_place(&mut self, size: u64) -> Option<PageLink> {
        let data = self.empty_link_registry.get_data_mut();
        let base_obj = data.get_base_mut();

        if base_obj.len() == 0 {
            return None;
        }
        let link = base_obj.find_greater_equal(PageLink::new(0, 0, size as u32));

        match link {
            Some(link) => {
                let link = link.1;
                Some(link)
            }
            None => None,
        }
    }

    fn apply_place(&mut self, link: &PageLink, size: u64) {
        if link.len > size as u32 {
            let new_link = PageLink::new(
                link.page_index,
                link.start + size as u32,
                link.len - size as u32,
            );
            self.empty_link_registry.add_link(new_link);
        }

        self.empty_link_registry.remove_link(link.clone());
    }

    fn get_name(&self) -> String {
        "BestFitAdvisor".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::advisors::empty_link_registry::EmptyLinkRegistry;
    use crate::core::advisors::strategies::best_fit_advisor::BestFitAdvisor;
    use crate::core::advisors::strategies::place_advisor_strategy::PlaceAdvisorStrategy;
    use crate::core::link_struct::PageLink;
    use crate::core::structs::tree::object::balanced_tree::balanced_tree::BalancedTree;
    use crate::core::structs::tree::object::balanced_tree::decoratable_balanced_tree::DecoratableBalancedTree;
    use crate::core::structs::tree::object::tree_object::TreeObject;
    use crate::core::structs::tree::vectors::default_tree_vec::DefaultTreeVec;

    #[test]
    fn test_best_fit_advisor_new() {
        let nodes = DefaultTreeVec::<PageLink>::new();
        let tree = BalancedTree::<PageLink, DefaultTreeVec<PageLink>>::new_with_compare(
            nodes,
            PageLink::compare_by_len,
        );

        let dec_tree = DecoratableBalancedTree::<
            PageLink,
            DefaultTreeVec<PageLink>,
            BalancedTree<PageLink, DefaultTreeVec<PageLink>>,
        >::new(tree, PageLink::compare_by_index);

        let mut registry = EmptyLinkRegistry::<
            DefaultTreeVec<PageLink>,
            BalancedTree<PageLink, DefaultTreeVec<PageLink>>,
        >::new(dec_tree);
        let advisor = BestFitAdvisor::new(&mut registry);

        assert_eq!(advisor.get_name(), "BestFitAdvisor".to_string());
    }

    #[test]
    fn test_best_fit_advisor_provide_place() {
        let nodes = DefaultTreeVec::<PageLink>::new();
        let tree = BalancedTree::<PageLink, DefaultTreeVec<PageLink>>::new_with_compare(
            nodes,
            PageLink::compare_by_len,
        );

        let dec_tree = DecoratableBalancedTree::<
            PageLink,
            DefaultTreeVec<PageLink>,
            BalancedTree<PageLink, DefaultTreeVec<PageLink>>,
        >::new(tree, PageLink::compare_by_index);

        let mut registry = EmptyLinkRegistry::<
            DefaultTreeVec<PageLink>,
            BalancedTree<PageLink, DefaultTreeVec<PageLink>>,
        >::new(dec_tree);

        registry.add_link(PageLink::new(0, 0, 100));
        registry.add_link(PageLink::new(0, 100, 200));

        let mut advisor = BestFitAdvisor::new(&mut registry);

        let link = advisor.provide_place(100);

        assert_eq!(link, Some(PageLink::new(0, 0, 100)));

        let link = advisor.provide_place(300);

        assert_eq!(link, None);
    }

    #[test]
    fn test_best_fit_advisor_provide_place_with_empty_registry() {
        let nodes = DefaultTreeVec::<PageLink>::new();
        let tree = BalancedTree::<PageLink, DefaultTreeVec<PageLink>>::new_with_compare(
            nodes,
            PageLink::compare_by_len,
        );

        let dec_tree = DecoratableBalancedTree::<
            PageLink,
            DefaultTreeVec<PageLink>,
            BalancedTree<PageLink, DefaultTreeVec<PageLink>>,
        >::new(tree, PageLink::compare_by_index);

        let mut registry = EmptyLinkRegistry::<
            DefaultTreeVec<PageLink>,
            BalancedTree<PageLink, DefaultTreeVec<PageLink>>,
        >::new(dec_tree);
        let mut advisor = BestFitAdvisor::new(&mut registry);

        let link = advisor.provide_place(100);

        assert_eq!(link, None);
    }

    #[test]
    fn test_best_fit_advisor_apply_place() {
        let nodes = DefaultTreeVec::<PageLink>::new();
        let tree = BalancedTree::<PageLink, DefaultTreeVec<PageLink>>::new_with_compare(
            nodes,
            PageLink::compare_by_len,
        );

        let dec_tree = DecoratableBalancedTree::<
            PageLink,
            DefaultTreeVec<PageLink>,
            BalancedTree<PageLink, DefaultTreeVec<PageLink>>,
        >::new(tree, PageLink::compare_by_index);

        let mut registry = EmptyLinkRegistry::<
            DefaultTreeVec<PageLink>,
            BalancedTree<PageLink, DefaultTreeVec<PageLink>>,
        >::new(dec_tree);

        registry.add_link(PageLink::new(0, 0, 100));
        registry.add_link(PageLink::new(0, 100, 200));

        let mut advisor = BestFitAdvisor::new(&mut registry);

        let link = advisor.provide_place(100);

        assert_eq!(link, Some(PageLink::new(0, 0, 100)));

        advisor.apply_place(&link.unwrap(), 100);

        assert_eq!(registry.get_data().len(), 1);
    }
}
