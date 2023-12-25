use crate::core::{
    base::cast::{usize, usize::USIZE_SIZE},
};
use crate::page::link_struct::PageLink;

pub const PAGE_SIZE: u16 = 4096;

#[derive(Clone, Copy, Debug)]
pub struct PageInfo {
    index: usize,
    free: u16,
}

impl PageInfo {
    pub fn new(index: usize) -> PageInfo {
        PageInfo {
            index,
            free: PAGE_SIZE,
        }
    }

    pub fn get_index(self) -> usize {
        self.index
    }

    pub fn get_free(self) -> u16 {
        self.free
    }
}

impl From<[u8; 2 + USIZE_SIZE]> for PageInfo {
    fn from(bytes: [u8; 2 + USIZE_SIZE]) -> Self {
        let index =
            usize::from_be_bytes(bytes[0..USIZE_SIZE].try_into().unwrap());
        let free = u16::from_be_bytes(
            bytes[USIZE_SIZE..USIZE_SIZE + 2].try_into().unwrap(),
        );
        PageInfo { index, free }
    }
}

impl From<PageInfo> for [u8; 2 + USIZE_SIZE] {
    fn from(val: PageInfo) -> Self {
        let mut bytes = [0; 2 + USIZE_SIZE];
        bytes[0..USIZE_SIZE].copy_from_slice(&val.index.to_be_bytes());
        bytes[USIZE_SIZE..USIZE_SIZE + 2]
            .copy_from_slice(&val.free.to_be_bytes());
        bytes
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Page {
    info: PageInfo,
    data: [u8; PAGE_SIZE as usize],
}

impl Page {
    pub fn new(index: usize) -> Page {
        Page {
            info: PageInfo::new(index),
            data: [0; 4096],
        }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data[0..(PAGE_SIZE as usize - self.info.free as usize)]
    }

    pub fn get_free(&self) -> u16 {
        self.info.free
    }

    pub fn get_index(&self) -> usize {
        self.info.index
    }

    pub fn get_first_free(&self) -> u16 {
        PAGE_SIZE - self.info.free
    }

    pub fn can_fit(&self, len: u16) -> bool {
        self.info.free >= len
    }

    pub fn attach_data(&mut self, info: &[u8]) {
        let mut i = 0;

        while i < info.len() {
            self.data[PAGE_SIZE as usize - self.info.free as usize] = info[i];
            self.info.free -= 1;
            i += 1;
        }
    }

    pub fn update_data(
        &mut self,
        data: &[u8],
        link: PageLink,
    ) -> Result<PageLink, String> {
        if link.len as usize != data.len() {
            return Err("Data length does not match link length".to_string());
        }
        let mut i = 0;

        while i < data.len() {
            self.data[link.start as usize + i] = data[i];
            i += 1;
        }
        let res_link = PageLink::new(self.info.index, link.start, link.len);
        Ok(res_link)
    }

    pub fn erase_data(&mut self, link: PageLink) {
        let mut i: usize = 0;

        while i < link.len as usize {
            self.data[link.start as usize + i] = 0;
            i += 1;
        }
    }

    pub fn get_data_from_link(&self, link: PageLink) -> &[u8] {
        &self.data[link.start as usize..link.start as usize + link.len as usize]
    }
}

#[cfg(test)]
mod tests {
    use crate::core::base::cast::usize::USIZE_SIZE;
    use crate::page::page_struct::PAGE_SIZE;

    #[test]
    fn test_page_info_new() {
        let info = super::PageInfo::new(0);
        assert_eq!(info.index, 0);
        assert_eq!(info.free, PAGE_SIZE);
    }

    #[test]
    fn test_page_info_from() {
        let index: usize = 2;
        let free: u16 = 10;

        let mut bytes = [0; USIZE_SIZE + 2];
        bytes[0..USIZE_SIZE].copy_from_slice(&index.to_be_bytes());
        bytes[USIZE_SIZE..USIZE_SIZE + 2].copy_from_slice(&free.to_be_bytes());

        let info = super::PageInfo::from(bytes);
        assert_eq!(info.index, index);
        assert_eq!(info.free, free);
    }

    #[test]
    fn test_page_info_into() {
        let index: usize = 2;
        let free: u16 = 4092;

        let mut bytes = [0; USIZE_SIZE + 2];
        bytes[0..USIZE_SIZE].copy_from_slice(&index.to_be_bytes());
        bytes[USIZE_SIZE..USIZE_SIZE + 2].copy_from_slice(&free.to_be_bytes());

        let info = super::PageInfo::from(bytes);
        let bytes: [u8; USIZE_SIZE + 2] = info.into();
        assert_eq!(bytes[0..USIZE_SIZE], index.to_be_bytes());
        assert_eq!(bytes[USIZE_SIZE..USIZE_SIZE + 2], free.to_be_bytes());
    }

    #[test]
    fn test_page_new() {
        let page = super::Page::new(0);
        assert_eq!(page.info.index, 0);
        assert_eq!(page.info.free, PAGE_SIZE);
    }

    #[test]
    fn test_page_get_data() {
        let mut page = super::Page::new(0);
        let info = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        page.attach_data(&info);
        assert_eq!(page.get_data(), &info);
    }

    #[test]
    fn test_page_attach_info() {
        let mut page = super::Page::new(0);
        let info = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        page.attach_data(&info);
        assert_eq!(page.info.free, PAGE_SIZE - info.len() as u16);
    }

    #[test]
    fn test_page_attach_to_existing() {
        let mut page = super::Page::new(0);
        let info = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        page.attach_data(&info);
        let info2 = [11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        page.attach_data(&info2);
        assert_eq!(
            page.info.free,
            PAGE_SIZE - info.len() as u16 - info2.len() as u16
        );
        let mut expected: [u8; 20] = [0; 20];
        expected[0..10].copy_from_slice(&info);
        expected[10..20].copy_from_slice(&info2);
        assert_eq!(page.get_data(), &expected);
    }

    #[test]
    fn test_page_update_data() {
        let mut page = super::Page::new(0);
        let info = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        page.attach_data(&info);
        let data = [11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

        let link = super::PageLink::new(0, 0, 10);

        page.update_data(&data, link).unwrap();
        assert_eq!(page.info.free, PAGE_SIZE - info.len() as u16);
        assert_eq!(page.get_data(), &data);
    }

    #[test]
    fn test_page_erase_data() {
        let mut page = super::Page::new(0);
        let info = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        page.attach_data(&info);

        let link = super::PageLink::new(0, 0, 5);

        page.erase_data(link);
        assert_eq!(page.info.free, PAGE_SIZE - info.len() as u16);
        assert_eq!(page.get_data(), &[0, 0, 0, 0, 0, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_page_get_data_from_link() {
        let mut page = super::Page::new(0);
        let info = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        page.attach_data(&info);

        let link = super::PageLink::new(0, 0, 5);

        assert_eq!(page.get_data_from_link(link), &[1, 2, 3, 4, 5]);
    }
}
