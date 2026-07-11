// source snippet: key=lib_rheap  prefix=lib_rheap

pub struct Rheap {
    heap: BinaryHeap<Reverse<i64>>,
}

impl Rheap {
    // 0 <= size <= 10^8 is constrained.
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }
    pub fn push(&mut self, x: i64) {
        self.heap.push(Reverse(x));
    }

    pub fn peak (&mut self) -> i64 {
        let Reverse(x) = self.heap.peek().unwrap();
        *x
    }
    pub fn pop(&mut self) -> Option<i64> {
        let x = self.heap.pop();
        if x == None {
            return None;
        }
        let Reverse(x) = x.unwrap();
        return Some(x);
    }
}
