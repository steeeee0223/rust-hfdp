use rust_hfdp::adapter::v1::{Duck, MallardDuck, Turkey, TurkeyAdapter, WildTurkey};

fn main() {
    let duck = MallardDuck;
    let turkey = WildTurkey;
    let turkey_adapter = TurkeyAdapter::new(Box::new(WildTurkey));

    println!("\nThe turkey says...");
    turkey.gobble();
    turkey.fly();

    println!("\nThe duck says...");
    test_duck(Box::new(duck));

    println!("\nThe fake duck (turkey) says...");
    test_duck(Box::new(turkey_adapter));
}

fn test_duck(duck: Box<dyn Duck>) {
    duck.as_ref().quack();
    duck.as_ref().fly();
}
