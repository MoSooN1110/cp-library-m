// source snippet: key=lib_dynamic_segtree  prefix=lib_dynamic_segtree

pub trait Monoid {
    type S;
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
    fn id() -> Self::S;
}

pub struct AddMonoid;

impl Monoid for AddMonoid {
    type S = usize;
    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        *a + *b
    }
    fn id() -> Self::S {
        0
    }
}

/// Let $\\{a_{i}\\}_{i=1}^{N}$ be a sequence of type Monoid::S.
pub struct SegmentTreeDynamic<M>
where
    M: Monoid,
{
    size: usize,
    data: HashMap<usize, M::S>,
}

impl<M> SegmentTreeDynamic<M>
where
    M: Monoid,
    M::S: Clone,
{
    /// Creates a segment tree with $\\{a_{i}\\}_{i=1}^{N}$ inside.
    /// n: lenght of $\\{a_{i}\\}_{i=1}^{N}$ (i.e. N)
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        SegmentTreeDynamic::<M> {
            size,
            data: HashMap::new(),
        }
    }

    /// Updates $a_{idx}$ to x.
    pub fn update(&mut self, mut idx: usize, x: M::S) {
        idx += self.size - 1;
        self.data.insert(idx, x);
        while idx > 0 {
            idx = (idx - 1) / 2;
            *self.data.entry(idx).or_insert(M::id()) = M::op(
                &self.data.get(&(2 * idx + 1)).unwrap_or(&M::id()),
                &self.data.get(&(2 * idx + 2)).unwrap_or(&M::id()),
            );
        }
    }

    /// Returns $a_{idx}$.
    pub fn get(&self, idx: usize) -> M::S {
        self.fold(idx, idx + 1)
    }

    /// Returns the result (fold op $\left[a_{l}, ... ,a_{r}\right)).$
    /// (i.e. Return $a_{l} (op) a_{l + 1} (op) \cdots (op) a_{r-1})$
    /// Notice that this is a half-opened section.
    pub fn fold(&self, mut l: usize, mut r: usize) -> M::S {
        l += self.size - 1;
        r += self.size - 1;

        let mut sum_l = M::id();
        let mut sum_r = M::id();

        while l < r {
            if l % 2 == 0 {
                sum_l = M::op(&sum_l, &self.data.get(&(l)).unwrap_or(&M::id()));
            }
            if r % 2 == 0 {
                sum_r = M::op(&self.data.get(&(r - 1)).unwrap_or(&M::id()), &sum_r);
            }
            l = l / 2;
            r = (r - 1) / 2;
        }

        M::op(&sum_l, &sum_r)
    }
}
