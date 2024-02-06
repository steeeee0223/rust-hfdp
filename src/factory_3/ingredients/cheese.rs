pub trait Cheese {
    fn name(&self) -> &str;
}

pub struct ReggianoCheese;
impl Cheese for ReggianoCheese {
    fn name(&self) -> &str {
        "Reggiano Cheese"
    }
}

pub struct MozzarellaCheese;
impl Cheese for MozzarellaCheese {
    fn name(&self) -> &str {
        "Mozzarella Cheese"
    }
}
