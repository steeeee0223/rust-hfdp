pub trait Dough {
    fn name(&self) -> &str;
}

pub struct ThinCrustDough;
impl Dough for ThinCrustDough {
    fn name(&self) -> &str {
        "Thin Crust Dough"
    }
}
