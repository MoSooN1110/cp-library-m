// source snippet: key=lib_ratio  prefix=lib_ratio

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
#[derive(Clone, Copy, Debug)]
struct Ratio {
    numer: u64,
    denom: u64,
}

impl Ratio {
    fn new(numer: u64, denom: u64) -> Ratio {
        let g = gcd(numer, denom);
        Ratio {
            numer: numer / g,
            denom: denom / g,
        }
    }
}

impl PartialEq for Ratio {
    fn eq(&self, other: &Self) -> bool {
        (self.numer * other.denom).eq(&(self.denom * other.numer))
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.numer * other.denom).partial_cmp(&(self.denom * other.numer))
    }
}

impl Eq for Ratio {}

impl Ord for Ratio {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
