pub struct Ingredient {
    sauce: String,
    dough: String,
    toppings: Vec<String>,
}

pub trait Pizza {
    fn name(&self) -> &str;
    fn sauce(&self) -> &str;
    fn dough(&self) -> &str;
    fn toppings(&self) -> &Vec<String>;
    fn prepare(&self) {
        println!("Preparing {}...", self.name());
        println!("- Tossing dough: {}", self.dough());
        println!("- Adding sauce: {}", self.sauce());
        println!("- Adding toppings");
        for topping in self.toppings() {
            println!("\t{}", topping);
        }
    }
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

pub struct NYStyleCheesePizza {
    name: String,
    ingredient: Box<Ingredient>,
}
impl NYStyleCheesePizza {
    pub fn new() -> Self {
        let ingredient = Box::new(Ingredient {
            dough: String::from("Thin Crust Dough"),
            sauce: String::from("Marinara Sauce"),
            toppings: vec![String::from("Grated Reggiano Cheese")],
        });
        NYStyleCheesePizza {
            name: String::from("NY Style Sauce & Cheese Pizza"),
            ingredient,
        }
    }
}
impl Pizza for NYStyleCheesePizza {
    fn name(&self) -> &str {
        &self.name
    }
    fn sauce(&self) -> &str {
        &self.ingredient.sauce
    }
    fn dough(&self) -> &str {
        &self.ingredient.dough
    }
    fn toppings(&self) -> &Vec<String> {
        &self.ingredient.toppings
    }
}

pub struct CHStyleCheesePizza {
    name: String,
    ingredient: Box<Ingredient>,
}
impl CHStyleCheesePizza {
    pub fn new() -> Self {
        let ingredient = Box::new(Ingredient {
            dough: String::from("Extra Thick Crust Dough"),
            sauce: String::from("Plum Tomato Sauce"),
            toppings: vec![String::from("Shredded Mozzarella Cheese")],
        });
        CHStyleCheesePizza {
            name: String::from("Chicago Style Deep Dish Cheese Pizza"),
            ingredient,
        }
    }
}
impl Pizza for CHStyleCheesePizza {
    fn name(&self) -> &str {
        &self.name
    }
    fn sauce(&self) -> &str {
        &self.ingredient.sauce
    }
    fn dough(&self) -> &str {
        &self.ingredient.dough
    }
    fn toppings(&self) -> &Vec<String> {
        &self.ingredient.toppings
    }
    fn cut(&self) {
        println!("Cutting the pizza into square slices");
    }
}
