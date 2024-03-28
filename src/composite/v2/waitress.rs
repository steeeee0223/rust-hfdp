use super::Menu;

pub struct Waitress {
    all_menus: Menu,
}
impl Waitress {
    pub fn new(all_menus: Menu) -> Self {
        Waitress { all_menus }
    }
    pub fn print_menu(&self) {
        println!("{}", self.all_menus);
    }
}
