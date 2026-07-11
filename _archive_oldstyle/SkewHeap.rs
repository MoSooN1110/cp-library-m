// source snippet: key=SkewHeap  prefix=SkewHeap
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

#[derive(Debug, Clone)]
struct SkewHeapNode<T: Ord> {
    v: T,
    l: SkewHeap<T>,
    r: SkewHeap<T>,
    length: usize,
}
#[derive(Debug, Clone)]
pub struct SkewHeap<T: Ord>(Option<Box<SkewHeapNode<T>>>);
impl<T: Ord> SkewHeapNode<T> {
    fn swap(&mut self) {
        let &mut SkewHeapNode {
            ref mut l,
            ref mut r,
            ..
        } = self;
        std::mem::swap(l, r);
    }
    fn divide(self) -> (T, SkewHeap<T>, SkewHeap<T>) {
        let SkewHeapNode { v, l, r, .. } = self;
        (v, l, r)
    }
}
impl<T: Ord> Default for SkewHeap<T> {
    fn default() -> Self {
        SkewHeap(None)
    }
}
impl<T: Ord> SkewHeap<T> {
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
    pub fn len(&self) -> usize {
        self.0.as_ref().map(|n| n.length).unwrap_or(0)
    }
    pub fn meld(&mut self, mut other: SkewHeap<T>) {
        if other.0.is_none() {
            return;
        }
        if self.0.is_none() {
            *self = other;
            return;
        }
        if self.0.as_ref().unwrap().as_ref().v < other.0.as_ref().unwrap().as_ref().v {
            std::mem::swap(self, &mut other);
        }
        if let Some(ref mut node) = self.0.as_mut() {
            node.length += other.0.as_ref().unwrap().length;
            node.r.meld(other);
            node.swap();
        }
    }
    pub fn push(&mut self, x: T) {
        let n = SkewHeap(Some(Box::new(SkewHeapNode {
            v: x,
            l: SkewHeap::default(),
            r: SkewHeap::default(),
            length: 1,
        })));
        self.meld(n);
    }
    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.0.take() {
            let (v, mut l, r) = node.divide();
            l.meld(r);
            *self = l;
            Some(v)
        } else {
            None
        }
    }
    pub fn peek(&self) -> Option<&T> {
        self.0.as_ref().map(|node| &node.v)
    }
}
