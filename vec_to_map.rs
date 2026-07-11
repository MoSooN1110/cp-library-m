// source snippet: key=lib_vec_to_map  prefix=lib_vec_to_map

pub trait VecToMap<T> {
    fn vec_to_map(&self) -> BTreeMap<T, usize>;
}
impl<T: Ord + Copy> VecToMap<T> for [T] {
    fn vec_to_map(&self) -> BTreeMap<T, usize> {
        let mut bt = BTreeMap::new();
        for i in 0..self.len() {
            *bt.entry(self[i]).or_insert(0) += 1;
        }
        return bt;
    }
}
