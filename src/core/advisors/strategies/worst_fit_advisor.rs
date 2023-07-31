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


#[cfg(test)]
mod tests {
    use crate::core::advisors::empty_link_registry::EmptyLinkRegistry;
    use crate::core::advisors::strategies::place_advisor_strategy::PlaceAdvisorStrategy;
    use crate::core::advisors::strategies::worst_fit_advisor::WorstFitAdvisor;
    use crate::core::link_struct::PageLink;
    use crate::core::structs::tree::object::balanced_tree::decoratable_balanced_tree::DecoratableBalancedTree;
    use crate::core::structs::tree::object::bin_heap::bin_heap::BinHeap;
    use crate::core::structs::tree::object::tree_object::TreeObject;
    use crate::core::structs::tree::vectors::normalized_tree_vec::NormalizedTreeVector;

    #[test]
    fn test_worst_fit_advisor_new() {
        let heap = BinHeap::<PageLink>::new_with_compare(PageLink::compare_by_len);
        let tree = DecoratableBalancedTree::<PageLink, NormalizedTreeVector<PageLink>, BinHeap<PageLink>>::new(heap, PageLink::compare_by_index);

        let mut registry = EmptyLinkRegistry::<NormalizedTreeVector<PageLink>, BinHeap<PageLink>>::new(tree);

        let advisor = WorstFitAdvisor::new(&mut registry);

        assert_eq!(advisor.get_name(), "WorstFit".to_string());
    }

    #[test]
    fn test_worst_fit_advisor_provide_place() {
        let heap = BinHeap::<PageLink>::new_with_compare(PageLink::compare_by_len);
        let tree = DecoratableBalancedTree::<PageLink, NormalizedTreeVector<PageLink>, BinHeap<PageLink>>::new(heap, PageLink::compare_by_index);

        let mut registry = EmptyLinkRegistry::<NormalizedTreeVector<PageLink>, BinHeap<PageLink>>::new(tree);

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
        let heap = BinHeap::<PageLink>::new_with_compare(PageLink::compare_by_len);
        let tree = DecoratableBalancedTree::<PageLink, NormalizedTreeVector<PageLink>, BinHeap<PageLink>>::new(heap, PageLink::compare_by_index);

        let mut registry = EmptyLinkRegistry::<NormalizedTreeVector<PageLink>, BinHeap<PageLink>>::new(tree);

        let mut advisor = WorstFitAdvisor::new(&mut registry);

        let link = advisor.provide_place(100);

        assert_eq!(link, None);
    }

    #[test]
    fn test_worst_fit_advisor_apply_place() {
        let heap = BinHeap::<PageLink>::new_with_compare(PageLink::compare_by_len);
        let tree = DecoratableBalancedTree::<PageLink, NormalizedTreeVector<PageLink>, BinHeap<PageLink>>::new(heap, PageLink::compare_by_index);

        let mut registry = EmptyLinkRegistry::<NormalizedTreeVector<PageLink>, BinHeap<PageLink>>::new(tree);

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
