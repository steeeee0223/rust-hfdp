pub trait Beverage {
    fn cost(&self) -> f32;
    fn description(&self) -> String;
}

pub struct Espresso;
impl Beverage for Espresso {
    fn cost(&self) -> f32 {
        1.99
    }
    fn description(&self) -> String {
        String::from("Espresso")
    }
}

pub struct HouseBlend;
impl Beverage for HouseBlend {
    fn cost(&self) -> f32 {
        0.89
    }
    fn description(&self) -> String {
        String::from("House Blend Coffee")
    }
}
