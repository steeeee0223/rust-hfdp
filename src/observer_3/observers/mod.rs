mod current_condition;
mod predict;

pub use current_condition::*;
pub use predict::*;

pub trait Observer {
    fn update(&self, temp: f64);
}
