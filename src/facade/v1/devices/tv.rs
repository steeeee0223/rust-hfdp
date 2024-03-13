pub struct TV {
    description: String,
}

impl TV {
    pub fn new(description: &str) -> Self {
        TV {
            description: description.to_owned(),
        }
    }
    pub fn on(&self) {
        println!("{} TV on", self.description);
    }
    pub fn off(&self) {
        println!("{} TV off", self.description);
    }
}
