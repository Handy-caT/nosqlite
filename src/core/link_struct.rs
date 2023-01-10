struct PageLink {
    page: u64,
    start: u32,
    len: u32,
}

impl PageLink {
    fn new(page: u64, start: u32, len: u32) -> PageLink {
        PageLink {
            page,
            start,
            len,
        }
    }
}

impl From<&[u8]> for PageLink {
    fn from(bytes: &[u8]) -> Self {
        let page = u64::from_be_bytes(bytes[0..8].try_into().unwrap());
        let start = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
        let len = u32::from_be_bytes(bytes[12..16].try_into().unwrap());
        PageLink {
            page,
            start,
            len,
        }
    }
}

impl Into<[u8]> for PageLink {
    fn into(self) -> [u8; 16] {
        let mut bytes = [0; 16];
        bytes[0..8].copy_from_slice(&self.page.to_be_bytes());
        bytes[8..12].copy_from_slice(&self.start.to_be_bytes());
        bytes[12..16].copy_from_slice(&self.len.to_be_bytes());
        bytes
    }
}

