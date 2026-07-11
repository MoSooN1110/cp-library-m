// source snippet: key=lib_centoroid_decomp  prefix=lib_centoroid_decomp

//c.f.https://qiita.com/drken/items/4b4c3f1824339b090202
//only 1 centoroid
//(recursive は検証していない)
struct Tree {
    n: usize,                  // ツリーのサイズ
    max_v: usize,              // ツリーのサイズの最大値
    tree: Vec<Vec<usize>>,     // ツリーを隣接リスト形式のグラフ構造で表したもの
    size_subtree: Vec<usize>, // size_subtree[v] := v を根とする部分ツリーのサイズ (分割統治の毎ステップごとに再利用)
    is_removed: Vec<bool>,    // is_removed[v] := v が既に取り除かれたかどうか
    who_is_parent: Vec<isize>, // who_is_parent[v] := ツリーDP時に v の親が誰だったか
    centroids: Vec<usize>,    // 重心のリスト
}

impl Tree {
    fn new(n: usize, max_v: usize) -> Self {
        Self {
            n,
            max_v,
            tree: vec![vec![]; max_v],
            size_subtree: vec![0; max_v],
            is_removed: vec![false; max_v],
            who_is_parent: vec![-1; max_v],
            centroids: vec![],
        }
    }

    fn find_centroid_recursive(&mut self, v: usize, size: usize, p: isize) {
        self.size_subtree[v] = 1;
        self.who_is_parent[v] = p;
        let mut is_centroid = true;
        let children = self.tree[v].clone(); // 子ノードのクローンを作成
        for &ch in &children {
            if ch == p as usize || self.is_removed[ch] {
                continue;
            }
            self.find_centroid_recursive(ch, size, v as isize);
            if self.size_subtree[ch] > size / 2 {
                is_centroid = false;
            }
            self.size_subtree[v] += self.size_subtree[ch];
        }
        if size - self.size_subtree[v] > size / 2 {
            is_centroid = false;
        }
        if is_centroid {
            self.centroids.push(v);
        }
    }

    fn init(&mut self) {
        for i in 0..self.max_v {
            self.is_removed[i] = false;
        }
    }

    // first: 重心, second: (重心の子ノード, 子部分木のサイズ) からなるベクトル
    fn find_centroid(&mut self, root: usize, size: usize) -> (usize, Vec<(usize, usize)>) {
        let mut subtrees = Vec::new();
        self.centroids.clear();
        self.find_centroid_recursive(root, size, -1);
        let center = self.centroids[0];
        let children = self.tree[center].clone(); // 子ノードのクローンを作成
        for &ch in &children {
            if self.is_removed[ch] {
                continue;
            }
            if ch == self.who_is_parent[center] as usize {
                subtrees.push((ch, size - self.size_subtree[center]));
            } else {
                subtrees.push((ch, self.size_subtree[ch]));
            }
        }
        (center, subtrees)
    }
}
