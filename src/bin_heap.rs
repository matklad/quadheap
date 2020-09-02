pub struct BinHeap<T> {
    vec: Vec<T>,
}

impl<T: Ord> BinHeap<T> {
    pub fn with_capacity(cap: usize) -> BinHeap<T> {
        BinHeap {
            vec: Vec::with_capacity(cap),
        }
    }

    pub fn push(&mut self, value: T) {
        self.vec.push(value);
        self.sift_up(self.vec.len() - 1)
    }
    pub fn pop_min(&mut self) -> Option<T> {
        if self.vec.is_empty() {
            return None;
        }
        let res = self.vec.swap_remove(0);
        self.sift_down(0);
        Some(res)
    }
    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let p_idx = (idx - 1) / 2;
            if self.vec[p_idx] <= self.vec[idx] {
                break;
            }
            self.vec.swap(p_idx, idx);
            idx = p_idx;
        }
    }
    fn sift_down(&mut self, mut idx: usize) {
        loop {
            let (c1_idx, c2_idx) = (2 * idx + 1, 2 * idx + 2);
            if !(c2_idx < self.vec.len()) {
                if c1_idx < self.vec.len() && !(self.vec[idx] <= self.vec[c1_idx]) {
                    self.vec.swap(idx, c1_idx);
                }
                break;
            }
            let c_idx = c1_idx + (self.vec[c2_idx] < self.vec[c1_idx]) as usize;
            if self.vec[idx] <= self.vec[c_idx] {
                break;
            }
            self.vec.swap(idx, c_idx);
            idx = c_idx;
        }
    }
}
