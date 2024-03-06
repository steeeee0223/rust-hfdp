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

pub struct GarageDoor;
impl GarageDoor {
    pub fn up(&self) {
        println!("Garage door is open...");
    }
    pub fn down(&self) {
        println!("Garage door is closed...");
    }
}
