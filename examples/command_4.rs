use std::{cell::RefCell, rc::Rc};

use rust_hfdp::command_4::{
    CeilingFan, CeilingFanOffCommand, CeilingFanOnCommand, Light, LightOffCommand, LightOnCommand,
    MacroCommand, RemoteControl, Stereo, StereoOffCommand, StereoOnWithCDCommand,
};

fn main() {
    let mut remote = RemoteControl::new();
    // Devices
    let living_room_light = Rc::new(RefCell::new(Light::new("Living Room")));
    let ceiling_fan = Rc::new(RefCell::new(CeilingFan::new("Living Room")));
    let stereo = Rc::new(RefCell::new(Stereo::new("Living Room")));
    // Commands
    let living_room_light_on = LightOnCommand::new(Rc::clone(&living_room_light));
    let living_room_light_off = LightOffCommand::new(Rc::clone(&living_room_light));
    let ceiling_fan_on = CeilingFanOnCommand::new(Rc::clone(&ceiling_fan));
    let ceiling_fan_off = CeilingFanOffCommand::new(Rc::clone(&ceiling_fan));
    let stereo_on = StereoOnWithCDCommand::new(Rc::clone(&stereo));
    let stereo_off = StereoOffCommand::new(Rc::clone(&stereo));
    // Macro
    let party_on = MacroCommand::new(vec![
        Rc::new(RefCell::new(living_room_light_on)),
        Rc::new(RefCell::new(ceiling_fan_on)),
        Rc::new(RefCell::new(stereo_on)),
    ]);
    let party_off = MacroCommand::new(vec![
        Rc::new(RefCell::new(living_room_light_off)),
        Rc::new(RefCell::new(ceiling_fan_off)),
        Rc::new(RefCell::new(stereo_off)),
    ]);
    // Set commands
    remote.command_mut(
        0,
        Rc::new(RefCell::new(party_on)),
        Rc::new(RefCell::new(party_off)),
    );

    // Execute
    println!("{}", remote);
    remote.press_on(0);
    println!("{}", remote);
    remote.press_off(0);
}
