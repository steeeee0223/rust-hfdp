use std::collections::{hash_map::IntoValues, HashMap};

use super::MenuItem;

pub struct CafeMenu {
    menu_items: HashMap<String, MenuItem>,
}
impl CafeMenu {
    pub fn new() -> Self {
        let mut menu = CafeMenu {
            menu_items: HashMap::new(),
        };
        menu.add_item(
            "Veggie Burger & Air Fries",
            "Veggie burger on a whole wheat bun, lettuce, tomato, and fries",
            true,
            3.99,
        );
        menu.add_item(
            "Soup of the day",
            "A cup of the soup of the day, with a side salad",
            false,
            3.69,
        );
        menu.add_item(
            "Burrito",
            "A large burrito, with whole pinto beans, salsa, guacamole",
            true,
            4.29,
        );
        menu
    }
    pub fn add_item(&mut self, name: &str, description: &str, vegetarian: bool, price: f32) {
        let menu_item = MenuItem::new(name, description, vegetarian, price);
        self.menu_items.insert(name.to_owned(), menu_item);
    }
    pub fn create_iter(self) -> IntoValues<String, MenuItem> {
        self.menu_items.into_values()
    }
}
