mod current_condition;
mod predict;

pub use current_condition::*;
pub use predict::*;

pub trait Observer {
    fn update(&mut self, temp: f64);
    fn get_id(&self) -> usize;
}
