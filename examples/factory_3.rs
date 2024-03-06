use rust_hfdp::factory::v3::{CHPizzaStore, NYPizzaStore, PizzaStore};

fn main() {
    let ny_store = NYPizzaStore;
    let chicago_store = CHPizzaStore;

    let pizza = ny_store.order_pizza("cheese");
    println!("Ethan ordered a {}", pizza);
    println!("------------");
    let pizza = chicago_store.order_pizza("cheese");
    println!("Joel ordered a {}", pizza);
    println!("------------");
    let pizza = ny_store.order_pizza("clam");
    println!("Ethan ordered a {}", pizza);
    println!("------------");
    let pizza = chicago_store.order_pizza("clam");
    println!("Joel ordered a {}", pizza);
}
