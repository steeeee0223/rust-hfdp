use std::fmt;

use super::MenuComponent;

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
}
impl fmt::Display for MenuItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n{}{}, {} -- {}",
            self.name(),
            if self.is_vegetarian() { "(v)" } else { "" },
            self.price(),
            self.description()
        )
    }
}
impl MenuComponent for MenuItem {
    fn name(&self) -> String {
        self.name.to_owned()
    }
    fn description(&self) -> String {
        self.description.to_owned()
    }
    fn is_vegetarian(&self) -> bool {
        self.vegetarian
    }
    fn price(&self) -> f32 {
        self.price
    }
    fn add(&mut self, _menu: Box<dyn MenuComponent>) {
        unimplemented!("Unsupported method")
    }
    fn remove(&mut self, _menu: Box<dyn MenuComponent>) {
        unimplemented!("Unsupported method")
    }
    fn child(&self, _i: usize) -> &Box<dyn MenuComponent> {
        unimplemented!("Unsupported method")
    }
}
