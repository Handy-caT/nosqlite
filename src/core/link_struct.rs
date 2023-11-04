use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};

/// A struct that represents a link to a page.
pub struct PageLink {
    /// The index of the page.
    pub page_index: u64,
    /// The start index of the link on a page.
    pub start: u32,
    /// The length of the link.
    pub len: u32,
}

impl PageLink {
    /// Creates a new `PageLink` with the given parameters.
    /// # Arguments
    /// * `page` - The index of the page.
    /// * `start` - The start index of the link on a page.
    /// * `len` - The length of the link.
    /// # Returns
    /// A new `PageLink` with the given parameters.
    pub fn new(page: u64, start: u32, len: u32) -> PageLink {
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
        self.page_index * 4096 + self.start as u64
    }

    /// Returns the raw end of the link.
    /// Raw end is the index from the start of the file plus the length of the link.
    /// # Returns
    /// u64 - The raw end of the link.
    pub fn get_raw_end(&self) -> u64 {
        self.page_index * 4096 + self.start as u64 + self.len as u64 - 1
    }

    /// Compares two `PageLink`s by their length.
    /// # Arguments
    /// * `a` - The first `PageLink` to compare.
    /// * `b` - The second `PageLink` to compare.
    /// # Returns
    /// Ordering - The ordering of the two `PageLink`s.
    pub fn compare_by_len(a: &PageLink, b: &PageLink) -> Ordering {
        if a.len < b.len {
            Ordering::Less
        } else if a.len > b.len {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    /// Compares two `PageLink`s by their index.
    /// # Arguments
    /// * `a` - The first `PageLink` to compare.
    /// * `b` - The second `PageLink` to compare.
    /// # Returns
    /// Ordering - The ordering of the two `PageLink`s.
    pub fn compare_by_index(a: &PageLink, b: &PageLink) -> Ordering {
        if a.get_raw_index() < b.get_raw_index() {
            Ordering::Less
        } else if a.get_raw_index() > b.get_raw_index() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl From<[u8; 16]> for PageLink {
    fn from(bytes: [u8; 16]) -> Self {
        let page = u64::from_be_bytes(bytes[0..8].try_into().unwrap());
        let start = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
        let len = u32::from_be_bytes(bytes[12..16].try_into().unwrap());
        PageLink {
            page_index: page,
            start,
            len,
        }
    }
}

impl Into<[u8; 16]> for PageLink {
    fn into(self) -> [u8; 16] {
        let mut bytes = [0; 16];
        bytes[0..8].copy_from_slice(&self.page_index.to_be_bytes());
        bytes[8..12].copy_from_slice(&self.start.to_be_bytes());
        bytes[12..16].copy_from_slice(&self.len.to_be_bytes());
        bytes
    }
}

impl Clone for PageLink {
    fn clone(&self) -> Self {
        PageLink {
            page_index: self.page_index,
            start: self.start,
            len: self.len,
        }
    }
}

impl Copy for PageLink {}

impl Default for PageLink {
    fn default() -> Self {
        return PageLink::new(0, 0, 0);
    }
}

impl PartialEq for PageLink {
    fn eq(&self, other: &PageLink) -> bool {
        self.page_index == other.page_index && self.start == other.start
    }
}

impl Eq for PageLink {}

impl Debug for PageLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PageLink")
            .field("page_index", &self.page_index)
            .field("start", &self.start)
            .field("len", &self.len)
            .finish()
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
        let page: u64 = 2;
        let start: u32 = 10;
        let len: u32 = 20;

        let mut bytes = [0; 16];
        bytes[0..8].copy_from_slice(&page.to_be_bytes());
        bytes[8..12].copy_from_slice(&start.to_be_bytes());
        bytes[12..16].copy_from_slice(&len.to_be_bytes());

        let link = super::PageLink::from(bytes);
        assert_eq!(link.page_index, page);
        assert_eq!(link.start, start);
        assert_eq!(link.len, len);
    }

    #[test]
    fn test_page_link_into() {
        let page: u64 = 2;
        let start: u32 = 10;
        let len: u32 = 20;

        let link = super::PageLink::new(page, start, len);
        let bytes: [u8; 16] = link.into();

        assert_eq!(bytes[0..8], page.to_be_bytes());
        assert_eq!(bytes[8..12], start.to_be_bytes());
        assert_eq!(bytes[12..16], len.to_be_bytes());
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
            super::PageLink::compare_by_len(&link1, &link2),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            super::PageLink::compare_by_len(&link2, &link1),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            super::PageLink::compare_by_len(&link1, &link1),
            std::cmp::Ordering::Equal
        );
    }

    #[test]
    fn test_page_link_compare_by_index() {
        let link1 = super::PageLink::new(0, 0, 10);
        let link2 = super::PageLink::new(0, 10, 20);

        assert_eq!(
            super::PageLink::compare_by_index(&link1, &link2),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            super::PageLink::compare_by_index(&link2, &link1),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            super::PageLink::compare_by_index(&link1, &link1),
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
