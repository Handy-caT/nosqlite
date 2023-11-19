use crate::core::{
    allocation_request::{AllocationRequest, AllocationStatus},
    link_struct::PageLink,
    page_controller::PageController,
    page_struct::Page,
};

pub struct DataAllocator {
    allocated: Vec<AllocationRequest>,
    available_erased: Vec<AllocationRequest>,
    page_controller: PageController,
}

impl DataAllocator {
    pub fn new() -> DataAllocator {
        DataAllocator {
            allocated: Vec::new(),
            available_erased: Vec::new(),
            page_controller: PageController::new(),
        }
    }

    pub fn get_allocated(&self) -> &Vec<AllocationRequest> {
        &self.allocated
    }

    pub fn get_available_erased(&self) -> &Vec<AllocationRequest> {
        &self.available_erased
    }

    fn allocate_as_free(page: &Page) -> AllocationRequest {
        let mut request = AllocationRequest::new();
        let free = page.get_free();
        let start = page.get_first_free();
        let link = PageLink::new(page.get_index(), start, free);

        request.add_link(link);
        request.set_status(AllocationStatus::Free);

        request
    }

    fn allocate(&mut self, len: u64) {
        let request = AllocationRequest::new();
        let mut last_page = self.page_controller.get_last_page();

        if (last_page.get_free() as u64) < len {
            let free = Self::allocate_as_free(last_page);
            self.allocated.push(free);

            self.page_controller.add_page();
            last_page = self.page_controller.get_last_page();
        }
    }
}
