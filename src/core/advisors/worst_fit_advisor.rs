use crate::core::advisors::neighbor_checker::NeighborChecker;
use crate::core::advisors::place_advisor::PlaceAdvisor;
use crate::core::link_struct::PageLink;
use crate::core::structs::bin_heap::BinHeap;


struct WorstFitAdvisor {
    data: BinHeap<PageLink>,
    neighbour_checker: NeighborChecker
}

impl WorstFitAdvisor {
    pub fn new() -> WorstFitAdvisor {
        WorstFitAdvisor {
            data: BinHeap::new_with_compare(PageLink::compare_by_len),
            neighbour_checker: NeighborChecker::new()
        }
    }
}

impl PlaceAdvisor for WorstFitAdvisor {
    fn add_free(&mut self, link: PageLink) {

    }

    fn provide_place(&mut self, size: u64) -> Option<PageLink> {
        let mut res = None;
        let mut link = self.data.peek_max();

        if link.is_some() {
            let link = link.unwrap();
            if link.get_len() as u64 >= size {
                res = Some(link);
            }
        }

        res
    }

    fn apply_place(&mut self, link: PageLink, len: u64) {
        todo!()
    }
}