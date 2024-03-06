use rust_hfdp::decorator::v1::{Beverage, Condiment, Espresso, HouseBlend, Mocha, Whip};

fn main() {
    let beverage = Espresso;
    println!("{} - ${}", beverage.description(), beverage.cost());

    let beverage2 = HouseBlend;
    let mut mocha = Mocha::new(Box::new(beverage2));
    mocha = Mocha::new(Box::new(mocha));
    let whip = Whip::new(Box::new(mocha));
    println!("{} - ${}", whip.description(), whip.cost());
}
