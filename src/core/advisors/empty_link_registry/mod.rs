pub mod factory;
pub mod registry;

use crate::core::{
    advisors::empty_link_registry::registry::Registry,
    link_struct::PageLink,
    structs::tree::{
        object::{
            tree::{Tree, VecFunctions},
            BalancedTree, BinHeap,
        },
        vectors::{
            normalized_tree_vec::NormalizedTreeVector,
            optimized_tree_vec::OptimizedTreeVec,
            tree_vec::{Levels, TreeVec},
        },
    },
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
