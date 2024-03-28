use std::fmt;

pub trait MenuComponent
where
    Self: fmt::Display,
{
    fn add(&mut self, menu: Box<dyn MenuComponent>);
    fn remove(&mut self, menu: Box<dyn MenuComponent>);
    fn child(&self, i: usize) -> &Box<dyn MenuComponent>;
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn is_vegetarian(&self) -> bool;
    fn price(&self) -> f32;
}
