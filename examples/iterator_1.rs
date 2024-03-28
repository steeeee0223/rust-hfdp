use rust_hfdp::iterator::v1::{CafeMenu, DinerMenu, PancakeHouseMenu, Waitress};

fn main() {
    let pancake_house_menu = PancakeHouseMenu::new();
    let diner_menu = DinerMenu::new();
    let cafe_menu = CafeMenu::new();

    let waitress = Waitress::new(pancake_house_menu, diner_menu, cafe_menu);
    waitress.print_menu();
}
