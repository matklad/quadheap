pub struct QuadHeap<T> {
    pub(crate) vec: Vec<T>,
}

impl<T: Ord + std::fmt::Debug> QuadHeap<T> {
    pub fn with_capacity(cap: usize) -> QuadHeap<T> {
        QuadHeap {
            vec: Vec::with_capacity(cap),
        }
    }

    pub fn push(&mut self, value: T) {
        self.vec.push(value);
        self.sift_up(self.vec.len() - 1)
    }
    pub fn pop_min(&mut self) -> Option<T> {
        match self.vec.len() {
            0 => None,
            1 => self.vec.pop(),
            2 => {
                if self.vec[0] < self.vec[1] {
                    self.vec.swap(0, 1)
                }
                self.vec.pop()
            }
            _ => {
                let mut c_idx = 1 + (self.vec[2] < self.vec[1]) as usize;
                if self.vec[0] < self.vec[c_idx] {
                    c_idx = 0;
                }
                let res = self.vec.swap_remove(c_idx);
                self.sift_down(c_idx);
                Some(res)
            }
        }
    }
    fn sift_up(&mut self, mut idx: usize) {
        while idx > 2 {
            let p_idx = (idx - 3) / 4;
            if self.vec[p_idx] <= self.vec[idx] {
                break;
            }
            self.vec.swap(p_idx, idx);
            idx = p_idx;
        }
    }
    fn sift_down(&mut self, mut idx: usize) {
        loop {
            let (c1_idx, c2_idx, c3_idx, c4_idx) =
                (4 * idx + 3, 4 * idx + 4, 4 * idx + 5, 4 * idx + 6);
            if !(c4_idx < self.vec.len()) {
                if let Some((c_idx, _)) = self
                    .vec
                    .get(c1_idx..)
                    .unwrap_or_default()
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, it)| *it)
                {
                    if !(self.vec[idx] <= self.vec[c1_idx + c_idx]) {
                        self.vec.swap(idx, c1_idx + c_idx);
                    }
                }
                break;
            }
            let c12_idx = c1_idx + (self.vec[c2_idx] < self.vec[c1_idx]) as usize;
            let c34_idx = c3_idx + (self.vec[c4_idx] < self.vec[c3_idx]) as usize;
            let c_idx = if self.vec[c12_idx] < self.vec[c34_idx] {
                c12_idx
            } else {
                c34_idx
            };
            if self.vec[idx] <= self.vec[c_idx] {
                break;
            }
            self.vec.swap(idx, c_idx);
            idx = c_idx;
        }
    }
}
