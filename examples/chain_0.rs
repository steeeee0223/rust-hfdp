use rust_hfdp::chain::v0::{DogHandler, Handler, MonkeyHandler, SquirrelHandler};

fn client_code(handler: &dyn Handler<String>) {
    let foods = vec!["Nut", "Banana", "Cup of coffee"];
    for food in foods {
        println!("Client: Who wants a {}?", food);
        match handler.execute(&food.to_string()) {
            Some(result) => println!("  {}", result),
            None => println!("  {} was left untouched.", food),
        }
    }
}

fn main() {
    let mut monkey = Box::new(MonkeyHandler::new());
    let mut squirrel = Box::new(SquirrelHandler::new());
    let dog = Box::new(DogHandler::new());

    squirrel.set_next(dog);
    monkey.set_next(squirrel);

    println!("Chain: Monkey > Squirrel > Dog\n");
    client_code(&*monkey);
}
