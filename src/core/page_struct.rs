struct PageInfo {
    index: u64,
    free: u16,
}

impl PageInfo {
    fn new(index: u64) -> PageInfo {
        PageInfo {
            index,
            free: 4096,
        }
    }
}

impl From<&[u8]> for PageInfo {
    fn from(bytes: &[u8]) -> Self {
        let index = u64::from_be_bytes(bytes[0..8].try_into().unwrap());
        let free = u16::from_be_bytes(bytes[8..10].try_into().unwrap());
        PageInfo {
            index,
            free,
        }
    }
}

impl Into<[u8]> for PageInfo {
    fn into(self) -> [u8; 10] {
        let mut bytes = [0; 10];
        bytes[0..8].copy_from_slice(&self.index.to_be_bytes());
        bytes[8..10].copy_from_slice(&self.free.to_be_bytes());
        bytes
    }
}

struct Page {
    info: PageInfo,
    data: [u8; 4096],
}

impl Page {
    fn new(index: u64) -> Page {
        Page {
            info: PageInfo::new(index),
            data: [0; 4096],
        }
    }

    fn get_data(&self) -> &[u8] {
        &self.data[0..(4096 - self.info.free as usize)]
    }

    fn attach_info(&mut self, info: &[u8]) {
        let mut i = 0;

        while i < info.len() {
            self.data[4096 - self.info.free as usize] = info[i];
            self.info.free -= 1;
            i += 1;
        }
    }
}