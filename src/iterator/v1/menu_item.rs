#[derive(Default)]
pub struct MenuItem {
    name: String,
    description: String,
    vegetarian: bool,
    price: f32,
}

impl MenuItem {
    pub fn new(name: &str, description: &str, vegetarian: bool, price: f32) -> Self {
        MenuItem {
            name: name.to_owned(),
            description: description.to_owned(),
            vegetarian,
            price,
        }
    }
    pub fn name(&self) -> String {
        self.name.to_owned()
    }
    pub fn description(&self) -> String {
        self.description.to_owned()
    }
    pub fn is_vegetarian(&self) -> bool {
        self.vegetarian
    }
    pub fn price(&self) -> f32 {
        self.price
    }
}
