use super::{GarageDoor, Light};

pub trait Command {
    fn execute(&self);
}

pub struct LightOnCommand {
    light: Box<Light>,
}
impl LightOnCommand {
    pub fn new(light: Light) -> LightOnCommand {
        LightOnCommand {
            light: Box::new(light),
        }
    }
}
impl Command for LightOnCommand {
    fn execute(&self) {
        self.light.on();
    }
}

pub struct GarageDoorOpenCommand {
    garage_door: Box<GarageDoor>,
}
impl GarageDoorOpenCommand {
    pub fn new(garage_door: GarageDoor) -> Self {
        GarageDoorOpenCommand {
            garage_door: Box::new(garage_door),
        }
    }
}
impl Command for GarageDoorOpenCommand {
    fn execute(&self) {
        self.garage_door.up();
    }
}
