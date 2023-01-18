use crate::core::link_struct::PageLink;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum AllocationStatus {
    Allocated,
    Free,
    Used,
}


pub struct AllocationRequest {
    links: Vec<PageLink>,
    status: AllocationStatus,
}

impl AllocationRequest {
    pub fn new() -> AllocationRequest {
        AllocationRequest {
            links: Vec::new(),
            status: AllocationStatus::Free,
        }
    }

    pub fn get_links(&self) -> &Vec<PageLink> {
        &self.links
    }

    pub fn get_status(&self) -> &AllocationStatus {
        &self.status
    }

    fn set_status(&mut self, status: AllocationStatus) {
        self.status = status;
    }

    fn add_link(&mut self, link: PageLink) {
        self.links.push(link);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let request = AllocationRequest::new();
        assert_eq!(request.get_links().len(), 0);
        assert_eq!(request.get_status(), &AllocationStatus::Free);
    }

    #[test]
    fn test_set_status() {
        let mut request = AllocationRequest::new();
        request.set_status(AllocationStatus::Allocated);
        assert_eq!(request.get_status(), &AllocationStatus::Allocated);
    }

    #[test]
    fn test_add_link() {
        let mut request = AllocationRequest::new();
        let link = PageLink::new(0, 0, 16);
        request.add_link(link);
        assert_eq!(request.get_links().len(), 1);
    }
}