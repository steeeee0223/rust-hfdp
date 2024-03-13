use super::DVDPlayer;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Amplifier {
    description: String,
    dvd_player: Option<Rc<RefCell<DVDPlayer>>>,
    level: i32,
}

impl Amplifier {
    pub fn new(description: &str) -> Self {
        Amplifier {
            description: description.to_owned(),
            dvd_player: None,
            level: 0,
        }
    }

    pub fn on(&self) {
        println!("{} Amplifier on", self.description);
    }

    pub fn off(&self) {
        println!("{} Amplifier off", self.description);
    }

    pub fn set_volume(&mut self, level: i32) {
        self.level = level;
        println!("{} Amplifier setting volume to {}", self.description, level);
    }

    pub fn set_dvd(&mut self, dvd_player: Rc<RefCell<DVDPlayer>>) {
        self.dvd_player = Some(dvd_player);
    }
}
