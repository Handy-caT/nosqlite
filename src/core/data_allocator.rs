use crate::core::{
    advisors::{
        empty_link_registry::{
            factory::{
                BestFitEmptyLinkRegistryFactory, EmptyLinkRegistryFactory,
            },
            registry::EmptyLinkStorage,
            EmptyLinkRegistry,
        },
        strategies::{
            best_fit_advisor::BestFitAdvisor,
            place_advisor_strategy::PlaceAdvisorStrategy,
            worst_fit_advisor::WorstFitAdvisor,
        },
    },
    link_struct::PageLink,
    page_controller::PageController,
    page_struct::PAGE_SIZE,
};
use std::rc::Rc;

/// [`DataAllocator`] is a struct that is responsible for allocating and
/// deallocating memory.
pub struct DataAllocator {
    /// [`EmptyLinkRegistry`] that is used to find empty places.
    empty_link_registry: EmptyLinkRegistry,

    /// Link to the [`PageController`] that is used to
    /// allocate and deallocate pages.
    page_controller: Rc<PageController>,

    /// Lin to the first free space in memory.
    tail_link: PageLink,
}

impl DataAllocator {
    /// Creates a new [`DataAllocator`].
    /// # Arguments
    /// * `page_controller` - Link to the [`PageController`]
    /// # Returns
    /// * `DataAllocator` - New [`DataAllocator`]
    pub fn new(page_controller: Rc<PageController>) -> Self {
        Self {
            empty_link_registry:
                BestFitEmptyLinkRegistryFactory::create_empty_link_registry(),
            page_controller,
            tail_link: PageLink::new(0, 0, PAGE_SIZE),
        }
    }

    /// Gets [`PlaceAdvisorStrategy`] that is used to provide
    /// place for new data.
    /// # Returns
    /// * `&mut dyn PlaceAdvisorStrategy` - [`PlaceAdvisorStrategy`] that is
    /// used to provide place for new data.
    fn get_place_advisor(&mut self) -> &mut dyn PlaceAdvisorStrategy {
        match self.empty_link_registry {
            EmptyLinkRegistry::BestFit(ref mut registry) => {
                let advisor = BestFitAdvisor::new(registry);
                Box::leak(Box::new(advisor))
            }
            EmptyLinkRegistry::WorstFit(ref mut registry) => {
                let advisor = WorstFitAdvisor::new(registry);
                Box::leak(Box::new(advisor))
            }
        }
    }

    /// Gets [`EmptyLinkStorage`] that is used to store empty links.
    /// # Returns
    /// * `&mut dyn EmptyLinkStorage` - [`EmptyLinkStorage`] that is
    /// used to store empty links.
    fn get_empty_link_registry(&mut self) -> &mut dyn EmptyLinkStorage {
        match self.empty_link_registry {
            EmptyLinkRegistry::BestFit(ref mut registry) => registry,
            EmptyLinkRegistry::WorstFit(ref mut registry) => registry,
        }
    }

    /// Allocates memory for data by given size.
    /// # Arguments
    /// * `size` - Size of data that should be allocated.
    /// # Returns
    /// * `Option<PageLink>` - Link to the allocated memory.
    pub fn allocate(&mut self, size: u16) -> Option<PageLink> {
        let advisor = self.get_place_advisor();

        let link = advisor.provide_place(size);

        if let Some(link) = link {
            advisor.apply_place(&link, size);
            Some(link)
        } else {
            let link =
                PageLink::new_from_raw(self.tail_link.get_raw_index(), size);

            self.tail_link = PageLink::new_from_raw(link.get_raw_end(), 0);
            self.tail_link.len = self.tail_link.get_len_till_end();

            Some(link)
        }
    }

    /// Mark link as empty.
    /// # Arguments
    /// * `link` - Link to the memory that should be marked as empty.
    pub fn remove(&mut self, link: PageLink) {
        self.get_empty_link_registry().add_link(link);
    }

    /// Gets the size of allocated memory.
    /// # Returns
    /// * `u64` - Size of allocated memory.
    pub fn allocated_size(&self) -> u64 {
        self.tail_link.get_raw_index()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{
        data_allocator::DataAllocator, link_struct::PageLink,
        page_controller::PageController, page_struct::PAGE_SIZE,
    };
    use std::rc::Rc;

    #[test]
    fn test_data_allocator_new() {
        let page_controller = Rc::new(PageController::new());
        let data_allocator = DataAllocator::new(page_controller);

        assert_eq!(data_allocator.empty_link_registry.get_name(), "BestFit");
        assert_eq!(data_allocator.tail_link, PageLink::new(0, 0, 0));
        assert_eq!(data_allocator.allocated_size(), 0);
    }

    #[test]
    fn test_data_allocator_allocate() {
        let page_controller = Rc::new(PageController::new());
        let mut data_allocator = DataAllocator::new(page_controller);

        let link = data_allocator.allocate(10);

        assert_eq!(link, Some(PageLink::new(0, 0, 10)));
        assert_eq!(
            data_allocator.tail_link,
            PageLink::new(0, 10, PAGE_SIZE - 10)
        );
        assert_eq!(data_allocator.allocated_size(), 10);
    }

    #[test]
    fn test_data_allocator_allocate_end_of_page() {
        let page_controller = Rc::new(PageController::new());
        let mut data_allocator = DataAllocator::new(page_controller);

        let link = data_allocator.allocate(PAGE_SIZE - 10);

        assert_eq!(link, Some(PageLink::new(0, 0, PAGE_SIZE - 10)));
        assert_eq!(
            data_allocator.tail_link,
            PageLink::new(0, PAGE_SIZE - 10, 0)
        );

        let link = data_allocator.allocate(20);

        assert_eq!(link, Some(PageLink::new(0, PAGE_SIZE - 10, 20)));
        assert_eq!(
            data_allocator.tail_link,
            PageLink::new(1, 10, PAGE_SIZE - 10)
        );
    }

    #[test]
    fn test_data_allocator_remove() {
        let page_controller = Rc::new(PageController::new());
        let mut data_allocator = DataAllocator::new(page_controller);

        let link = data_allocator.allocate(10);

        assert_eq!(link, Some(PageLink::new(0, 0, 10)));
        assert_eq!(
            data_allocator.tail_link,
            PageLink::new(0, 10, PAGE_SIZE - 10)
        );

        data_allocator.remove(PageLink::new(0, 0, 10));

        assert_eq!(
            data_allocator.tail_link,
            PageLink::new(0, 10, PAGE_SIZE - 10)
        );
        assert_eq!(data_allocator.empty_link_registry.len(), 1);
        assert_eq!(data_allocator.allocated_size(), 10);
    }

    #[test]
    fn test_data_allocator_add_after_remove() {
        let page_controller = Rc::new(PageController::new());
        let mut data_allocator = DataAllocator::new(page_controller);

        let link = data_allocator.allocate(10);

        assert_eq!(link, Some(PageLink::new(0, 0, 10)));
        assert_eq!(
            data_allocator.tail_link,
            PageLink::new(0, 10, PAGE_SIZE - 10)
        );

        data_allocator.remove(PageLink::new(0, 0, 10));

        assert_eq!(
            data_allocator.tail_link,
            PageLink::new(0, 10, PAGE_SIZE - 10)
        );
        assert_eq!(data_allocator.empty_link_registry.len(), 1);

        let link = data_allocator.allocate(10);

        assert_eq!(link, Some(PageLink::new(0, 0, 10)));
        assert_eq!(
            data_allocator.tail_link,
            PageLink::new(0, 10, PAGE_SIZE - 10)
        );
        assert_eq!(data_allocator.empty_link_registry.len(), 0);
    }
}
