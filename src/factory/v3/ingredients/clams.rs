pub trait Clams {
    fn name(&self) -> &str;
}

pub struct FreshClams;
impl Clams for FreshClams {
    fn name(&self) -> &str {
        "Fresh Clams"
    }
}

pub struct FrozenClams;
impl Clams for FrozenClams {
    fn name(&self) -> &str {
        "Frozen Clams"
    }
}
