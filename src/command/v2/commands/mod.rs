mod ceiling_fan;
mod garage_door;
mod light;
mod stereo;

pub use ceiling_fan::*;
pub use garage_door::*;
pub use light::*;
pub use stereo::*;

pub trait Command {
    fn name(&self) -> String;
    fn execute(&self);
}

pub struct NoCommand;
impl Command for NoCommand {
    fn name(&self) -> String {
        String::from("No command")
    }
    fn execute(&self) {}
}
