use crate::{
    core::structs::tree::{
        object::{
            tree::{FindFunctions, Tree},
            BalancedTree,
        },
        vectors::tree_vec::{Indexes, Levels, TreeVec},
    },
    data::advisors::{
        empty_link_registry::registry::{EmptyLinkStorage, Registry},
        strategies::place_advisor_strategy::PlaceAdvisorStrategy,
    },
    page::link_struct::PageLink,
};

/// [`BestFitAdvisor`] is a strategy that finds the best fit for a given size.
/// It uses [`Registry`] with [`BalancedTree`] as a data structure.
/// So the getting the best fit is O(log n).
pub struct BestFitAdvisor<'a, V>
where
    V: TreeVec<PageLink> + Sized + Indexes<PageLink> + Levels,
{
    /// Link to the [`Registry`]
    empty_link_registry: &'a mut Registry<V, BalancedTree<PageLink, V>>,
}

impl<'a, V> BestFitAdvisor<'a, V>
where
    V: TreeVec<PageLink> + Sized + Indexes<PageLink> + Levels,
{
    /// Creates a new [`BestFitAdvisor`]
    /// # Arguments
    /// * `empty_link_registry` - Link to the [`Registry`]
    /// # Returns
    /// * `BestFitAdvisor` - New [`BestFitAdvisor`]
    pub fn new(
        empty_link_registry: &'a mut Registry<V, BalancedTree<PageLink, V>>,
    ) -> Self {
        BestFitAdvisor {
            empty_link_registry,
        }
    }

    pub fn get_empty_link_registry(
        &self,
    ) -> &Registry<V, BalancedTree<PageLink, V>> {
        self.empty_link_registry
    }

    pub fn get_empty_link_registry_mut(
        &mut self,
    ) -> &mut Registry<V, BalancedTree<PageLink, V>> {
        self.empty_link_registry
    }
}

impl<'a, V> PlaceAdvisorStrategy for BestFitAdvisor<'a, V>
where
    V: TreeVec<PageLink> + Sized + Indexes<PageLink> + Levels,
{
    fn provide_place(&mut self, size: u16) -> Option<PageLink> {
        let data = self.empty_link_registry.get_data_mut();
        let base_obj = data.get_base_mut();

        if base_obj.len() == 0 {
            return None;
        }
        let link = base_obj.find_greater_equal(PageLink::new(0, 0, size));

        match link {
            Some(link) => {
                let link = link.1;
                Some(link)
            }
            None => None,
        }
    }

    fn apply_place(&mut self, link: &PageLink, size: u16) {
        if link.len > size {
            let new_link = PageLink::new(
                link.page_index,
                link.start + size,
                link.len - size,
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
    use crate::{
        core::structs::tree::{
            object::tree::Tree, vectors::optimized_tree_vec::OptimizedTreeVec,
        },
        data::advisors::{
            empty_link_registry::{
                factory::{
                    BestFitEmptyLinkRegistryFactory, EmptyLinkRegistryFactory,
                },
                registry::EmptyLinkStorage as _,
                EmptyLinkRegistry,
            },
            strategies::{
                best_fit_advisor::BestFitAdvisor,
                place_advisor_strategy::PlaceAdvisorStrategy,
            },
        },
        page::link_struct::PageLink,
    };

    #[test]
    fn test_best_fit_advisor_new() {
        let registry =
            BestFitEmptyLinkRegistryFactory::create_empty_link_registry();
        let EmptyLinkRegistry::BestFit(mut registry) = registry else {
            panic!("Wrong type of registry");
        };
        let advisor: BestFitAdvisor<'_, OptimizedTreeVec<PageLink>> =
            BestFitAdvisor::<'_, OptimizedTreeVec<PageLink>>::new(
                &mut registry,
            );

        assert_eq!(advisor.get_name(), "BestFitAdvisor".to_string());
    }

    #[test]
    fn test_best_fit_advisor_provide_place() {
        let registry =
            BestFitEmptyLinkRegistryFactory::create_empty_link_registry();
        let EmptyLinkRegistry::BestFit(mut registry) = registry else {
            panic!("Wrong type of registry");
        };

        registry.add_link(PageLink::new(0, 0, 100));
        registry.add_link(PageLink::new(0, 100, 200));

        let mut advisor: BestFitAdvisor<'_, OptimizedTreeVec<PageLink>> =
            BestFitAdvisor::<'_, OptimizedTreeVec<PageLink>>::new(
                &mut registry,
            );

        let link = advisor.provide_place(100);

        assert_eq!(link, Some(PageLink::new(0, 0, 100)));

        let link = advisor.provide_place(300);

        assert_eq!(link, None);
    }

    #[test]
    fn test_best_fit_advisor_provide_place_with_empty_registry() {
        let registry =
            BestFitEmptyLinkRegistryFactory::create_empty_link_registry();
        let EmptyLinkRegistry::BestFit(mut registry) = registry else {
            panic!("Wrong type of registry");
        };
        let mut advisor: BestFitAdvisor<'_, OptimizedTreeVec<PageLink>> =
            BestFitAdvisor::<'_, OptimizedTreeVec<PageLink>>::new(
                &mut registry,
            );

        let link = advisor.provide_place(100);

        assert_eq!(link, None);
    }

    #[test]
    fn test_best_fit_advisor_apply_place() {
        let registry =
            BestFitEmptyLinkRegistryFactory::create_empty_link_registry();
        let EmptyLinkRegistry::BestFit(mut registry) = registry else {
            panic!("Wrong type of registry");
        };

        registry.add_link(PageLink::new(0, 0, 100));
        registry.add_link(PageLink::new(0, 100, 200));

        let mut advisor: BestFitAdvisor<'_, OptimizedTreeVec<PageLink>> =
            BestFitAdvisor::<'_, OptimizedTreeVec<PageLink>>::new(
                &mut registry,
            );

        let link = advisor.provide_place(100);

        assert_eq!(link, Some(PageLink::new(0, 0, 100)));

        advisor.apply_place(&link.unwrap(), 100);

        assert_eq!(registry.get_data().len(), 1);
    }

    #[test]
    fn test_best_fit_advisor_apply_place_bigger_link() {
        let registry =
            BestFitEmptyLinkRegistryFactory::create_empty_link_registry();
        let EmptyLinkRegistry::BestFit(mut registry) = registry else {
            panic!("Wrong type of registry");
        };

        registry.add_link(PageLink::new(0, 0, 200));

        let mut advisor: BestFitAdvisor<'_, OptimizedTreeVec<PageLink>> =
            BestFitAdvisor::<'_, OptimizedTreeVec<PageLink>>::new(
                &mut registry,
            );

        let link = advisor.provide_place(100);

        assert!(link.is_some());
        let link = link.unwrap();

        assert_eq!(link.len, 200);

        advisor.apply_place(&link, 100);

        assert_eq!(registry.get_data().len(), 1);
    }
}
