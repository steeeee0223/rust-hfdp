use std::{cell::RefCell, rc::Rc};

use rust_hfdp::command::v3::{Light, LightOffCommand, LightOnCommand, RemoteControl};

fn main() {
    let mut remote = RemoteControl::new();
    // Devices
    let living_room_light = Rc::new(RefCell::new(Light::new("Living Room")));
    // Commands
    let living_room_light_on = LightOnCommand::new(Rc::clone(&living_room_light));
    let living_room_light_off = LightOffCommand::new(Rc::clone(&living_room_light));
    // Set commands
    remote.command_mut(
        0,
        Rc::new(RefCell::new(living_room_light_on)),
        Rc::new(RefCell::new(living_room_light_off)),
    );
    // Execute
    remote.press_on(0);
    remote.press_off(0);
    println!("{}", remote);
    remote.press_undo();
    remote.press_off(0);
    remote.press_on(0);
    println!("{}", remote);
    remote.press_undo();
}
