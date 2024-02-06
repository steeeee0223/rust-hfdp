use crate::factory_3::ingredients::{
    BlackOlives, Eggplant, Garlic, Mushroom, Onion, RedPepper, Spinach,
};

use super::ingredients::{
    Cheese, Clams, Dough, FreshClams, FrozenClams, MarinaraSauce, MozzarellaCheese, Pepperoni,
    PlumTomatoSauce, ReggianoCheese, Sauce, SlicedPepperoni, ThinCrustDough, Veggies,
};

pub trait PizzaIngredientFactory {
    fn create_cheese(&self) -> Box<dyn Cheese>;
    fn create_clams(&self) -> Box<dyn Clams>;
    fn create_dough(&self) -> Box<dyn Dough>;
    fn create_pepperoni(&self) -> Box<dyn Pepperoni>;
    fn create_sauce(&self) -> Box<dyn Sauce>;
    fn create_veggies(&self) -> Vec<Box<dyn Veggies>>;
}

pub struct NYPizzaIngredientFactory;
impl PizzaIngredientFactory for NYPizzaIngredientFactory {
    fn create_cheese(&self) -> Box<dyn Cheese> {
        Box::new(ReggianoCheese)
    }

    fn create_clams(&self) -> Box<dyn Clams> {
        Box::new(FreshClams)
    }

    fn create_dough(&self) -> Box<dyn Dough> {
        Box::new(ThinCrustDough)
    }

    fn create_pepperoni(&self) -> Box<dyn Pepperoni> {
        Box::new(SlicedPepperoni)
    }

    fn create_sauce(&self) -> Box<dyn Sauce> {
        Box::new(MarinaraSauce)
    }

    fn create_veggies(&self) -> Vec<Box<dyn Veggies>> {
        vec![
            Box::new(Garlic),
            Box::new(Onion),
            Box::new(Mushroom),
            Box::new(RedPepper),
        ]
    }
}
pub struct CHPizzaIngredientFactory;
impl PizzaIngredientFactory for CHPizzaIngredientFactory {
    fn create_cheese(&self) -> Box<dyn Cheese> {
        Box::new(MozzarellaCheese)
    }

    fn create_clams(&self) -> Box<dyn Clams> {
        Box::new(FrozenClams)
    }

    fn create_dough(&self) -> Box<dyn Dough> {
        Box::new(ThinCrustDough)
    }

    fn create_pepperoni(&self) -> Box<dyn Pepperoni> {
        Box::new(SlicedPepperoni)
    }

    fn create_sauce(&self) -> Box<dyn Sauce> {
        Box::new(PlumTomatoSauce)
    }

    fn create_veggies(&self) -> Vec<Box<dyn Veggies>> {
        vec![Box::new(BlackOlives), Box::new(Spinach), Box::new(Eggplant)]
    }
}
