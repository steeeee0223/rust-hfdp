use std::io;

pub trait BeverageWithHook {
    fn boil_water(&self) {
        println!("Boiling water...");
    }
    fn pour_in_cup(&self) {
        println!("Pouring into cup...");
    }
    fn brew(&self);
    fn add_condiments(&self);
    fn should_add_condiments(&self) -> bool {
        true
    }
    fn prepare(&self) {
        self.boil_water();
        self.brew();
        self.pour_in_cup();
        if self.should_add_condiments() {
            self.add_condiments();
        }
    }
}

enum Input {
    YES,
    NO,
}

pub struct CoffeeWithHook;
impl CoffeeWithHook {
    fn user_input(&self) -> Input {
        let mut input = String::new();

        println!("Would you like milk and sugar with your coffee (y/n)? ");
        io::stdin()
            .read_line(&mut input)
            .expect("Error whiling retrieving input");

        match input.trim().to_lowercase().as_str() {
            "y" => Input::YES,
            "yes" => Input::YES,
            _ => Input::NO,
        }
    }
}
impl BeverageWithHook for CoffeeWithHook {
    fn brew(&self) {
        println!("Dripping coffee through filter...");
    }
    fn add_condiments(&self) {
        println!("Adding sugar & milk...");
    }
    fn should_add_condiments(&self) -> bool {
        match self.user_input() {
            Input::YES => true,
            Input::NO => false,
        }
    }
}
