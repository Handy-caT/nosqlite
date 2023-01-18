use crate::core::page_struct::Page;

struct PageController {
    pages: Vec<Page>,
    page_count: u64,
}

impl PageController {
    pub fn new() -> PageController {
        PageController {
            pages: Vec::new(),
            page_count: 0,
        }
    }

    pub fn get_page_count(&self) -> u64 {
        self.page_count
    }

    pub fn add_page(&mut self) {
        let new_page = Page::new(self.page_count);

        self.pages.push(new_page);
        self.page_count += 1;
    }

    pub fn get_page(&mut self, index: u64) -> &mut Page {
        &mut self.pages[index as usize]
    }

    pub fn get_last_page(&mut self) -> &mut Page {
        &mut self.pages[self.page_count as usize - 1]
    }

}


#[cfg(test)]
mod test {
    use crate::core::page_controller::PageController;

    #[test]
    fn test_page_controller_new() {
        let controller = PageController::new();
        assert_eq!(controller.get_page_count(), 0);
        assert_eq!(controller.pages.len(), 0);
    }

    #[test]
    fn test_page_controller_add_page() {
        let mut controller = PageController::new();
        controller.add_page();
        assert_eq!(controller.get_page_count(), 1);
        assert_eq!(controller.pages.len(), 1);
    }

    #[test]
    fn test_page_controller_get_page() {
        let mut controller = PageController::new();
        controller.add_page();
        let page = controller.get_page(0);
        assert_eq!(page.get_index(), 0);
    }

    #[test]
    fn test_page_controller_get_last_page() {
        let mut controller = PageController::new();
        controller.add_page();
        controller.add_page();
        let page = controller.get_last_page();
        assert_eq!(page.get_index(), 1);
    }
}