pub trait Sauce {
    fn name(&self) -> &str;
}

pub struct MarinaraSauce;
impl Sauce for MarinaraSauce {
    fn name(&self) -> &str {
        "Marinara Sauce"
    }
}

pub struct PlumTomatoSauce;
impl Sauce for PlumTomatoSauce {
    fn name(&self) -> &str {
        "Plum Tomato Sauce"
    }
}
