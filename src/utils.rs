pub trait DiffOrZero {
    fn diff_or_zero(&self, other: &Self) -> Self;
}

impl DiffOrZero for u16 {
    fn diff_or_zero(&self, b: &u16) -> u16 {
        if self > b {
            return self - b;
        } else {
            return 0;
        }
    }
}