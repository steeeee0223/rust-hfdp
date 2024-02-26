use rust_hfdp::template_1::{Beverage, BeverageWithHook as _, CoffeeWithHook, Tea};

fn main() {
    println!("Making tea...");

    let tea = Tea;
    tea.prepare();

    println!("\nMaking coffee...");
    let coffee = CoffeeWithHook;
    coffee.prepare();
}
