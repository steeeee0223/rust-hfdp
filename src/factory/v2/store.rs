use super::{CHStyleCheesePizza, NYStyleCheesePizza, Pizza};

pub trait PizzaStore {
    fn order_pizza(&self, ty: &str) -> Box<dyn Pizza> {
        let pizza = self.create_pizza(ty);
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.pack();
        pizza
    }
    fn create_pizza(&self, ty: &str) -> Box<dyn Pizza>;
}

pub struct NYPizzaStore;
impl PizzaStore for NYPizzaStore {
    fn create_pizza(&self, ty: &str) -> Box<dyn Pizza> {
        match ty {
            "cheese" => Box::new(NYStyleCheesePizza::new()),
            _ => panic!("Unknown pizza type: {}", ty),
        }
    }
}

pub struct CHPizzaStore;
impl PizzaStore for CHPizzaStore {
    fn create_pizza(&self, ty: &str) -> Box<dyn Pizza> {
        match ty {
            "cheese" => Box::new(CHStyleCheesePizza::new()),
            _ => panic!("Unknown pizza type: {}", ty),
        }
    }
}
