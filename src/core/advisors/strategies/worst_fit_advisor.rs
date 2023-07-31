use crate::core::advisors::empty_link_registry::EmptyLinkRegistry;
use crate::core::advisors::strategies::place_advisor_strategy::PlaceAdvisorStrategy;
use crate::core::link_struct::PageLink;
use crate::core::structs::tree::object::bin_heap::bin_heap::BinHeap;
use crate::core::structs::tree::vectors::normalized_tree_vec::NormalizedTreeVector;

pub struct WorstFitAdvisor<'a>
{
    empty_link_registry: &'a mut EmptyLinkRegistry<NormalizedTreeVector<PageLink>, BinHeap<PageLink>>
}

impl <'a> WorstFitAdvisor<'a>
{
    pub fn new(empty_link_registry: &'a mut EmptyLinkRegistry<NormalizedTreeVector<PageLink>, BinHeap<PageLink>>) -> Self {
        WorstFitAdvisor {
            empty_link_registry
        }
    }
}

impl <'a> PlaceAdvisorStrategy for WorstFitAdvisor<'a>
{
    fn provide_place(&mut self, size: u64) -> Option<PageLink> {
        let data = self.empty_link_registry.get_data_mut();
        let base_obj = data.get_base_mut();

        let link = base_obj.peek_max();

        match link {
            Some(link) => {
                if link.len >= size as u32 {
                    Some(link.clone())
                } else {
                    None
                }
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
        "WorstFit".to_string()
    }
}
