use super::Command;

pub struct SimpleRemoteControl {
    slot: Option<Box<dyn Command>>,
}
impl SimpleRemoteControl {
    pub fn new() -> Self {
        SimpleRemoteControl { slot: None }
    }
    pub fn command_mut(&mut self, command: Box<dyn Command>) {
        self.slot = Some(command);
    }
    pub fn button_pressed(&self) {
        self.slot.as_ref().unwrap().execute();
    }
}
