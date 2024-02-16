use std::{cell::RefCell, rc::Rc};

use rust_hfdp::command_2::{
    CeilingFan, CeilingFanOffCommand, CeilingFanOnCommand, GarageDoor, GarageDoorCloseCommand,
    GarageDoorOpenCommand, Light, LightOffCommand, LightOnCommand, RemoteControl, Stereo,
    StereoOffCommand, StereoOnWithCDCommand,
};

fn main() {
    let mut remote = RemoteControl::new();
    // Devices
    let living_room_light = Rc::new(RefCell::new(Light::new("Living Room")));
    let kitchen_light = Rc::new(RefCell::new(Light::new("Kitchen")));
    let ceiling_fan = Rc::new(RefCell::new(CeilingFan::new("Living Room")));
    let garage_door = Rc::new(RefCell::new(GarageDoor));
    let stereo = Rc::new(RefCell::new(Stereo::new("Living Room")));
    // Commands
    let living_room_light_on = LightOnCommand::new(Rc::clone(&living_room_light));
    let living_room_light_off = LightOffCommand::new(Rc::clone(&living_room_light));
    let kitchen_light_on = LightOnCommand::new(Rc::clone(&kitchen_light));
    let kitchen_light_off = LightOffCommand::new(Rc::clone(&kitchen_light));
    let ceiling_fan_on = CeilingFanOnCommand::new(Rc::clone(&ceiling_fan));
    let ceiling_fan_off = CeilingFanOffCommand::new(Rc::clone(&ceiling_fan));
    let garage_door_open = GarageDoorOpenCommand::new(Rc::clone(&garage_door));
    let garage_door_close = GarageDoorCloseCommand::new(Rc::clone(&garage_door));
    let stereo_on = StereoOnWithCDCommand::new(Rc::clone(&stereo));
    let stereo_off = StereoOffCommand::new(Rc::clone(&stereo));
    // Set commands
    remote.command_mut(
        0,
        Box::new(living_room_light_on),
        Box::new(living_room_light_off),
    );
    remote.command_mut(1, Box::new(kitchen_light_on), Box::new(kitchen_light_off));
    remote.command_mut(2, Box::new(ceiling_fan_on), Box::new(ceiling_fan_off));
    remote.command_mut(3, Box::new(garage_door_open), Box::new(garage_door_close));
    remote.command_mut(4, Box::new(stereo_on), Box::new(stereo_off));
    // Execute
    println!("{}", remote);
    remote.press_on(0);
    remote.press_off(0);
    remote.press_on(1);
    remote.press_off(1);
    remote.press_on(2);
    remote.press_off(2);
    remote.press_on(3);
    remote.press_off(3);
}
