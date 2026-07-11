// source snippet: key=LexicalPermutation  prefix=LexicalPermutation
// (旧実装/CamelCase。lib_ 版の旧バージョンの可能性あり)

/// Ported from [bluss/permutohedron](https://github.com/bluss/permutohedron)
pub trait LexicalPermutation {
    /// Return \`true\` if the slice was permuted, \`false\` if it is already
    /// at the last ordered permutation.
    fn next_permutation(&mut self) -> bool;
    /// Return \`true\` if the slice was permuted, \`false\` if it is already
    /// at the first ordered permutation.
    fn prev_permutation(&mut self) -> bool;
}
impl<T> LexicalPermutation for [T]
where
    T: Ord,
{
    /// Original author in Rust: Thomas Backman <serenity@exscape.org>
    fn next_permutation(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i - 1] {
            j -= 1;
        }
        self.swap(j, i - 1);
        self[i..].reverse();
        true
    }
    fn prev_permutation(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] <= self[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        self[i..].reverse();
        let mut j = self.len() - 1;
        while j >= i && self[j - 1] < self[i - 1] {
            j -= 1;
        }
        self.swap(i - 1, j);
        true
    }
}
