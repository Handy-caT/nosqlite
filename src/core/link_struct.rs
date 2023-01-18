pub struct PageLink {
    page_index: u64,
    start: u32,
    len: u32,
}

impl PageLink {
    pub fn new(page: u64, start: u32, len: u32) -> PageLink {
        PageLink {
            page_index: page,
            start,
            len,
        }
    }

    pub fn get_page_index(&self) -> u64 {
        self.page_index
    }

    pub fn get_start(&self) -> u32 {
        self.start
    }

    pub fn get_len(&self) -> u32 {
        self.len
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


#[cfg(test)]
mod tests {
    #[test]
    fn test_page_link_new() {
        let link = super::PageLink::new(0, 0, 10);
        assert_eq!(link.page_index, 0);
        assert_eq!(link.start, 0);
        assert_eq!(link.len, 10);
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
}