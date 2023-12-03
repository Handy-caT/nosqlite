use crate::core::{
    advisors::{
        empty_link_registry::registry::Registry,
        strategies::place_advisor_strategy::PlaceAdvisorStrategy,
    },
    link_struct::PageLink,
    structs::tree::{
        object::BinHeap, vectors::normalized_tree_vec::NormalizedTreeVector,
    },
};
use crate::core::advisors::empty_link_registry::registry::EmptyLinkStorage;

/// [`WorstFitAdvisor`] is a strategy that provides the worst fit
/// for a given size.
/// It uses [`Registry`] with [`BinHeap`] as a base structure.
/// So the getting the biggest length is O(1).
pub struct WorstFitAdvisor<'a> {
    /// Link to the [`Registry`]
    empty_link_registry:
        &'a mut Registry<NormalizedTreeVector<PageLink>, BinHeap<PageLink>>,
}

impl<'a> WorstFitAdvisor<'a> {
    /// Creates a new [`WorstFitAdvisor`]
    /// # Arguments
    /// * `empty_link_registry` - Link to the [`Registry`]
    /// # Returns
    /// * `WorstFitAdvisor` - New [`WorstFitAdvisor`]
    pub fn new(
        empty_link_registry: &'a mut Registry<
            NormalizedTreeVector<PageLink>,
            BinHeap<PageLink>,
        >,
    ) -> Self {
        WorstFitAdvisor {
            empty_link_registry,
        }
    }
}

impl<'a> PlaceAdvisorStrategy for WorstFitAdvisor<'a> {
    fn provide_place(&mut self, size: u16) -> Option<PageLink> {
        let data = self.empty_link_registry.get_data_mut();
        let base_obj = data.get_base_mut();

        let link = base_obj.peek_max();

        link.filter(|&link| link.len >= size)
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
        "WorstFit".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{
        advisors::{
            empty_link_registry::{
                factory::{
                    EmptyLinkRegistryFactory, WorstFitEmptyLinkRegistryFactory,
                },
                EmptyLinkRegistry,
            },
            strategies::{
                place_advisor_strategy::PlaceAdvisorStrategy,
                worst_fit_advisor::WorstFitAdvisor,
            },
        },
        link_struct::PageLink,
        structs::tree::object::tree::Tree,
    };
    use crate::core::advisors::empty_link_registry::registry::EmptyLinkStorage as _;

    #[test]
    fn test_worst_fit_advisor_new() {
        let registry =
            WorstFitEmptyLinkRegistryFactory::create_empty_link_registry();
        let EmptyLinkRegistry::WorstFit(mut registry) = registry else {
            panic!("Wrong type of registry");
        };

        let advisor = WorstFitAdvisor::new(&mut registry);

        assert_eq!(advisor.get_name(), "WorstFit".to_string());
    }

    #[test]
    fn test_worst_fit_advisor_provide_place() {
        let registry =
            WorstFitEmptyLinkRegistryFactory::create_empty_link_registry();
        let EmptyLinkRegistry::WorstFit(mut registry) = registry else {
            panic!("Wrong type of registry");
        };

        registry.add_link(PageLink::new(0, 0, 100));
        registry.add_link(PageLink::new(0, 100, 200));

        let mut advisor = WorstFitAdvisor::new(&mut registry);

        let link = advisor.provide_place(100);

        assert_eq!(link, Some(PageLink::new(0, 100, 200)));

        let link = advisor.provide_place(300);

        assert_eq!(link, None);
    }

    #[test]
    fn test_worst_fit_advisor_provide_place_with_empty_registry() {
        let registry =
            WorstFitEmptyLinkRegistryFactory::create_empty_link_registry();
        let EmptyLinkRegistry::WorstFit(mut registry) = registry else {
            panic!("Wrong type of registry");
        };

        let mut advisor = WorstFitAdvisor::new(&mut registry);

        let link = advisor.provide_place(100);

        assert_eq!(link, None);
    }

    #[test]
    fn test_worst_fit_advisor_apply_place() {
        let registry =
            WorstFitEmptyLinkRegistryFactory::create_empty_link_registry();
        let EmptyLinkRegistry::WorstFit(mut registry) = registry else {
            panic!("Wrong type of registry");
        };

        registry.add_link(PageLink::new(0, 0, 100));
        registry.add_link(PageLink::new(0, 100, 200));

        let mut advisor = WorstFitAdvisor::new(&mut registry);

        let link = advisor.provide_place(100);
        assert_eq!(link, Some(PageLink::new(0, 100, 200)));

        advisor.apply_place(&link.unwrap(), 100);

        let data = registry.get_data_mut();
        let link = data.remove_by_value(PageLink::new(0, 200, 100));
        assert_eq!(link, Some(PageLink::new(0, 200, 100)));
    }
}
