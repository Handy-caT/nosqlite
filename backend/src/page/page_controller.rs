use crate::page::page_struct::Page;

#[derive(Debug)]
pub struct PageController {
    pages: Vec<Page>,
    page_count: usize,
}

impl Default for PageController {
    fn default() -> Self {
        let pages = vec![Page::new(0)];
        PageController {
            pages,
            page_count: 1,
        }
    }
}

impl PageController {
    pub fn get_page_count(&self) -> usize {
        self.page_count
    }

    pub fn add_page(&mut self) {
        let new_page = Page::new(self.page_count);

        self.pages.push(new_page);
        self.page_count += 1;
    }

    pub fn get_page(&mut self, index: usize) -> &mut Page {
        &mut self.pages[index]
    }

    pub fn get_last_page(&mut self) -> &mut Page {
        &mut self.pages[self.page_count - 1]
    }
}

#[cfg(test)]
mod test {
    use crate::page::page_controller::PageController;

    #[test]
    fn test_page_controller_new() {
        let controller = PageController::default();
        assert_eq!(controller.get_page_count(), 1);
        assert_eq!(controller.pages.len(), 1);
    }

    #[test]
    fn test_page_controller_add_page() {
        let mut controller = PageController::default();
        controller.add_page();
        assert_eq!(controller.get_page_count(), 2);
        assert_eq!(controller.pages.len(), 2);
    }

    #[test]
    fn test_page_controller_get_page() {
        let mut controller = PageController::default();
        controller.add_page();
        let page = controller.get_page(0);
        assert_eq!(page.get_index(), 0);
    }

    #[test]
    fn test_page_controller_get_last_page() {
        let mut controller = PageController::default();
        controller.add_page();
        let page = controller.get_last_page();
        assert_eq!(page.get_index(), 1);
    }
}
