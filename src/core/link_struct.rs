use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
};

/// A struct that represents a link to a page.
#[derive(Debug, Default, Copy, Clone, Eq)]
pub struct PageLink {
    /// The index of the page.
    pub page_index: u32,
    /// The start index of the link on a page.
    pub start: u16,
    /// The length of the link.
    pub len: u16,
}

impl PageLink {
    /// Creates a new `PageLink` with the given parameters.
    /// # Arguments
    /// * `page` - The index of the page.
    /// * `start` - The start index of the link on a page.
    /// * `len` - The length of the link.
    /// # Returns
    /// A new `PageLink` with the given parameters.
    pub fn new(page: u32, start: u16, len: u16) -> PageLink {
        PageLink {
            page_index: page,
            start,
            len,
        }
    }

    /// Returns the raw index of the link.
    /// Raw index is the index from the start of the file.
    /// # Returns
    /// u64 - The raw index of the link.
    pub fn get_raw_index(&self) -> u64 {
        u64::from(self.page_index * 4096 + u32::from(self.start))
    }

    /// Returns the raw end of the link.
    /// Raw end is the index from the start of the file plus
    /// the length of the link.
    /// # Returns
    /// u64 - The raw end of the link.
    pub fn get_raw_end(&self) -> u64 {
        u64::from(self.page_index * 4096 + u32::from(self.start) + u32::from(self.len) - 1)
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
}

impl From<[u8; 8]> for PageLink {
    fn from(bytes: [u8; 8]) -> Self {
        let page = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
        let start = u16::from_be_bytes(bytes[4..6].try_into().unwrap());
        let len = u16::from_be_bytes(bytes[6..8].try_into().unwrap());
        PageLink {
            page_index: page,
            start,
            len,
        }
    }
}

impl Into<[u8; 8]> for PageLink {
    fn into(self) -> [u8; 8] {
        let mut bytes = [0; 8];
        bytes[0..4].copy_from_slice(&self.page_index.to_be_bytes());
        bytes[4..6].copy_from_slice(&self.start.to_be_bytes());
        bytes[6..8].copy_from_slice(&self.len.to_be_bytes());
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
    #[test]
    fn test_page_link_new() {
        let link = super::PageLink::new(0, 0, 10);
        assert_eq!(link.page_index, 0);
        assert_eq!(link.start, 0);
        assert_eq!(link.len, 10);
        assert_eq!(link.get_raw_index(), 0);
        assert_eq!(link.get_raw_end(), 9);
    }

    #[test]
    fn test_page_link_from() {
        let page: u32 = 2;
        let start: u16 = 10;
        let len: u16 = 20;

        let mut bytes = [0; 8];
        bytes[0..4].copy_from_slice(&page.to_be_bytes());
        bytes[4..6].copy_from_slice(&start.to_be_bytes());
        bytes[6..8].copy_from_slice(&len.to_be_bytes());

        let link = super::PageLink::from(bytes);
        assert_eq!(link.page_index, page);
        assert_eq!(link.start, start);
        assert_eq!(link.len, len);
    }

    #[test]
    fn test_page_link_into() {
        let page: u32 = 2;
        let start: u16 = 10;
        let len: u16 = 20;

        let link = super::PageLink::new(page, start, len);
        let bytes: [u8; 8] = link.into();

        assert_eq!(bytes[0..4], page.to_be_bytes());
        assert_eq!(bytes[4..6], start.to_be_bytes());
        assert_eq!(bytes[6..8], len.to_be_bytes());
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

        assert_eq!(link1.eq(&link1), true);
        assert_eq!(link1.eq(&link2), true);

        assert_eq!(link1.eq(&link3), false);
        assert_eq!(link2.eq(&link3), false);
    }
}
