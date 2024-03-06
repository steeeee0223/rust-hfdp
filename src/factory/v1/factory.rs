use super::{CheesePizza, ClamPizza, Pizza};

pub struct SimplePizzaFactory;

impl SimplePizzaFactory {
    pub fn create_pizza(ty: &str) -> Box<dyn Pizza> {
        match ty {
            "cheese" => Box::new(CheesePizza::new()),
            "clam" => Box::new(ClamPizza::new()),
            _ => panic!("Unknown pizza type {ty}"),
        }
    }
}
