use super::behaviors::{FlyBehavior, QuackBehavior};

pub trait Duck {
    fn display(&self);
    fn perform_fly(&self, fly_behavior: &impl FlyBehavior) {
        fly_behavior.fly();
    }
    fn perform_quack(&self, quack_behavior: &impl QuackBehavior) {
        quack_behavior.quack();
    }
    fn swim(&self) {
        println!("All ducks float, even decoys!");
    }
}

pub struct MallardDuck;
impl Duck for MallardDuck {
    fn display(&self) {
        println!("I'm a real Mallard duck");
    }
}
pub struct ReadheadDuck;
impl Duck for ReadheadDuck {
    fn display(&self) {
        println!("I'm a real Readhead duck");
    }
}
pub struct RubberDuck;
impl Duck for RubberDuck {
    fn display(&self) {
        println!("I'm a rubber duck");
    }
}

pub struct DecoyDuck;
impl Duck for DecoyDuck {
    fn display(&self) {
        println!("I'm a decoy duck");
    }
}
