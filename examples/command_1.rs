use rust_hfdp::command::v1::{
    GarageDoor, GarageDoorOpenCommand, Light, LightOnCommand, SimpleRemoteControl,
};

fn main() {
    let mut remote = SimpleRemoteControl::new();
    // Devices
    let light = Light::new("Living room");
    // Commands
    let light_on = LightOnCommand::new(light);
    let garage_open = GarageDoorOpenCommand::new(GarageDoor);
    // Execute
    remote.command_mut(Box::new(light_on));
    remote.button_pressed();
    remote.command_mut(Box::new(garage_open));
    remote.button_pressed();
}
