// source snippet: key=lib_multiset_i64  prefix=lib_multiset_i64

#[derive(Clone, Debug)]
struct MultiSet {
    set: BTreeSet<i64>,
    map: HashMap<i64, i64>,
}

impl MultiSet {
    fn new() -> MultiSet {
        MultiSet {
            set: BTreeSet::new(),
            map: HashMap::new(),
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
    fn insert(&mut self, i: i64) -> Option<i64> {
        if let Some(_i) = self.set.get(&i) {
            //setにある
            *self.map.entry(i).or_insert(0) += 1;
        } else {
            //setにない
            self.set.insert(i);
            *self.map.entry(i).or_insert(0) += 1;
        }
        return Some(i);
    }

    ///1つ削除
    fn erase(&mut self, e: i64) -> Option<i64> {
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
    fn get_min(&self) -> Option<i64> {
        if let Some(&m) = self.set.iter().nth(0) {
            return Some(m);
        } else {
            return None;
        }
    }

    ///最大値の取得
    fn get_max(&self) -> Option<i64> {
        if let Some(&m) = self.set.iter().last() {
            return Some(m);
        } else {
            return None;
        }
    }
}
