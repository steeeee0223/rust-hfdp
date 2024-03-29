use super::MenuComponent;

pub struct Waitress {
    all_menus: Box<dyn MenuComponent>,
}
impl Waitress {
    pub fn new(all_menus: Box<dyn MenuComponent>) -> Self {
        Waitress { all_menus }
    }
    pub fn print_menu(&self) {
        println!("{}", self.all_menus);
    }
}
