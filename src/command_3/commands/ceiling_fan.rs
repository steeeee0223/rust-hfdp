use std::{cell::RefCell, rc::Rc};

use super::Command;

pub enum Speed {
    HIGH,
    MEDIUM,
    LOW,
    OFF,
}

pub struct CeilingFan {
    location: String,
    speed: Speed,
}
impl CeilingFan {
    pub fn new(location: &str) -> Self {
        CeilingFan {
            location: String::from(location),
            speed: Speed::OFF,
        }
    }
    pub fn high(&mut self) {
        self.speed = Speed::HIGH;
        println!("Speed of {} ceiling fan is 3...", self.location);
    }
    pub fn medium(&mut self) {
        self.speed = Speed::MEDIUM;
        println!("Speed of {} ceiling fan is 2...", self.location);
    }
    pub fn low(&mut self) {
        self.speed = Speed::LOW;
        println!("Speed of {} ceiling fan is 1...", self.location);
    }
    pub fn off(&mut self) {
        self.speed = Speed::OFF;
        println!("Speed of {} ceiling fan is off...", self.location);
    }
    pub fn speed(&self) -> u8 {
        match self.speed {
            Speed::HIGH => 3,
            Speed::MEDIUM => 2,
            Speed::LOW => 1,
            Speed::OFF => 0,
        }
    }
}

pub struct CeilingFanOnCommand {
    ceiling_fan: Rc<RefCell<CeilingFan>>,
}
impl CeilingFanOnCommand {
    pub fn new(ceiling_fan: Rc<RefCell<CeilingFan>>) -> Self {
        CeilingFanOnCommand { ceiling_fan }
    }
}
impl Command for CeilingFanOnCommand {
    fn name(&self) -> String {
        format!("{} Ceiling Fan On", self.ceiling_fan.borrow().location)
    }
    fn execute(&mut self) {
        self.ceiling_fan.borrow_mut().high();
        self.ceiling_fan.borrow().speed();
    }
    fn undo(&mut self) {
        self.ceiling_fan.borrow_mut().off();
        self.ceiling_fan.borrow().speed();
    }
}

pub struct CeilingFanOffCommand {
    ceiling_fan: Rc<RefCell<CeilingFan>>,
    history: u8,
}
impl CeilingFanOffCommand {
    pub fn new(ceiling_fan: Rc<RefCell<CeilingFan>>) -> Self {
        let speed = ceiling_fan.borrow().speed();
        CeilingFanOffCommand {
            ceiling_fan,
            history: speed,
        }
    }
}
impl Command for CeilingFanOffCommand {
    fn name(&self) -> String {
        format!("{} Ceiling Fan Off", self.ceiling_fan.borrow().location)
    }
    fn execute(&mut self) {
        self.history = self.ceiling_fan.borrow().speed();
        self.ceiling_fan.borrow_mut().off();
        self.ceiling_fan.borrow().speed();
    }
    fn undo(&mut self) {
        match self.history {
            3 => self.ceiling_fan.borrow_mut().high(),
            2 => self.ceiling_fan.borrow_mut().medium(),
            1 => self.ceiling_fan.borrow_mut().low(),
            0 => self.ceiling_fan.borrow_mut().off(),
            other => panic!("Error speed: {}", other),
        }
        self.ceiling_fan.borrow().speed();
    }
}
