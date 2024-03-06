use rust_hfdp::strategy::v2::behaviors::{FlyWithWings, MuteQuack, Quack};
use rust_hfdp::strategy::v2::ducks::{Duck, MallardDuck};

fn main() {
    let duck = MallardDuck;
    duck.swim();
    duck.display();
    duck.perform_fly(&FlyWithWings);
    duck.perform_quack(&Quack);
    duck.perform_quack(&MuteQuack);
}
