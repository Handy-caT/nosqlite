pub mod factory;
pub mod registry;

use crate::{
    core::structs::tree::{
        object::{BalancedTree, BinHeap},
        vectors::{
            normalized_tree_vec::NormalizedTreeVector,
            optimized_tree_vec::OptimizedTreeVec,
        },
    },
    data::advisors::empty_link_registry::registry::Registry,
    page::link_struct::PageLink,
};

pub type BestFitEmptyLinkRegistry = Registry<
    OptimizedTreeVec<PageLink>,
    BalancedTree<PageLink, OptimizedTreeVec<PageLink>>,
>;

pub type WorstFitEmptyLinkRegistry =
    Registry<NormalizedTreeVector<PageLink>, BinHeap<PageLink>>;

/// A registry for empty links.
/// Is used with [`PlaceAdvisorStrategy`].
pub enum EmptyLinkRegistry {
    /// A registry for empty links that uses the best fit strategy.
    /// Is used with [`BestFitAdvisor`].
    BestFit(BestFitEmptyLinkRegistry),

    /// A registry for empty links that uses the worst fit strategy.
    /// Is used with [`WorstFitAdvisor`].
    WorstFit(WorstFitEmptyLinkRegistry),
}

impl EmptyLinkRegistry {
    /// Gets the name of the registry.
    /// # Returns
    /// * `String` - Name of the registry.
    pub fn get_name(&self) -> String {
        match self {
            EmptyLinkRegistry::BestFit(_) => "BestFit".to_string(),
            EmptyLinkRegistry::WorstFit(_) => "WorstFit".to_string(),
        }
    }

    /// Gets the length of the registry.
    /// # Returns
    /// * `usize` - Length of the registry.
    pub fn len(&self) -> usize {
        match self {
            EmptyLinkRegistry::BestFit(registry) => registry.len(),
            EmptyLinkRegistry::WorstFit(registry) => registry.len(),
        }
    }
}
