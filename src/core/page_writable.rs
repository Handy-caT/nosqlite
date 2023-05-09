use crate::core::link_struct::PageLink;
use crate::core::page_struct::Page;

pub trait PageWritable {
    fn write_with_link(self, page: &mut Page, link: &mut PageLink) -> Result<PageLink, String>;
    fn write(self, page: &mut Page) -> Result<PageLink, String>;

    fn read(page: &Page, link: &PageLink) -> Result<Self, String>
    where
        Self: Sized;
}


impl PageWritable for PageLink {
    fn write_with_link(self, page: &mut Page, link: &mut PageLink) -> Result<PageLink, String> {
        let mut bytes: [u8; 16] = self.into();
        if !page.can_fit(bytes.len() as u16) {
            return Err("Page is full".to_string());
        }
        let first_free = page.get_first_free();

        page.update_data(&bytes, link).unwrap();
        let res_link = PageLink::new(link.get_page_index(), first_free as u32, 16);
        return Ok(res_link);
    }

    fn write(self, page: &mut Page) -> Result<PageLink, String> {
        let mut bytes: [u8; 16] = self.into();
        let first_free = page.get_first_free();
        if !page.can_fit(bytes.len() as u16) {
            return Err("Page is full".to_string());
        }

        let link = PageLink::new(page.get_index(), first_free as u32, 16);
        page.update_data(&bytes, &link).unwrap();
        return Ok(link);
    }

    fn read(page: &Page, link: &PageLink) -> Result<Self, String> {
        let mut bytes: &[u8] = page.get_data_from_link(link);

        let res_link = PageLink::from(<[u8; 16]>::try_from(bytes).unwrap());
        return Ok(res_link);
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::page_struct::Page;

    #[test]
    fn test_write() {
        let mut page = Page::new(0);
        let link = PageLink::new(0, 0, 16);
        let bytes: [u8; 16] = link.into();
        let res_link = link.write(&mut page).unwrap();

        assert_eq!(res_link.get_page_index(), 0);
        assert_eq!(res_link.get_start(), 0);
        assert_eq!(res_link.get_len(), 16);

        assert_eq!(page.get_data_from_link(&res_link), bytes);
    }

    #[test]
    fn test_write_with_link() {
        let mut page = Page::new(0);
        let link = PageLink::new(0, 0, 16);
        let bytes: [u8; 16] = link.into();
        let mut res_link = PageLink::new(0, 0, 16);

        link.write_with_link(&mut page, &mut res_link).unwrap();

        assert_eq!(res_link.get_page_index(), 0);
        assert_eq!(res_link.get_start(), 0);
        assert_eq!(res_link.get_len(), 16);

        assert_eq!(page.get_data_from_link(&res_link), bytes);
    }

    #[test]
    fn test_read() {
        let mut page = Page::new(0);
        let link = PageLink::new(0, 0, 16);
        let res_link = link.write(&mut page).unwrap();

        let res_link = PageLink::read(&page, &res_link).unwrap();

        assert_eq!(res_link.get_page_index(), 0);
        assert_eq!(res_link.get_start(), 0);
        assert_eq!(res_link.get_len(), 16);
    }
}