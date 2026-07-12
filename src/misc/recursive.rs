//! Recursive closure helpers without macros.
//!
//! Arguments are passed as a single value. Use tuples for functions with
//! multiple arguments.
//!
//! ```
//! use cplib::misc::recursive::*;
//!
//! let mut fact = RecursiveFunction::new(|f, n: u64| {
//!     if n <= 1 { 1 } else { n * f(n - 1) }
//! });
//! assert_eq!(fact.call(10), 3_628_800);
//!
//! let mut comb = MemoizedRecursiveFunction::new(|f, (n, r): (usize, usize)| {
//!     if r > n {
//!         0usize
//!     } else if r == 0 || r == n {
//!         1
//!     } else {
//!         f((n - 1, r)) + f((n - 1, r - 1))
//!     }
//! });
//! assert_eq!(comb.call((30, 12)), 86_493_225);
//! ```

use std::collections::HashMap;
use std::hash::{BuildHasher, Hash, RandomState};
use std::marker::PhantomData;

/// A callable wrapper for recursive closures.
///
/// The closure receives a recursive callback and the current argument.
pub struct RecursiveFunction<Arg, Ret, F> {
    f: F,
    _marker: PhantomData<fn(Arg) -> Ret>,
}

impl<Arg, Ret, F> RecursiveFunction<Arg, Ret, F>
where
    F: FnMut(&mut dyn FnMut(Arg) -> Ret, Arg) -> Ret,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _marker: PhantomData,
        }
    }

    pub fn call(&mut self, arg: Arg) -> Ret {
        fn call_inner<Arg, Ret, F>(f: *mut F, arg: Arg) -> Ret
        where
            F: FnMut(&mut dyn FnMut(Arg) -> Ret, Arg) -> Ret,
        {
            let f_ptr = f;
            let mut recur = move |next| call_inner(f_ptr, next);
            unsafe { (&mut *f)(&mut recur, arg) }
        }

        call_inner(&mut self.f, arg)
    }
}

/// A recursive closure with `HashMap` memoization.
///
/// The argument type is also the memoization key, so use a tuple for multiple
/// arguments.
pub struct MemoizedRecursiveFunction<Arg, Ret, F, S = RandomState> {
    memo: HashMap<Arg, Ret, S>,
    f: F,
    _marker: PhantomData<fn(Arg) -> Ret>,
}

impl<Arg, Ret, F> MemoizedRecursiveFunction<Arg, Ret, F, RandomState>
where
    Arg: Eq + Hash + Clone,
    Ret: Clone,
    F: FnMut(&mut dyn FnMut(Arg) -> Ret, Arg) -> Ret,
{
    pub fn new(f: F) -> Self {
        Self::with_hasher(f, RandomState::new())
    }
}

impl<Arg, Ret, F, S> MemoizedRecursiveFunction<Arg, Ret, F, S>
where
    Arg: Eq + Hash + Clone,
    Ret: Clone,
    S: BuildHasher,
    F: FnMut(&mut dyn FnMut(Arg) -> Ret, Arg) -> Ret,
{
    pub fn with_hasher(f: F, hash_builder: S) -> Self {
        Self {
            memo: HashMap::with_hasher(hash_builder),
            f,
            _marker: PhantomData,
        }
    }

    pub fn call(&mut self, arg: Arg) -> Ret {
        fn call_inner<Arg, Ret, F, S>(
            this: *mut MemoizedRecursiveFunction<Arg, Ret, F, S>,
            arg: Arg,
        ) -> Ret
        where
            Arg: Eq + Hash + Clone,
            Ret: Clone,
            S: BuildHasher,
            F: FnMut(&mut dyn FnMut(Arg) -> Ret, Arg) -> Ret,
        {
            unsafe {
                if let Some(value) = (*this).memo.get(&arg).cloned() {
                    return value;
                }
                let this_ptr = this;
                let mut recur = move |next| call_inner(this_ptr, next);
                let value = (&mut (*this).f)(&mut recur, arg.clone());
                (*this).memo.insert(arg, value.clone());
                value
            }
        }

        call_inner(self, arg)
    }

    pub fn memo_len(&self) -> usize {
        self.memo.len()
    }

    pub fn get(&self, arg: &Arg) -> Option<&Ret> {
        self.memo.get(arg)
    }

    pub fn into_memo(self) -> HashMap<Arg, Ret, S> {
        self.memo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial() {
        let mut fact = RecursiveFunction::new(|f, n: u64| if n <= 1 { 1 } else { n * f(n - 1) });
        assert_eq!(fact.call(0), 1);
        assert_eq!(fact.call(10), 3_628_800);
    }

    #[test]
    fn can_capture_mutable_state() {
        let mut calls = 0usize;
        let mut sum_to = RecursiveFunction::new(|f, n: usize| {
            calls += 1;
            if n == 0 {
                0
            } else {
                n + f(n - 1)
            }
        });
        assert_eq!(sum_to.call(5), 15);
        drop(sum_to);
        assert_eq!(calls, 6);
    }

    #[test]
    fn memoized_fibonacci_reuses_states() {
        let mut fib = MemoizedRecursiveFunction::new(
            |f, n: usize| {
                if n < 2 {
                    n
                } else {
                    f(n - 1) + f(n - 2)
                }
            },
        );
        assert_eq!(fib.call(40), 102_334_155);
        assert_eq!(fib.memo_len(), 41);
        assert_eq!(fib.get(&10), Some(&55));
    }

    #[test]
    fn tuple_argument_combination() {
        let mut comb = MemoizedRecursiveFunction::new(|f, (n, r): (usize, usize)| {
            if r > n {
                0usize
            } else if r == 0 || r == n {
                1
            } else {
                f((n - 1, r)) + f((n - 1, r - 1))
            }
        });
        assert_eq!(comb.call((30, 12)), 86_493_225);
        assert_eq!(comb.call((6, 2)), 15);
    }
}
