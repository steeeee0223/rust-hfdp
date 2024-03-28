use rust_hfdp::composite::v2::{Menu, MenuComponent, Waitress};

fn main() {
    let mut pancake_house_menu = Menu::Menu(
        "PANCAKE HOUSE MENU".to_owned(),
        "Breakfast".to_owned(),
        vec![],
    );
    let mut diner_menu = Menu::Menu("DINER MENU".to_owned(), "Lunch".to_owned(), vec![]);
    let mut cafe_menu = Menu::Menu("CAFE MENU".to_owned(), "Dinner".to_owned(), vec![]);
    let mut dessert_menu = Menu::Menu(
        "DESSERT MENU".to_owned(),
        "Dessert of course!".to_owned(),
        vec![],
    );
    let mut all_menus = Menu::Menu(
        "ALL MENUS".to_owned(),
        "All menus combined".to_owned(),
        vec![],
    );
    // Pancake House Menu
    pancake_house_menu.add(Menu::MenuItem(
        "Regular Pancake Breakfast".to_owned(),
        "Pancakes with fried eggs, sausage".to_owned(),
        false,
        2.99,
    ));
    pancake_house_menu.add(Menu::MenuItem(
        "Blueberry Pancakes".to_owned(),
        "Pancakes made with fresh blueberries".to_owned(),
        true,
        3.49,
    ));
    pancake_house_menu.add(Menu::MenuItem(
        "Waffles".to_owned(),
        "Waffles, with your choice of blueberries or strawberries".to_owned(),
        true,
        3.59,
    ));
    // Cafe Menu
    cafe_menu.add(Menu::MenuItem(
        "Veggie Burger & Air Fries".to_owned(),
        "Veggie burger on a whole wheat bun, lettuce, tomato, and fries".to_owned(),
        true,
        3.99,
    ));
    cafe_menu.add(Menu::MenuItem(
        "Soup of the day".to_owned(),
        "A cup of the soup of the day, with a side salad".to_owned(),
        false,
        3.69,
    ));
    cafe_menu.add(Menu::MenuItem(
        "Burrito".to_owned(),
        "A large burrito, with whole pinto beans, salsa, guacamole".to_owned(),
        true,
        4.29,
    ));
    // Diner Menu
    diner_menu.add(Menu::MenuItem(
        "Vegetarian BLT".to_owned(),
        "(Fakin') Bacon with lettuce & tomato on whole wheat".to_owned(),
        true,
        2.99,
    ));
    diner_menu.add(Menu::MenuItem(
        "BLT".to_owned(),
        "Bacon with lettuce & tomato on whole wheat".to_owned(),
        false,
        2.99,
    ));
    diner_menu.add(Menu::MenuItem(
        "Soup of the day".to_owned(),
        "Soup of the day, with a side of potato salad".to_owned(),
        true,
        3.29,
    ));
    diner_menu.add(Menu::MenuItem(
        "Hotdog".to_owned(),
        "A hot dog, with saurkraut, relish, onions, topped with cheese".to_owned(),
        false,
        3.05,
    ));
    diner_menu.add(Menu::MenuItem(
        "Steamed Veggies & Brown Rice".to_owned(),
        "Steamed vegetables over brown rice".to_owned(),
        true,
        3.99,
    ));
    diner_menu.add(Menu::MenuItem(
        "Pasta".to_owned(),
        "Spaghetti with Mariana Sauce, and a slice of sourdough bread".to_owned(),
        false,
        3.89,
    ));
    // Dessert Menu
    dessert_menu.add(Menu::MenuItem(
        "Apple Pie".to_owned(),
        "Apple pie with a flakey crust, topped with vanilla ice cream".to_owned(),
        true,
        1.59,
    ));
    // Add to parent menus
    diner_menu.add(dessert_menu);
    all_menus.add(pancake_house_menu);
    all_menus.add(diner_menu);
    all_menus.add(cafe_menu);
    // Waitress Display
    let waitress = Waitress::new(all_menus);
    waitress.print_menu();
}
