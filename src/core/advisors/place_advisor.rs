use crate::core::link_struct::PageLink;

pub trait PlaceAdvisor {
    fn add_free(&mut self, link: PageLink);
    fn provide_place(&mut self, size: u64) -> Option<PageLink>;
    fn apply_place(&mut self, link: PageLink, len: u64);
}