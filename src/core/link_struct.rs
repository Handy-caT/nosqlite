use crate::core::base::cast::usize::USIZE_SIZE;
use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
};
use crate::core::page_struct::PAGE_SIZE;

/// A struct that represents a link to a page.
#[derive(Debug, Default, Copy, Clone, Eq)]
pub struct PageLink {
    /// The index of the page.
    pub page_index: usize,

    /// The start index of the link on a page.
    pub start: u16,

    /// The length of the link.
    pub len: u16,
}

impl PageLink {
    /// Creates a new [`PageLink`] with the given parameters.
    /// # Arguments
    /// * `page` - The index of the page.
    /// * `start` - The start index of the link on a page.
    /// * `len` - The length of the link.
    /// # Returns
    /// A new [`PageLink`] with the given parameters.
    pub fn new(page: usize, start: u16, len: u16) -> Self {
        PageLink {
            page_index: page,
            start,
            len,
        }
    }

    /// Creates a new [`PageLink`] from the given raw index and length.
    /// # Arguments
    /// * `index` - The raw index of the link.
    /// * `len` - The length of the link.
    /// # Returns
    /// A new [`PageLink`] from the given raw index and length.
    pub fn new_from_raw(index: u64, len: u16) -> Self {
        let page_index = (index / u64::from(PAGE_SIZE)) as usize;
        let start = (index % u64::from(PAGE_SIZE)) as u16;

        PageLink {
            page_index,
            start,
            len,
        }
    }

    /// Returns the raw index of the link.
    /// Raw index is the index from the start of the file.
    /// # Returns
    /// u64 - The raw index of the link.
    pub fn get_raw_index(&self) -> u64 {
        u64::try_from(self.page_index).unwrap() * u64::from(PAGE_SIZE) + u64::from(self.start)
    }

    /// Returns the raw end of the link.
    /// Raw end is the index from the start of the file plus
    /// the length of the link.
    /// # Returns
    /// u64 - The raw end of the link.
    pub fn get_raw_end(&self) -> u64 {
        u64::try_from(self.page_index).unwrap() * u64::from(PAGE_SIZE)
            + u64::from(self.start)
            + u64::from(self.len)
    }

    /// Compares two `PageLink`s by their length.
    /// # Arguments
    /// * `a` - The first `PageLink` to compare.
    /// * `b` - The second `PageLink` to compare.
    /// # Returns
    /// Ordering - The ordering of the two `PageLink`s.
    pub fn compare_by_len(a: PageLink, b: PageLink) -> Ordering {
        a.len.cmp(&b.len)
    }

    /// Compares two `PageLink`s by their index.
    /// # Arguments
    /// * `a` - The first `PageLink` to compare.
    /// * `b` - The second `PageLink` to compare.
    /// # Returns
    /// Ordering - The ordering of the two `PageLink`s.
    pub fn compare_by_index(a: PageLink, b: PageLink) -> Ordering {
        a.get_raw_index().cmp(&b.get_raw_index())
    }

    /// Returns the length of the link till the end of the page.
    /// # Returns
    /// u16 - The length of the link till the end of the page.
    pub fn get_len_till_end(&self) -> u16 {
        PAGE_SIZE - self.start
    }
}

impl From<[u8; 4 + USIZE_SIZE]> for PageLink {
    fn from(bytes: [u8; 4 + USIZE_SIZE]) -> Self {
        let page =
            usize::from_be_bytes(bytes[0..USIZE_SIZE].try_into().unwrap());
        let start = u16::from_be_bytes(
            bytes[USIZE_SIZE..USIZE_SIZE + 2].try_into().unwrap(),
        );
        let len = u16::from_be_bytes(
            bytes[USIZE_SIZE + 2..USIZE_SIZE + 4].try_into().unwrap(),
        );
        PageLink {
            page_index: page,
            start,
            len,
        }
    }
}

impl From<PageLink> for [u8; 4 + USIZE_SIZE] {
    fn from(val: PageLink) -> Self {
        let mut bytes = [0; 4 + USIZE_SIZE];
        bytes[0..USIZE_SIZE].copy_from_slice(&val.page_index.to_be_bytes());
        bytes[USIZE_SIZE..USIZE_SIZE + 2]
            .copy_from_slice(&val.start.to_be_bytes());
        bytes[USIZE_SIZE + 2..USIZE_SIZE + 4]
            .copy_from_slice(&val.len.to_be_bytes());
        bytes
    }
}

impl PartialEq for PageLink {
    fn eq(&self, other: &PageLink) -> bool {
        self.page_index == other.page_index && self.start == other.start
    }
}

impl Ord for PageLink {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_raw_index().cmp(&other.get_raw_index())
    }
}

impl PartialOrd for PageLink {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for PageLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PageLink {{ page_index: {}, start: {}, len: {} }}",
            self.page_index, self.start, self.len
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::core::base::cast::usize::USIZE_SIZE;

    #[test]
    fn test_page_link_new() {
        let link = super::PageLink::new(0, 0, 10);
        assert_eq!(link.page_index, 0);
        assert_eq!(link.start, 0);
        assert_eq!(link.len, 10);
        assert_eq!(link.get_raw_index(), 0);
        assert_eq!(link.get_raw_end(), 10);
    }

    #[test]
    fn test_page_link_from() {
        let page: usize = 2;
        let start: u16 = 10;
        let len: u16 = 20;

        let mut bytes = [0; USIZE_SIZE + 4];
        bytes[0..USIZE_SIZE].copy_from_slice(&page.to_be_bytes());
        bytes[USIZE_SIZE..USIZE_SIZE + 2].copy_from_slice(&start.to_be_bytes());
        bytes[USIZE_SIZE + 2..USIZE_SIZE + 4]
            .copy_from_slice(&len.to_be_bytes());

        let link = super::PageLink::from(bytes);
        assert_eq!(link.page_index, page);
        assert_eq!(link.start, start);
        assert_eq!(link.len, len);
    }

    #[test]
    fn test_page_link_into() {
        let page: usize = 2;
        let start: u16 = 10;
        let len: u16 = 20;

        let link = super::PageLink::new(page, start, len);
        let bytes: [u8; USIZE_SIZE + 4] = link.into();

        assert_eq!(bytes[0..USIZE_SIZE], page.to_be_bytes());
        assert_eq!(bytes[USIZE_SIZE..USIZE_SIZE + 2], start.to_be_bytes());
        assert_eq!(bytes[USIZE_SIZE + 2..USIZE_SIZE + 4], len.to_be_bytes());
    }

    #[test]
    fn test_page_link_display() {
        let link = super::PageLink::new(0, 0, 10);
        assert_eq!(
            format!("{}", link),
            "PageLink { page_index: 0, start: 0, len: 10 }"
        );
    }

    #[test]
    fn test_page_link_compare_by_len() {
        let link1 = super::PageLink::new(0, 0, 10);
        let link2 = super::PageLink::new(0, 0, 20);

        assert_eq!(
            super::PageLink::compare_by_len(link1, link2),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            super::PageLink::compare_by_len(link2, link1),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            super::PageLink::compare_by_len(link1, link1),
            std::cmp::Ordering::Equal
        );
    }

    #[test]
    fn test_page_link_compare_by_index() {
        let link1 = super::PageLink::new(0, 0, 10);
        let link2 = super::PageLink::new(0, 10, 20);

        assert_eq!(
            super::PageLink::compare_by_index(link1, link2),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            super::PageLink::compare_by_index(link2, link1),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            super::PageLink::compare_by_index(link1, link1),
            std::cmp::Ordering::Equal
        );
    }

    #[test]
    fn test_page_link_eq() {
        let link1 = super::PageLink::new(0, 0, 10);
        let link2 = super::PageLink::new(0, 0, 20);
        let link3 = super::PageLink::new(0, 10, 20);

        assert!(link1.eq(&link1));
        assert!(link1.eq(&link2));

        assert!(!link1.eq(&link3));
        assert!(!link2.eq(&link3));
    }
}
