pub trait Pepperoni {
    fn name(&self) -> &str;
}

pub struct SlicedPepperoni;
impl Pepperoni for SlicedPepperoni {
    fn name(&self) -> &str {
        "Sliced Pepperoni"
    }
}
