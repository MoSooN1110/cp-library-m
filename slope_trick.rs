// source snippet: key=lib_slope_trick  prefix=lib_slope_trick

pub struct SlopeTrick {
    l: BinaryHeap<i64>,
    r: BinaryHeap<i64>,
    al: i64,
    ar: i64,
    miny: i64,
}
impl SlopeTrick {
    // 0 <= size <= 10^8 is constrained.

    pub fn new() -> Self {
        Self {
            l: BinaryHeap::new(),
            r: BinaryHeap::new(),
            al: 0,
            ar: 0,
            miny: 0,
        }
    }
    pub fn push_left(&mut self, x: i64) {
        self.l.push(x - self.al);
    }

    pub fn push_right(&mut self, x: i64) {
        self.r.push(-1 * (x - self.ar));
    }
    pub fn pop_left(&mut self) -> i64 {
        let poped = self.l.pop().unwrap() + self.al;
        return poped;
    }
    pub fn pop_right(&mut self) -> i64 {
        let poped = -1 * self.r.pop().unwrap() + self.ar;
        return poped;
    }
    pub fn top_left(&mut self) -> i64 {
        if self.l.is_empty() {
            return std::i64::MAX;
        }
        return *self.l.peek().unwrap() + self.al;
    }
    pub fn top_right(&mut self) -> i64 {
        if self.r.is_empty() {
            return std::i64::MAX;
        }
        return -1 * (*self.r.peek().unwrap() - self.ar);
    }
    pub fn add_xma(&mut self, a: i64) {
        if !self.l.is_empty() {
            self.miny += max(0, self.top_left() - a);
        }
        self.push_left(a);
        let x = self.pop_left();
        self.push_right(x);
    }
    pub fn add_amx(&mut self, a: i64) {
        if !self.r.is_empty() {
            self.miny += max(0, -1 * self.top_right() + a);
        }
        self.push_right(a);
        let x = self.pop_right();
        self.push_left(x);
    }
}
