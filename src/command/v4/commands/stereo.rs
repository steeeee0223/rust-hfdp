use std::{cell::RefCell, rc::Rc};

use super::Command;

pub struct Stereo {
    location: String,
}
impl Stereo {
    pub fn new(location: &str) -> Self {
        Stereo {
            location: String::from(location),
        }
    }
    pub fn on(&self) {
        println!("{} stereo is on...", self.location);
    }
    pub fn off(&self) {
        println!("{} stereo is off...", self.location);
    }
    pub fn set_cd(&self) {
        println!("Set stereo CD...");
    }
    pub fn set_dvd(&self) {
        println!("Set stereo DVD...");
    }
    pub fn set_radio(&self) {
        println!("Set stereo radio...");
    }
    pub fn set_volume(&self, vol: i32) {
        println!("Set stereo volume to {}...", vol);
    }
}

pub struct StereoOnWithCDCommand {
    stereo: Rc<RefCell<Stereo>>,
}
impl StereoOnWithCDCommand {
    pub fn new(stereo: Rc<RefCell<Stereo>>) -> Self {
        StereoOnWithCDCommand { stereo }
    }
}
impl Command for StereoOnWithCDCommand {
    fn name(&self) -> String {
        format!("{} Stereo On with CD", self.stereo.borrow().location)
    }
    fn execute(&mut self) {
        self.stereo.borrow().on();
        self.stereo.borrow().set_cd();
        self.stereo.borrow().set_volume(11);
    }
    fn undo(&mut self) {
        self.stereo.borrow().off();
    }
}

pub struct StereoOffCommand {
    stereo: Rc<RefCell<Stereo>>,
}
impl StereoOffCommand {
    pub fn new(stereo: Rc<RefCell<Stereo>>) -> Self {
        StereoOffCommand { stereo }
    }
}
impl Command for StereoOffCommand {
    fn name(&self) -> String {
        format!("{} Stereo Off", self.stereo.borrow().location)
    }
    fn execute(&mut self) {
        self.stereo.borrow().off();
    }
    fn undo(&mut self) {
        self.stereo.borrow().on();
        self.stereo.borrow().set_cd();
        self.stereo.borrow().set_volume(11);
    }
}
