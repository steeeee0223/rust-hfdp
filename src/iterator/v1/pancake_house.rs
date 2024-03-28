use super::MenuItem;
use std::vec::IntoIter;

pub struct PancakeHouseMenu {
    menu_items: Vec<MenuItem>,
}

impl PancakeHouseMenu {
    pub fn new() -> Self {
        let mut menu = PancakeHouseMenu { menu_items: vec![] };
        menu.add_item(
            "Regular Pancake Breakfast",
            "Pancakes with fried eggs, sausage",
            false,
            2.99,
        );
        menu.add_item(
            "Blueberry Pancakes",
            "Pancakes made with fresh blueberries",
            true,
            3.49,
        );
        menu.add_item(
            "Waffles",
            "Waffles, with your choice of blueberries or strawberries",
            true,
            3.59,
        );
        menu
    }
    pub fn add_item(&mut self, name: &str, description: &str, vegetarian: bool, price: f32) {
        let menu_item = MenuItem::new(name, description, vegetarian, price);
        self.menu_items.push(menu_item);
    }
    pub fn create_iter(self) -> IntoIter<MenuItem> {
        self.menu_items.into_iter()
    }
}
