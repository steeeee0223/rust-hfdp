use std::rc::{Rc, Weak};

use super::observers::Observer;

pub struct WeatherData {
    observers: Vec<Weak<dyn Observer>>,
    temperature: f64,
}

impl WeatherData {
    pub fn new() -> Self {
        WeatherData {
            observers: vec![],
            temperature: f64::NAN,
        }
    }

    pub fn register_observer(&mut self, observer: &Rc<dyn Observer>) {
        self.observers.push(Rc::downgrade(observer));
    }

    pub fn remove_observer(&mut self, observer: &Rc<dyn Observer>) {
        if let Some(i) = self.observers.iter().position(|x| match x.upgrade() {
            Some(ptr) => Rc::ptr_eq(&ptr, observer),
            None => false,
        }) {
            self.observers.swap_remove(i);
        }
    }

    pub fn notify(&self) {
        self.observers.iter().for_each(|observer| {
            if let Some(x) = observer.upgrade() {
                x.update(self.temperature)
            }
        })
    }

    pub fn set_measurements(&mut self, temperature: f64) {
        self.temperature = temperature;
        self.notify();
    }
}
