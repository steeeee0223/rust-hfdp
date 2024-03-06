use super::{Pizza, SimplePizzaFactory};

pub struct PizzaStore;

impl PizzaStore {
    pub fn order_pizza(&self, ty: &str) -> Box<dyn Pizza> {
        let pizza = SimplePizzaFactory::create_pizza(ty);
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.pack();
        pizza
    }
}
