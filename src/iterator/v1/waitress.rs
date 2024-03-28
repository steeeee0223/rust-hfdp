use super::{CafeMenu, DinerMenu, MenuItem, PancakeHouseMenu};

pub struct Waitress {
    pancake_house_menu: PancakeHouseMenu,
    diner_menu: DinerMenu,
    cafe_menu: CafeMenu,
}
impl Waitress {
    pub fn new(
        pancake_house_menu: PancakeHouseMenu,
        diner_menu: DinerMenu,
        cafe_menu: CafeMenu,
    ) -> Self {
        Waitress {
            pancake_house_menu,
            diner_menu,
            cafe_menu,
        }
    }
    pub fn print_menu(self) {
        let pancake_iter = self.pancake_house_menu.create_iter();
        let diner_iter = self.diner_menu.create_iter();
        let cafe_iter = self.cafe_menu.create_iter();

        println!("MENU\n---\nBREAKFAST");
        Waitress::print_one(pancake_iter);

        println!("\nLUNCH");
        Waitress::print_one(diner_iter);

        println!("\nDINNER");
        Waitress::print_one(cafe_iter);
    }
    fn print_one(mut iter: impl Iterator<Item = MenuItem>) {
        while let Some(item) = iter.next() {
            println!(
                "{}, ${} -- {}",
                item.name(),
                item.price(),
                item.description()
            );
        }
    }
}
