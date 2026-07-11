// source snippet: key=lib_total_ord  prefix=lib_total_ord

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Total<T>(pub T);
impl<T: PartialEq> Eq for Total<T> {}
impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
impl<T: std::str::FromStr> std::str::FromStr for Total<T> {
    type Err = T::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = T::from_str(s)?;
        Ok(Total(res))
    }
}
impl<T: std::fmt::Display> std::fmt::Debug for Total<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<T: std::fmt::Display> std::fmt::Display for Total<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<T: std::ops::Add<Output = T>> std::ops::Add for Total<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Total(self.0 + other.0)
    }
}
impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Total<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Total(self.0 - other.0)
    }
}
impl<T: std::ops::Mul<Output = T>> std::ops::Mul for Total<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Total(self.0 * other.0)
    }
}
impl<T: std::ops::Div<Output = T>> std::ops::Div for Total<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Total(self.0 / other.0)
    }
}
impl<T: std::ops::Neg<Output = T>> std::ops::Neg for Total<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Total(-self.0)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Rev<T>(pub T);
impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}
impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}
