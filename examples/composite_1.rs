use rust_hfdp::composite::v1::{Menu, MenuComponent, MenuItem, Waitress};

fn main() {
    let mut pancake_house_menu = Menu::new("PANCAKE HOUSE MENU", "Breakfast");
    let mut diner_menu = Menu::new("DINER MENU", "Lunch");
    let mut cafe_menu = Menu::new("CAFE MENU", "Dinner");
    let mut dessert_menu = Menu::new("DESSERT MENU", "Dessert of course!");
    let mut all_menus = Menu::new("ALL MENUS", "All menus combined");
    // Pancake House Menu
    pancake_house_menu.add(Box::new(MenuItem::new(
        "Regular Pancake Breakfast",
        "Pancakes with fried eggs, sausage",
        false,
        2.99,
    )));
    pancake_house_menu.add(Box::new(MenuItem::new(
        "Blueberry Pancakes",
        "Pancakes made with fresh blueberries",
        true,
        3.49,
    )));
    pancake_house_menu.add(Box::new(MenuItem::new(
        "Waffles",
        "Waffles, with your choice of blueberries or strawberries",
        true,
        3.59,
    )));
    // Cafe Menu
    cafe_menu.add(Box::new(MenuItem::new(
        "Veggie Burger & Air Fries",
        "Veggie burger on a whole wheat bun, lettuce, tomato, and fries",
        true,
        3.99,
    )));
    cafe_menu.add(Box::new(MenuItem::new(
        "Soup of the day",
        "A cup of the soup of the day, with a side salad",
        false,
        3.69,
    )));
    cafe_menu.add(Box::new(MenuItem::new(
        "Burrito",
        "A large burrito, with whole pinto beans, salsa, guacamole",
        true,
        4.29,
    )));
    // Diner Menu
    diner_menu.add(Box::new(MenuItem::new(
        "Vegetarian BLT",
        "(Fakin') Bacon with lettuce & tomato on whole wheat",
        true,
        2.99,
    )));
    diner_menu.add(Box::new(MenuItem::new(
        "BLT",
        "Bacon with lettuce & tomato on whole wheat",
        false,
        2.99,
    )));
    diner_menu.add(Box::new(MenuItem::new(
        "Soup of the day",
        "Soup of the day, with a side of potato salad",
        true,
        3.29,
    )));
    diner_menu.add(Box::new(MenuItem::new(
        "Hotdog",
        "A hot dog, with saurkraut, relish, onions, topped with cheese",
        false,
        3.05,
    )));
    diner_menu.add(Box::new(MenuItem::new(
        "Steamed Veggies & Brown Rice",
        "Steamed vegetables over brown rice",
        true,
        3.99,
    )));
    diner_menu.add(Box::new(MenuItem::new(
        "Pasta",
        "Spaghetti with Mariana Sauce, and a slice of sourdough bread",
        false,
        3.89,
    )));
    // Dessert Menu
    dessert_menu.add(Box::new(MenuItem::new(
        "Apple Pie",
        "Apple pie with a flakey crust, topped with vanilla ice cream",
        true,
        1.59,
    )));
    // Add to parent menus
    diner_menu.add(Box::new(dessert_menu));
    all_menus.add(Box::new(pancake_house_menu));
    all_menus.add(Box::new(diner_menu));
    all_menus.add(Box::new(cafe_menu));
    // Waitress Display
    let waitress = Waitress::new(Box::new(all_menus));
    waitress.print_menu();
}
