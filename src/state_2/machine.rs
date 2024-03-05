use super::{NoQuarterState, SoldOutState, State};
use std::fmt;

pub struct GumballMachine {
    state: Option<Box<dyn State>>,
    count: usize,
}
impl GumballMachine {
    pub fn new(count: usize) -> Self {
        let state: Option<Box<dyn State>> = if count > 0 {
            Some(Box::new(NoQuarterState))
        } else {
            Some(Box::new(SoldOutState))
        };
        GumballMachine { count, state }
    }
    pub fn count(&self) -> usize {
        self.count
    }
    pub fn insert_quarter(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.insert_quarter());
        }
    }
    pub fn eject_quarter(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.eject_quarter());
        }
    }
    pub fn turn_crank(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.turn_crank(self));
        }
        if let Some(state) = self.state.take() {
            self.state = Some(state.dispense(self));
        }
    }
    pub fn release_ball(&mut self) {
        println!("A gumball comes rolling out the slot...");
        if self.count > 0 {
            self.count -= 1
        }
    }
}
impl fmt::Display for GumballMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\nMighty Gumball v2.0, Inc.\nInventory: {} gumballs\nMachine is waiting for quarter\n",
            self.count
        )
    }
}
