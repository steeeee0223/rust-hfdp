use super::observers::Observer;

pub struct WeatherData<'a> {
    observers: Vec<&'a dyn Observer>,
    temperature: f64,
}

impl<'a> WeatherData<'a> {
    pub fn new() -> Self {
        WeatherData {
            observers: vec![],
            temperature: f64::NAN,
        }
    }

    pub fn register_observer(&mut self, observer: &'a dyn Observer) {
        self.observers.push(observer);
    }

    pub fn remove_observer(&mut self, observer: &'a dyn Observer) {
        if let Some(i) = self
            .observers
            .iter()
            .position(|&x| x as *const _ == observer as *const _)
        {
            self.observers.swap_remove(i);
        }
    }

    pub fn notify(&self) {
        self.observers
            .iter()
            .for_each(|&observer| observer.update(self.temperature))
    }

    pub fn set_measurements(&mut self, temperature: f64) {
        self.temperature = temperature;
        self.notify();
    }
}
