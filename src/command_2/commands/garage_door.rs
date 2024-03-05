use std::{cell::RefCell, rc::Rc};

use super::Command;

pub struct GarageDoor;
impl GarageDoor {
    pub fn up(&self) {
        println!("Garage door is open...");
    }
    pub fn down(&self) {
        println!("Garage door is closed...");
    }
}

pub struct GarageDoorOpenCommand {
    garage_door: Rc<RefCell<GarageDoor>>,
}
impl GarageDoorOpenCommand {
    pub fn new(garage_door: Rc<RefCell<GarageDoor>>) -> Self {
        GarageDoorOpenCommand { garage_door }
    }
}
impl Command for GarageDoorOpenCommand {
    fn name(&self) -> String {
        String::from("Garage Door Open")
    }
    fn execute(&self) {
        self.garage_door.borrow().up();
    }
}

pub struct GarageDoorCloseCommand {
    garage_door: Rc<RefCell<GarageDoor>>,
}
impl GarageDoorCloseCommand {
    pub fn new(garage_door: Rc<RefCell<GarageDoor>>) -> Self {
        GarageDoorCloseCommand { garage_door }
    }
}
impl Command for GarageDoorCloseCommand {
    fn name(&self) -> String {
        String::from("Garage Door Close")
    }
    fn execute(&self) {
        self.garage_door.borrow().down();
    }
}
