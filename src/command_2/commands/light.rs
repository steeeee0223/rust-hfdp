use std::{cell::RefCell, rc::Rc};

use super::Command;

pub struct Light {
    location: String,
}
impl Light {
    pub fn new(location: &str) -> Self {
        Light {
            location: String::from(location),
        }
    }
    pub fn on(&self) {
        println!("{} light is on...", self.location);
    }
    pub fn off(&self) {
        println!("{} light is off...", self.location);
    }
}

pub struct LightOnCommand {
    light: Rc<RefCell<Light>>,
}
impl LightOnCommand {
    pub fn new(light: Rc<RefCell<Light>>) -> LightOnCommand {
        LightOnCommand { light }
    }
}
impl Command for LightOnCommand {
    fn name(&self) -> String {
        format!("{} Light On", self.light.borrow().location)
    }
    fn execute(&self) {
        self.light.borrow().on();
    }
}

pub struct LightOffCommand {
    light: Rc<RefCell<Light>>,
}
impl LightOffCommand {
    pub fn new(light: Rc<RefCell<Light>>) -> LightOffCommand {
        LightOffCommand { light }
    }
}
impl Command for LightOffCommand {
    fn name(&self) -> String {
        format!("{} Light Off", self.light.borrow().location)
    }
    fn execute(&self) {
        self.light.borrow().off();
    }
}
