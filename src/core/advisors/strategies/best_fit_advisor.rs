use crate::core::{
    advisors::{
        empty_link_registry::EmptyLinkRegistry,
        strategies::place_advisor_strategy::PlaceAdvisorStrategy,
    },
    link_struct::PageLink,
    structs::tree::{
        object::{
            BalancedTree,
            tree::{FindFunctions, Tree},
        },
        vectors::tree_vec::{Indexes, Levels, TreeVec},
    },
};

/// BestFitAdvisor is a strategy that finds the best fit for a given size.
/// It uses EmptyLinkRegistry with BalancedTree as a data structure.
/// So the getting the best fit is O(log n).
pub struct BestFitAdvisor<'a, V>
where
    V: TreeVec<PageLink> + Sized + Indexes<PageLink> + Levels,
{
    /// Link to the EmptyLinkRegistry
    empty_link_registry:
        &'a mut EmptyLinkRegistry<V, BalancedTree<PageLink, V>>,
}

impl<'a, V> BestFitAdvisor<'a, V>
where
    V: TreeVec<PageLink> + Sized + Indexes<PageLink> + Levels,
{
    /// Creates a new BestFitAdvisor
    /// # Arguments
    /// * `empty_link_registry` - Link to the EmptyLinkRegistry
    /// # Returns
    /// * `BestFitAdvisor` - New BestFitAdvisor
    pub fn new(
        empty_link_registry: &'a mut EmptyLinkRegistry<
            V,
            BalancedTree<PageLink, V>,
        >,
    ) -> Self {
        BestFitAdvisor {
            empty_link_registry,
        }
    }

    pub fn get_empty_link_registry(
        &self,
    ) -> &EmptyLinkRegistry<V, BalancedTree<PageLink, V>> {
        self.empty_link_registry
    }

    pub fn get_empty_link_registry_mut(
        &mut self,
    ) -> &mut EmptyLinkRegistry<V, BalancedTree<PageLink, V>> {
        self.empty_link_registry
    }
}

impl<'a, V> PlaceAdvisorStrategy for BestFitAdvisor<'a, V>
where
    V: TreeVec<PageLink> + Sized + Indexes<PageLink> + Levels,
{
    fn provide_place(&mut self, size: u64) -> Option<PageLink> {
        let data = self.empty_link_registry.get_data_mut();
        let base_obj = data.get_base_mut();

        if base_obj.len() == 0 {
            return None;
        }
        let link =
            base_obj.find_greater_equal(PageLink::new(0, 0, size as u16));

        match link {
            Some(link) => {
                let link = link.1;
                Some(link)
            }
            None => None,
        }
    }

    fn apply_place(&mut self, link: &PageLink, size: u64) {
        if u64::from(link.len) > size {
            let new_link = PageLink::new(
                link.page_index,
                link.start + size as u16,
                link.len - size as u16,
            );
            self.empty_link_registry.add_link(new_link);
        }

        self.empty_link_registry.remove_link(*link);
    }

    fn get_name(&self) -> String {
        "BestFitAdvisor".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{
        advisors::{
            empty_link_registry::EmptyLinkRegistry,
            strategies::{
                best_fit_advisor::BestFitAdvisor,
                place_advisor_strategy::PlaceAdvisorStrategy,
            },
        },
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
    fn test_best_fit_advisor_new() {
        let nodes = DefaultTreeVec::<PageLink>::new();
        let tree =
            BalancedTree::<PageLink, DefaultTreeVec<PageLink>>
            ::new_with_compare(
            nodes,
            PageLink::compare_by_len,
        );

        let dec_tree = Decoratable::<
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
        let tree =
            BalancedTree::<PageLink, DefaultTreeVec<PageLink>>
            ::new_with_compare(
            nodes,
            PageLink::compare_by_len,
        );

        let dec_tree = Decoratable::<
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
        let tree =
            BalancedTree::<PageLink, DefaultTreeVec<PageLink>>
            ::new_with_compare(
            nodes,
            PageLink::compare_by_len,
        );

        let dec_tree = Decoratable::<
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
        let tree =
            BalancedTree::<PageLink, DefaultTreeVec<PageLink>>
            ::new_with_compare(
            nodes,
            PageLink::compare_by_len,
        );

        let dec_tree = Decoratable::<
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
