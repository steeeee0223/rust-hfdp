use std::array::IntoIter;

use super::MenuItem;

pub struct DinerMenu {
    menu_items: [MenuItem; 6],
    num_items: usize,
}
impl DinerMenu {
    pub fn new() -> Self {
        let mut menu = DinerMenu {
            menu_items: Default::default(),
            num_items: 0,
        };
        menu.add_item(
            "Vegetarian BLT",
            "(Fakin') Bacon with lettuce & tomato on whole wheat",
            true,
            2.99,
        );
        menu.add_item(
            "BLT",
            "Bacon with lettuce & tomato on whole wheat",
            false,
            2.99,
        );
        menu.add_item(
            "Soup of the day",
            "Soup of the day, with a side of potato salad",
            true,
            3.29,
        );
        menu.add_item(
            "Hotdog",
            "A hot dog, with saurkraut, relish, onions, topped with cheese",
            false,
            3.05,
        );
        menu.add_item(
            "Steamed Veggies & Brown Rice",
            "Steamed vegetables over brown rice",
            true,
            3.99,
        );
        menu.add_item(
            "Pasta",
            "Spaghetti with Mariana Sauce, and a slice of sourdough bread",
            false,
            3.89,
        );
        menu
    }
    pub fn add_item(&mut self, name: &str, description: &str, vegetarian: bool, price: f32) {
        let menu_item = MenuItem::new(name, description, vegetarian, price);
        if self.num_items >= 6 {
            println!("Menu is full!")
        } else {
            self.menu_items[self.num_items] = menu_item;
            self.num_items += 1;
        }
    }
    pub fn create_iter(self) -> IntoIter<MenuItem, 6> {
        self.menu_items.into_iter()
    }
}
