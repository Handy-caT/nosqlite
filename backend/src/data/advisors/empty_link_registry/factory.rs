use common::structs::tree::{
    object::{
        balanced_tree::Decoratable, tree::Tree as _, BalancedTree, BinHeap,
    },
    vectors::{
        normalized_tree_vec::NormalizedTreeVector,
        optimized_tree_vec::OptimizedTreeVec,
    },
};

use crate::{
    data::advisors::empty_link_registry::{
        registry::Registry, BestFitEmptyLinkRegistry, EmptyLinkRegistry,
    },
    page::link_struct::PageLink,
};

/// [`EmptyLinkRegistryFactory`] is a trait that is used to create
/// [`EmptyLinkRegistry`] instances.
pub trait EmptyLinkRegistryFactory {
    /// Creates a new [`EmptyLinkRegistry`] instance.
    fn create_empty_link_registry() -> EmptyLinkRegistry;
}

/// A factory for [`EmptyLinkRegistry`] instances that use the best fit
/// strategy.
pub struct BestFitEmptyLinkRegistryFactory;

impl EmptyLinkRegistryFactory for BestFitEmptyLinkRegistryFactory {
    fn create_empty_link_registry() -> EmptyLinkRegistry {
        let tree =
            BalancedTree::<PageLink, OptimizedTreeVec<PageLink>>
            ::new_with_compare(
                PageLink::compare_by_len,
            );

        let dec_tree = Decoratable::<PageLink, _, _>::new_with_existing(
            tree,
            PageLink::compare_by_index,
        );

        let registry = BestFitEmptyLinkRegistry::new(dec_tree);

        EmptyLinkRegistry::BestFit(registry)
    }
}

/// A factory for [`EmptyLinkRegistry`] instances that use the worst fit
/// strategy.
pub struct WorstFitEmptyLinkRegistryFactory;

impl EmptyLinkRegistryFactory for WorstFitEmptyLinkRegistryFactory {
    fn create_empty_link_registry() -> EmptyLinkRegistry {
        let tree = Decoratable::<
            PageLink,
            NormalizedTreeVector<PageLink>,
            BinHeap<PageLink>,
        >::new_with_compare(PageLink::compare_by_index);

        let registry = Registry::<_, _>::new(tree);

        EmptyLinkRegistry::WorstFit(registry)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::advisors::empty_link_registry::{
        factory::{
            BestFitEmptyLinkRegistryFactory, EmptyLinkRegistryFactory,
            WorstFitEmptyLinkRegistryFactory,
        },
        EmptyLinkRegistry,
    };

    #[test]
    fn test_best_fit_empty_link_registry_factory_create_empty_link_registry() {
        let registry =
            BestFitEmptyLinkRegistryFactory::create_empty_link_registry();
        let EmptyLinkRegistry::BestFit(_registry) = registry else {
            panic!("Wrong type of registry");
        };
    }

    #[test]
    fn test_worst_fit_empty_link_registry_factory_create_empty_link_registry() {
        let registry =
            WorstFitEmptyLinkRegistryFactory::create_empty_link_registry();
        let EmptyLinkRegistry::WorstFit(_registry) = registry else {
            panic!("Wrong type of registry");
        };
    }
}
