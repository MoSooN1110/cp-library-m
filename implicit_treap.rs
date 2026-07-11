// source snippet: key=lib_implicit_treap  prefix=lib_implicit_treap
// Your description.

//randuse
use rand::Rng;

/// Treapの各ノードを表す構造体。T は格納する値の型です。
#[derive(Debug)]
struct Node<T> {
	value: T,
	priority: u32,
	size: usize,
	left: Option<Box<Node<T>>>,
	right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
	/// 新しいノードを生成します。
	fn new(value: T) -> Box<Self> {
		let mut rng = rand::thread_rng();
		Box::new(Node {
			value,
			priority: rng.gen(),
			size: 1,
			left: None,
			right: None,
		})
	}

	/// 子ノードから部分木サイズを再計算します。
	fn update(&mut self) {
		self.size = 1;
		if let Some(ref l) = self.left {
			self.size += l.size;
		}
		if let Some(ref r) = self.right {
			self.size += r.size;
		}
	}
}

/// Implicit Treapの構造体。内部にrootノードを持ちます。
pub struct ImplicitTreap<T> {
	root: Option<Box<Node<T>>>,
}

impl<T> ImplicitTreap<T> {
	/// 空のImplicit Treapを生成します。
	pub fn new() -> Self {
		Self { root: None }
	}

	/// ノードのサイズを返します（Noneの場合は0）。
	fn size(node: &Option<Box<Node<T>>>) -> usize {
		node.as_ref().map_or(0, |n| n.size)
	}

	/// 木を先頭からkey個のノードと残りに分割します。
	/// 戻り値は (left, right) となります。
	pub fn split(
		node: Option<Box<Node<T>>>,
		key: usize,
	) -> (Option<Box<Node<T>>>, Option<Box<Node<T>>>) {
		if let Some(mut n) = node {
			let left_size = Self::size(&n.left);
			if key <= left_size {
				let (left, right) = Self::split(n.left.take(), key);
				n.left = right;
				n.update();
				(left, Some(n))
			} else {
				let (left, right) = Self::split(n.right.take(), key - left_size - 1);
				n.right = left;
				n.update();
				(Some(n), right)
			}
		} else {
			(None, None)
		}
	}

	/// 2つのTreapをマージします。leftの全ノードがrightの全ノードより前に来る前提です。
	pub fn merge(left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
		match (left, right) {
			(None, r) => r,
			(l, None) => l,
			(Some(mut lnode), Some(mut rnode)) => {
				if lnode.priority > rnode.priority {
					lnode.right = Self::merge(lnode.right.take(), Some(rnode));
					lnode.update();
					Some(lnode)
				} else {
					rnode.left = Self::merge(Some(lnode), rnode.left.take());
					rnode.update();
					Some(rnode)
				}
			}
		}
	}

	/// pos の位置に value を挿入します。
	pub fn insert(&mut self, pos: usize, value: T) {
		let new_node = Some(Node::new(value));
		let (left, right) = Self::split(self.root.take(), pos);
		self.root = Self::merge(Self::merge(left, new_node), right);
	}

	/// pos の位置の要素を削除し、その値を返します。
	pub fn remove(&mut self, pos: usize) -> Option<T> {
		let (left, mid_right) = Self::split(self.root.take(), pos);
		let (mid, right) = Self::split(mid_right, 1);
		self.root = Self::merge(left, right);
		mid.map(|node| node.value)
	}

	/// pos の位置の要素への参照を返します。
	pub fn get(&self, pos: usize) -> Option<&T> {
		Self::get_node(&self.root, pos)
	}

	fn get_node<'a>(node: &'a Option<Box<Node<T>>>, pos: usize) -> Option<&'a T> {
		if let Some(n) = node {
			let left_size = Self::size(&n.left);
			if pos < left_size {
				Self::get_node(&n.left, pos)
			} else if pos == left_size {
				Some(&n.value)
			} else {
				Self::get_node(&n.right, pos - left_size - 1)
			}
		} else {
			None
		}
	}

	/// 中順巡回で要素を順番に取得します。
	/// ここでは各要素への参照のVecとして返します。
	pub fn inorder(&self) -> Vec<&T> {
		let mut res = Vec::new();
		Self::inorder_node(&self.root, &mut res);
		res
	}

	fn inorder_node<'a>(node: &'a Option<Box<Node<T>>>, res: &mut Vec<&'a T>) {
		if let Some(n) = node {
			Self::inorder_node(&n.left, res);
			res.push(&n.value);
			Self::inorder_node(&n.right, res);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::ImplicitTreap;

	#[test]
	fn test_characters() {
		let mut treap = ImplicitTreap::new();
		// 文字列 "hello" の各文字を順番に挿入
		for (i, ch) in "hello".chars().enumerate() {
			treap.insert(i, ch);
		}
		let result: String = treap.inorder().into_iter().copied().collect();
		assert_eq!(result, "hello");

		// 位置1 ('e') を削除 → "hllo" になるはず
		let removed = treap.remove(1);
		assert_eq!(removed, Some('e'));
		let result: String = treap.inorder().into_iter().copied().collect();
		assert_eq!(result, "hllo");
	}

	#[test]
	fn test_integers() {
		let mut treap = ImplicitTreap::new();
		// 0～9を挿入
		for i in 0..10 {
			treap.insert(i, i);
		}
		// 中順巡回で確認
		let result: Vec<_> = treap.inorder().into_iter().copied().collect();
		assert_eq!(result, (0..10).collect::<Vec<_>>());
		// 位置5の要素（5）を削除
		let removed = treap.remove(5);
		assert_eq!(removed, Some(5));
		let result: Vec<_> = treap.inorder().into_iter().copied().collect();
		let expected: Vec<_> = vec![0, 1, 2, 3, 4, 6, 7, 8, 9];
		assert_eq!(result, expected);
	}
}
