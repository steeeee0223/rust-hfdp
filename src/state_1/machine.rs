use std::fmt;

pub enum State {
    SOLD_OUT,
    NO_QUARTER,
    HAS_QUARTER,
    SOLD,
}

pub struct GumballMachine {
    state: State,
    count: usize,
}
impl GumballMachine {
    pub fn new(count: usize) -> Self {
        let state = if count > 0 {
            State::NO_QUARTER
        } else {
            State::SOLD_OUT
        };
        GumballMachine { count, state }
    }
    pub fn insert_quarter(&mut self) {
        match self.state {
            State::SOLD_OUT => println!("You can't insert a quarter, the machine is sold out"),
            State::NO_QUARTER => {
                self.state = State::HAS_QUARTER;
                println!("You inserted a quarter");
            }
            State::HAS_QUARTER => println!("You can't insert another quarter"),
            State::SOLD => println!("Please wait, we're already giving you a gumball"),
        }
    }
    pub fn eject_quarter(&mut self) {
        match self.state {
            State::SOLD_OUT => println!("You can't eject, you haven't inserted a quarter yet."),
            State::NO_QUARTER => println!("You haven't inserted a quarter"),
            State::HAS_QUARTER => {
                println!("Quarter returned");
                self.state = State::NO_QUARTER;
            }
            State::SOLD => println!("Sorry, you already turned the crank"),
        }
    }
    pub fn turn_crank(&mut self) {
        match self.state {
            State::SOLD_OUT => println!("You turned but there are no gumballs"),
            State::NO_QUARTER => println!("You turned but there's no quarter"),
            State::HAS_QUARTER => {
                println!("You turned");
                self.state = State::SOLD;
                self.dispense();
            }
            State::SOLD => println!("Turning twice doesn't get you another gumball!"),
        }
    }
    fn dispense(&mut self) {
        match self.state {
            State::SOLD => {
                println!("A gumball comes rolling out the slot");
                self.count -= 1;
                if self.count == 0 {
                    println!("Oops, out of gumballs!");
                    self.state = State::SOLD_OUT;
                } else {
                    self.state = State::NO_QUARTER;
                }
            }
            State::NO_QUARTER => println!("You need to pay first"),
            _ => println!("No gumball dispensed"),
        }
    }
}
impl fmt::Display for GumballMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\nMighty Gumball, Inc.\nInventory: {} gumballs\nMachine is waiting for quarter\n",
            self.count
        )
    }
}
