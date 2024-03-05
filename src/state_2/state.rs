use rand::Rng;

use super::GumballMachine;

pub trait State {
    fn insert_quarter(self: Box<Self>) -> Box<dyn State>;
    fn eject_quarter(self: Box<Self>) -> Box<dyn State>;
    fn turn_crank<'a>(self: Box<Self>, machine: &'a mut GumballMachine) -> Box<dyn State>;
    fn dispense<'a>(self: Box<Self>, machine: &'a mut GumballMachine) -> Box<dyn State>;
}

pub struct NoQuarterState;
impl State for NoQuarterState {
    fn insert_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("You inserted a quarter");
        Box::new(HasQuarterState)
    }
    fn eject_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("You haven't inserted a quarter");
        self
    }
    fn turn_crank<'a>(self: Box<Self>, _machine: &'a mut GumballMachine) -> Box<dyn State> {
        println!("You turned but there's no quarter");
        self
    }
    fn dispense<'a>(self: Box<Self>, _machine: &'a mut GumballMachine) -> Box<dyn State> {
        println!("You need to pay first");
        self
    }
}

pub struct HasQuarterState;
impl State for HasQuarterState {
    fn insert_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("You can't insert another quarter");
        self
    }
    fn eject_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("Quarter returned");
        Box::new(NoQuarterState)
    }
    fn turn_crank<'a>(self: Box<Self>, machine: &'a mut GumballMachine) -> Box<dyn State> {
        println!("You turned...");

        let mut rng = rand::thread_rng();
        let winner = rng.gen_range(0..10);
        if winner == 0 && machine.count() > 1 {
            Box::new(WinnerState)
        } else {
            Box::new(SoldState)
        }
    }
    fn dispense<'a>(self: Box<Self>, _machine: &'a mut GumballMachine) -> Box<dyn State> {
        println!("No gumball dispensed");
        self
    }
}

pub struct SoldState;
impl State for SoldState {
    fn insert_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("You can't insert a quarter, the machine is sold out");
        self
    }
    fn eject_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("You can't eject, you haven't inserted a quarter yet.");
        self
    }
    fn turn_crank<'a>(self: Box<Self>, _machine: &'a mut GumballMachine) -> Box<dyn State> {
        println!("You turned but there are no gumballs");
        self
    }
    fn dispense<'a>(self: Box<Self>, machine: &'a mut GumballMachine) -> Box<dyn State> {
        machine.release_ball();
        if machine.count() > 0 {
            println!("Released...");
            Box::new(NoQuarterState)
        } else {
            println!("Oops, out of gumballs!");
            Box::new(SoldOutState)
        }
    }
}

pub struct SoldOutState;
impl State for SoldOutState {
    fn insert_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("You can't insert a quarter, the machine is sold out");
        self
    }
    fn eject_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("You haven't inserted a quarter");
        self
    }
    fn turn_crank<'a>(self: Box<Self>, _machine: &'a mut GumballMachine) -> Box<dyn State> {
        println!("You turned but there are no gumballs");
        self
    }
    fn dispense<'a>(self: Box<Self>, _machine: &'a mut GumballMachine) -> Box<dyn State> {
        println!("No gumball dispensed");
        self
    }
}

pub struct WinnerState;
impl State for WinnerState {
    fn insert_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("You can't insert a quarter, the machine is sold out");
        self
    }
    fn eject_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("You can't eject, you haven't inserted a quarter yet.");
        self
    }
    fn turn_crank<'a>(self: Box<Self>, _machine: &'a mut GumballMachine) -> Box<dyn State> {
        println!("You turned but there are no gumballs");
        self
    }
    fn dispense<'a>(self: Box<Self>, machine: &'a mut GumballMachine) -> Box<dyn State> {
        println!("YOU'RE A WINNER! You get 2 gumballs for your quarter");
        let mut i = 2;
        while i > 0 {
            machine.release_ball();
            if machine.count() == 0 {
                return Box::new(SoldOutState);
            }
            i -= 1;
        }
        Box::new(NoQuarterState)
    }
}
