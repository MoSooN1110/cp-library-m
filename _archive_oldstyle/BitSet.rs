// source snippet: key=BitSet  prefix=BitSet
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

const TRUE: &bool = &true;
const FALSE: &bool = &false;
#[derive(Clone, Debug)]
/// Efficient bool collection
pub struct BitSet {
    buf: Vec<u64>,
    size: usize,
}
impl BitSet {
    #[allow(dead_code)]
    pub fn new(size: usize) -> BitSet {
        BitSet {
            buf: vec![0; (size + 63) / 64],
            size,
        }
    }
    #[allow(dead_code)]
    pub fn set(&mut self, i: usize, b: bool) {
        assert!(i < self.size);
        if b {
            self.buf[i >> 6] |= 1 << (i & 63);
        } else {
            self.buf[i >> 6] &= !(1 << (i & 63));
        }
    }
    #[allow(dead_code)]
    pub fn count_ones(&self) -> u32 {
        self.buf.iter().map(|x| x.count_ones()).sum()
    }
    #[allow(dead_code)]
    fn chomp(&mut self) {
        let r = self.size & 63;
        if r != 0 {
            if let Some(x) = self.buf.last_mut() {
                let d = 64 - r;
                *x = (*x << d) >> d;
            }
        }
    }
}
impl std::ops::Index<usize> for BitSet {
    type Output = bool;
    fn index(&self, index: usize) -> &bool {
        [FALSE, TRUE][(self.buf[index >> 6] >> (index & 63)) as usize & 1]
    }
}
#[allow(clippy::suspicious_op_assign_impl)]
impl std::ops::ShlAssign<usize> for BitSet {
    fn shl_assign(&mut self, x: usize) {
        let q = x >> 6;
        let r = x & 63;
        if q >= self.buf.len() {
            for x in &mut self.buf {
                *x = 0;
            }
            return;
        }
        if r == 0 {
            for i in (q..self.buf.len()).rev() {
                self.buf[i] = self.buf[i - q];
            }
        } else {
            for i in (q + 1..self.buf.len()).rev() {
                self.buf[i] = (self.buf[i - q] << r) | (self.buf[i - q - 1] >> (64 - r));
            }
            self.buf[q] = self.buf[0] << r;
        }
        for x in &mut self.buf[..q] {
            *x = 0;
        }
        self.chomp();
    }
}
impl std::ops::Shl<usize> for BitSet {
    type Output = Self;
    fn shl(mut self, x: usize) -> Self {
        self <<= x;
        self
    }
}
#[allow(clippy::suspicious_op_assign_impl)]
impl std::ops::ShrAssign<usize> for BitSet {
    fn shr_assign(&mut self, x: usize) {
        let q = x >> 6;
        let r = x & 63;
        if q >= self.buf.len() {
            for x in &mut self.buf {
                *x = 0;
            }
            return;
        }
        if r == 0 {
            for i in 0..self.buf.len() - q {
                self.buf[i] = self.buf[i + q];
            }
        } else {
            for i in 0..self.buf.len() - q - 1 {
                self.buf[i] = (self.buf[i + q] >> r) | (self.buf[i + q + 1] << (64 - r));
            }
            let len = self.buf.len();
            self.buf[len - q - 1] = self.buf[len - 1] >> r;
        }
        let len = self.buf.len();
        for x in &mut self.buf[len - q..] {
            *x = 0;
        }
    }
}
impl std::ops::Shr<usize> for BitSet {
    type Output = Self;
    fn shr(mut self, x: usize) -> Self {
        self >>= x;
        self
    }
}
impl<'a> std::ops::BitAndAssign<&'a BitSet> for BitSet {
    fn bitand_assign(&mut self, rhs: &'a Self) {
        for (a, b) in self.buf.iter_mut().zip(rhs.buf.iter()) {
            *a &= *b;
        }
    }
}
impl<'a> std::ops::BitAnd<&'a BitSet> for BitSet {
    type Output = Self;
    fn bitand(mut self, rhs: &'a Self) -> Self {
        self &= rhs;
        self
    }
}
impl<'a> std::ops::BitOrAssign<&'a BitSet> for BitSet {
    fn bitor_assign(&mut self, rhs: &'a Self) {
        for (a, b) in self.buf.iter_mut().zip(rhs.buf.iter()) {
            *a |= *b;
        }
        self.chomp();
    }
}
impl<'a> std::ops::BitOr<&'a BitSet> for BitSet {
    type Output = Self;
    fn bitor(mut self, rhs: &'a Self) -> Self {
        self |= rhs;
        self
    }
}
impl<'a> std::ops::BitXorAssign<&'a BitSet> for BitSet {
    fn bitxor_assign(&mut self, rhs: &'a Self) {
        for (a, b) in self.buf.iter_mut().zip(rhs.buf.iter()) {
            *a ^= *b;
        }
        self.chomp();
    }
}
impl<'a> std::ops::BitXor<&'a BitSet> for BitSet {
    type Output = Self;
    fn bitxor(mut self, rhs: &'a Self) -> Self {
        self ^= rhs;
        self
    }
}
