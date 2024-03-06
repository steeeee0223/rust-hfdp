use std::{cell::RefCell, rc::Rc};

use super::observers::Observer;

pub struct WeatherData {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
    temperature: f64,
}

impl WeatherData {
    pub fn new() -> Self {
        WeatherData {
            observers: vec![],
            temperature: f64::NAN,
        }
    }

    pub fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    pub fn remove_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.remove(
            self.observers
                .iter()
                .position(|x| x.borrow().get_id() == observer.borrow().get_id())
                .expect("observer not found"),
        );
    }

    pub fn notify(&self) {
        self.observers
            .iter()
            .for_each(|observer| observer.borrow_mut().update(self.temperature))
    }

    pub fn set_measurements(&mut self, temperature: f64) {
        self.temperature = temperature;
        self.notify();
    }
}
