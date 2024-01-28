use super::beverages::Beverage;

pub trait Condiment: Beverage {
    fn new(beverage: Box<dyn Beverage>) -> Self;
}

pub struct Mocha {
    beverage: Box<dyn Beverage>,
}
impl Condiment for Mocha {
    fn new(beverage: Box<dyn Beverage>) -> Mocha {
        Mocha { beverage }
    }
}
impl Beverage for Mocha {
    fn cost(&self) -> f32 {
        self.beverage.cost() + 0.2
    }
    fn description(&self) -> String {
        self.beverage.description() + ", Mocha"
    }
}

pub struct Whip {
    beverage: Box<dyn Beverage>,
}
impl Condiment for Whip {
    fn new(beverage: Box<dyn Beverage>) -> Whip {
        Whip { beverage }
    }
}
impl Beverage for Whip {
    fn cost(&self) -> f32 {
        self.beverage.cost() + 0.1
    }
    fn description(&self) -> String {
        self.beverage.description() + ", Whip"
    }
}
