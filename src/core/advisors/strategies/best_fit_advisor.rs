use crate::core::advisors::empty_link_registry::EmptyLinkRegistry;
use crate::core::advisors::strategies::place_advisor_strategy::PlaceAdvisorStrategy;
use crate::core::link_struct::PageLink;
use crate::core::structs::tree::object::balanced_tree::balanced_tree::BalancedTree;
use crate::core::structs::tree::object::tree_object::TreeObjectFind;
use crate::core::structs::tree::vectors::tree_vec::{TreeVec, TreeVecIndexes, TreeVecLevels};

/// BestFitAdvisor is a strategy that finds the best fit for a given size.
/// It uses EmptyLinkRegistry with BalancedTree as a data structure. So the getting the best fit is O(log n).
pub struct BestFitAdvisor<'a, V>
where
    V: TreeVec<PageLink> + Sized + TreeVecIndexes<PageLink> + TreeVecLevels

{
    /// Link to the EmptyLinkRegistry
    empty_link_registry: &'a mut EmptyLinkRegistry<V, BalancedTree<PageLink, V>>
}

impl <'a, V> BestFitAdvisor<'a, V>
where
    V: TreeVec<PageLink> + Sized + TreeVecIndexes<PageLink> + TreeVecLevels
{
    /// Creates a new BestFitAdvisor
    /// # Arguments
    /// * `empty_link_registry` - Link to the EmptyLinkRegistry
    /// # Returns
    /// * `BestFitAdvisor` - New BestFitAdvisor
    pub fn new(empty_link_registry: &'a mut EmptyLinkRegistry<V, BalancedTree<PageLink, V>>) -> Self {
        BestFitAdvisor {
            empty_link_registry
        }
    }

    pub fn get_empty_link_registry(&self) -> &EmptyLinkRegistry<V, BalancedTree<PageLink, V>> {
        &self.empty_link_registry
    }

    pub fn get_empty_link_registry_mut(&mut self) -> &mut EmptyLinkRegistry<V, BalancedTree<PageLink, V>> {
        &mut self.empty_link_registry
    }
}

impl <'a, V> PlaceAdvisorStrategy for BestFitAdvisor<'a, V>
    where
        V: TreeVec<PageLink> + Sized + TreeVecIndexes<PageLink> + TreeVecLevels
{
    fn provide_place(&mut self, size: u64) -> Option<PageLink> {
        let data = self.empty_link_registry.get_data_mut();
        let base_obj = data.get_base_mut();

        let link = base_obj.find_greater_equal(PageLink::new(0, 0, size as u32));

        match link {
            Some(link) => {
                let link = link.1;
                Some(link)
            },
            None => None
        }
    }

    fn apply_place(&mut self, link: &PageLink, size: u64) {
        if link.len > size as u32 {
            let new_link = PageLink::new(link.page_index ,link.start + size as u32, link.len - size as u32);
            self.empty_link_registry.add_link(new_link);
        }

        self.empty_link_registry.remove_link(link.clone());
    }

    fn get_name(&self) -> String {
        "BestFitAdvisor".to_string()
    }
}
