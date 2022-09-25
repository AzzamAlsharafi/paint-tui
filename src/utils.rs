pub trait DiffOrZero {
    fn diff_or_zero(&self, other: &Self) -> Self;
}

pub trait AddSubOrZero {
    fn add_sub_or_zero(&self, other: &i16) -> Self;
}

impl DiffOrZero for u16 {
    fn diff_or_zero(&self, b: &u16) -> u16 {
        if self > b {
            self - b
        } else {
            0
        }
    }
}

impl DiffOrZero for usize {
    fn diff_or_zero(&self, b: &usize) -> usize {
        if self > b {
            self - b
        } else {
            0
        }
    }
}

impl AddSubOrZero for u16 {
    fn add_sub_or_zero(&self, other: &i16) -> u16 {
        if other < &0 {
            return self.diff_or_zero(&other.unsigned_abs());
        }

        self + other.unsigned_abs()
    }
}

impl AddSubOrZero for usize {
    fn add_sub_or_zero(&self, other: &i16) -> usize {
        if other < &0 {
            return self.diff_or_zero(&usize::from(other.unsigned_abs()));
        }

        self + usize::from(other.unsigned_abs())
    }
}
