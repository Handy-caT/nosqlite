use std::intrinsics::size_of;

pub const USIZE_SIZE: usize = size_of::<usize>();

pub trait Usize {
    fn to_usize(self) -> usize;
}

impl Usize for u64 {
    fn to_usize(self) -> usize {
        if let Ok(res) = self.try_into() {
            res
        } else {
            let round = self >> 32;
            if let Ok(res) = round.try_into() {
                res
            } else {
                let round = self >> 16;
                round as usize
            }
        }
    }
}

impl Usize for u32 {
    fn to_usize(self) -> usize {
        if let Ok(res) = self.try_into() {
            res
        } else {
            let round = self >> 16;
            round as usize
        }
    }
}
