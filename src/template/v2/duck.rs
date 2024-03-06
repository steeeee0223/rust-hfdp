use std::{cmp, fmt};

#[derive(Clone, PartialEq, Eq)]
pub struct Duck {
    pub name: String,
    pub weight: usize,
}
impl Duck {
    pub fn new(name: &str, weight: usize) -> Self {
        Duck {
            name: String::from(name),
            weight,
        }
    }
}
impl fmt::Display for Duck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} weighs {}", self.name, self.weight)
    }
}
impl PartialOrd for Duck {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}
impl Ord for Duck {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
