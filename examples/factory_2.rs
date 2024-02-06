use rust_hfdp::factory_2::{CHPizzaStore, NYPizzaStore, PizzaStore};

fn main() {
    let ny_store = NYPizzaStore;
    let ch_store = CHPizzaStore;

    let pizza = ny_store.order_pizza("cheese");
    println!("Ethan ordered a {}", pizza.name());
    println!("======");

    let pizza = ch_store.order_pizza("cheese");
    println!("Joel ordered a {}", pizza.name());
}
