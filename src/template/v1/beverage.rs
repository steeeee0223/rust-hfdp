pub trait Beverage {
    fn boil_water(&self) {
        println!("Boiling water...");
    }
    fn pour_in_cup(&self) {
        println!("Pouring into cup...");
    }
    fn brew(&self);
    fn add_condiments(&self);
    fn prepare(&self) {
        self.boil_water();
        self.brew();
        self.pour_in_cup();
        self.add_condiments();
    }
}

pub struct Coffee;
impl Beverage for Coffee {
    fn brew(&self) {
        println!("Dripping coffee through filter...");
    }
    fn add_condiments(&self) {
        println!("Adding sugar & milk...");
    }
}

pub struct Tea;
impl Beverage for Tea {
    fn brew(&self) {
        println!("Steeping tea bag...");
    }
    fn add_condiments(&self) {
        println!("Adding lemon...");
    }
}
