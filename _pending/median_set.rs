// source snippet: key=lib_median_set  prefix=lib_median_set

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug)]
struct MultiSet<T> {
    set: BTreeSet<T>,
    map: BTreeMap<T, usize>,
    ulen: usize,
}

impl<T: Ord + Copy + Display> MultiSet<T> {
    fn new() -> MultiSet<T> {
        MultiSet {
            set: BTreeSet::new(),
            map: BTreeMap::new(),
            ulen: 0,
        }
    }

    ///multiset的に書き出し
    fn print(&self) {
        print!("{{ ");
        for x in &self.set {
            if let Some(&num) = self.map.get(x) {
                for _i in 0..num {
                    print!("{} ", x);
                }
            }
        }
        println!("}}");
    }

    ///重複許可挿入
    fn insert(&mut self, i: T) -> Option<T> {
        if let Some(_i) = self.set.get(&i) {
            //setにある
            *self.map.entry(i).or_insert(0) += 1;
        } else {
            //setにない
            self.set.insert(i);
            *self.map.entry(i).or_insert(0) += 1;
        }
        self.ulen += 1;
        return Some(i);
    }

    ///1つ削除
    fn erase(&mut self, e: T) -> Option<T> {
        self.ulen -= 1;
        if let Some(_e) = self.set.get(&e) {
            //setにある
            *self.map.entry(e).or_insert(0) -= 1;
            if self.map[&e] == 0 {
                //なくなった
                self.set.take(&e);
            }
            return Some(e);
        } else {
            //setにない
            return None;
        }
    }

    ///最小値の取得
    fn get_min(&self) -> Option<T> {
        if let Some(&m) = self.set.iter().nth(0) {
            return Some(m);
        } else {
            return None;
        }
    }

    ///最大値の取得
    fn get_max(&self) -> Option<T> {
        if let Some(&m) = self.set.iter().last() {
            return Some(m);
        } else {
            return None;
        }
    }
    fn len(&self) -> usize {
        self.ulen
    }
}
#[test]
fn test_multiset() {
    let mut ms = MultiSet::new();
    ms.insert(1);
    ms.insert(2);
    ms.insert(3);
    assert_eq!(ms.len(), 3);
    assert_eq!(ms.get_max(), Some(3));
    assert_eq!(ms.get_min(), Some(1));
}

struct MedianSet {
    lset: MultiSet<i64>,
    rset: MultiSet<i64>,
}

impl MedianSet {
    fn new() -> MedianSet {
        MedianSet {
            lset: MultiSet::new(),
            rset: MultiSet::new(),
        }
    }
    fn insert(&mut self, input: i64) {
        self.lset.insert(input);
        let x = self.lset.get_max().unwrap();
        self.lset.erase(x);
        self.rset.insert(x);
        while self.lset.len() < self.rset.len() {
            let x = self.rset.get_min().unwrap();
            self.rset.erase(x);
            self.lset.insert(x);
        }
    }

    fn get_med(&mut self) -> i64 {
        let x = self.lset.get_max().unwrap_or(0);
        if self.lset.len() == self.rset.len() + 1 {
            return x;
        }
        let y = self.rset.get_min().unwrap_or(INF);

        return (x + y) / 2;
    }

    fn erase(&mut self, val: i64) {
        let m = self.get_med();
        if val <= m {
            self.lset.erase(val);
        } else {
            self.rset.erase(val);
        }
        if self.lset.len() == 0 || self.rset.len() == 0 {
            return;
        }
        let x = self.lset.get_max().unwrap();
        self.lset.erase(x);
        self.rset.insert(x);
        while self.lset.len() < self.rset.len() {
            let x = self.rset.get_min().unwrap();
            self.rset.erase(x);
            self.lset.insert(x);
        }
    }
}
#[test]

fn test_medianset() {
    let mut ms = MedianSet::new();
    ms.insert(1);

    ms.insert(2);

    ms.insert(4);

    ms.insert(4);
    assert_eq!(ms.get_med(), 3);
}
