use crate::core::{
    advisors::{
        empty_link_registry::BestFitEmptyLinkRegistry,
        strategies::PlaceAdvisorStrategyType,
    },
    link_struct::PageLink,
    page_controller::PageController,
};
use std::rc::Rc;

/// [`DataAllocator`] is a struct that is responsible for allocating and
/// deallocating memory.
pub struct DataAllocator {
    /// Strategy that is used to allocate memory.
    strategy: PlaceAdvisorStrategyType,

    /// [`EmptyLinkRegistry`] that is used to find empty places.
    empty_link_registry: BestFitEmptyLinkRegistry,

    /// Link to the [`PageController`] that is used to
    /// allocate and deallocate pages.
    page_controller: Rc<PageController>,

    /// Lin to the first free space in memory.
    tail_link: PageLink,
}

impl DataAllocator {
    pub fn new(_page_controller: Rc<PageController>) -> Self {
        // Self {
        //     strategy: PlaceAdvisorStrategyType::BestFit,
        //
        // }
        todo!()
    }
}
