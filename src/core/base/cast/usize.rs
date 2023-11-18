pub trait UsizeCast {
    fn to_usize(self) -> usize;
}

impl UsizeCast for u64 {
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

impl UsizeCast for u32 {
    fn to_usize(self) -> usize {
        if let Ok(res) = self.try_into() {
            res
        } else {
            let round = self >> 16;
            round as usize
        }
    }
}
