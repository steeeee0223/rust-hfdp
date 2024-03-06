use rust_hfdp::strategy::v1::{Duck, FlyNoWay, MuteQuack};

fn main() {
    let mut duck = Duck::new();
    duck.perform_fly();
    duck.perform_quack();
    println!("----------");
    duck.set_fly_behavior(Box::new(FlyNoWay));
    duck.perform_fly();
    println!("----------");
    duck.set_quack_behavior(Box::new(MuteQuack));
    duck.perform_quack();
}
