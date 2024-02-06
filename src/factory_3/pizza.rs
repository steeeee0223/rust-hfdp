use std::fmt;

use super::{
    ingredients::{Cheese, Clams, Dough, Sauce},
    PizzaIngredientFactory,
};
pub trait Pizza: fmt::Display {
    fn name(&self) -> &str;
    fn prepare(&mut self);
    fn bake(&self) {
        println!("Baking for 25 minutes at 350ºC...");
    }
    fn cut(&self) {
        println!("Cutting the pizza into diagonal slices...");
    }
    fn pack(&self) {
        println!("Placing pizza in official PizzaStore box...");
    }
}

pub struct CheesePizza {
    name: String,
    dough: Option<Box<dyn Dough>>,
    sauce: Option<Box<dyn Sauce>>,
    cheese: Option<Box<dyn Cheese>>,
    factory: Box<dyn PizzaIngredientFactory>,
}
impl CheesePizza {
    pub fn new(factory: Box<dyn PizzaIngredientFactory>, name: &str) -> Self {
        CheesePizza {
            name: String::from(name),
            factory,
            dough: None,
            sauce: None,
            cheese: None,
        }
    }
}
impl Pizza for CheesePizza {
    fn name(&self) -> &str {
        &self.name
    }
    fn prepare(&mut self) {
        println!("Preparing {}...", self.name);
        self.dough = Some(self.factory.create_dough());
        self.sauce = Some(self.factory.create_sauce());
        self.cheese = Some(self.factory.create_cheese());
    }
}
impl fmt::Display for CheesePizza {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n- dough: {}\n- sauce: {}\n- cheese: {}",
            self.name,
            self.dough.as_ref().unwrap().name(),
            self.sauce.as_ref().unwrap().name(),
            self.cheese.as_ref().unwrap().name(),
        )
    }
}
pub struct ClamPizza {
    name: String,
    dough: Option<Box<dyn Dough>>,
    sauce: Option<Box<dyn Sauce>>,
    cheese: Option<Box<dyn Cheese>>,
    clams: Option<Box<dyn Clams>>,
    factory: Box<dyn PizzaIngredientFactory>,
}
impl ClamPizza {
    pub fn new(factory: Box<dyn PizzaIngredientFactory>, name: &str) -> Self {
        ClamPizza {
            name: String::from(name),
            factory,
            dough: None,
            sauce: None,
            cheese: None,
            clams: None,
        }
    }
}
impl Pizza for ClamPizza {
    fn name(&self) -> &str {
        &self.name
    }
    fn prepare(&mut self) {
        println!("Preparing {}...", self.name);
        self.dough = Some(self.factory.create_dough());
        self.sauce = Some(self.factory.create_sauce());
        self.cheese = Some(self.factory.create_cheese());
        self.clams = Some(self.factory.create_clams());
    }
}
impl fmt::Display for ClamPizza {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n- dough: {}\n- sauce: {}\n- cheese: {}\n- clams: {}",
            self.name,
            self.dough.as_ref().unwrap().name(),
            self.sauce.as_ref().unwrap().name(),
            self.cheese.as_ref().unwrap().name(),
            self.clams.as_ref().unwrap().name(),
        )
    }
}
