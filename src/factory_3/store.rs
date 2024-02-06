use super::{CHPizzaIngredientFactory, CheesePizza, ClamPizza, NYPizzaIngredientFactory, Pizza};

pub trait PizzaStore {
    fn order_pizza(&self, ty: &str) -> Box<dyn Pizza> {
        let mut pizza = self.create_pizza(ty);
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
        let factory = Box::new(NYPizzaIngredientFactory);
        match ty {
            "cheese" => Box::new(CheesePizza::new(factory, "New York Style Cheese Pizza")),
            "clam" => Box::new(ClamPizza::new(factory, "New York Style Clam Pizza")),
            _ => panic!("Unknown pizza type: {}", ty),
        }
    }
}

pub struct CHPizzaStore;
impl PizzaStore for CHPizzaStore {
    fn create_pizza(&self, ty: &str) -> Box<dyn Pizza> {
        let factory = Box::new(CHPizzaIngredientFactory);
        match ty {
            "cheese" => Box::new(CheesePizza::new(factory, "Chicago Style Cheese Pizza")),
            "clam" => Box::new(ClamPizza::new(factory, "Chicago Style Clam Pizza")),
            _ => panic!("Unknown pizza type: {}", ty),
        }
    }
}
