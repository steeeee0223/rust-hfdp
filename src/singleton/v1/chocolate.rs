use std::{
    mem::MaybeUninit,
    sync::{Mutex, Once},
};

pub struct Status {
    pub empty: bool,
    pub boiled: bool,
}

pub static mut CHOCOLATE_BOILER: MaybeUninit<ChocolateBoiler> = MaybeUninit::uninit();
pub static ONCE: Once = Once::new();
pub static mut INIT: Option<fn() -> Status> = None;

pub struct ChocolateBoiler {
    inner: Mutex<Status>,
}

impl ChocolateBoiler {
    pub fn instance() -> &'static mut ChocolateBoiler {
        ONCE.call_once(|| {
            unsafe {
                if let None = INIT {
                    panic!("Singleton must be initialized before it is being used.")
                }
            }

            let init = unsafe { INIT.unwrap() };
            let value = init();
            let singleton = ChocolateBoiler {
                inner: Mutex::new(value),
            };

            unsafe {
                CHOCOLATE_BOILER.write(singleton);
            }
        });

        unsafe { CHOCOLATE_BOILER.assume_init_mut() }
    }

    pub fn fill(&mut self) {
        if self.empty() {
            println!("Filling the boiler with a milk/chocolate mixture...");
            *self.inner.get_mut().unwrap() = Status {
                empty: false,
                boiled: false,
            };
        } else {
            println!("The boiler is already filled...");
        }
    }

    pub fn drain(&mut self) {
        if !self.empty() && self.boiled() {
            println!("Draining the boiled milk and chocolate...");
            (*self.inner.get_mut().unwrap()).empty = true;
        } else {
            println!("Cannot drain the boiler...");
        }
    }

    pub fn boil(&mut self) {
        if !self.empty() && !self.boiled() {
            println!("Bringing the contents to a boil...");
            (*self.inner.get_mut().unwrap()).boiled = true;
        } else {
            println!("Either the boiler is empty or the mixture has already been boiled...");
        }
    }

    pub fn empty(&self) -> bool {
        (*self.inner.lock().unwrap()).empty
    }

    pub fn boiled(&self) -> bool {
        (*self.inner.lock().unwrap()).boiled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleton() {
        unsafe {
            super::INIT = Some(|| Status {
                empty: true,
                boiled: false,
            })
        }
        let instance1 = ChocolateBoiler::instance();
        let instance2 = ChocolateBoiler::instance();

        assert_eq!((*instance1.inner.lock().unwrap()).boiled, false);
        assert_eq!((*instance2.inner.lock().unwrap()).boiled, false);

        *instance1.inner.get_mut().unwrap() = Status {
            empty: false,
            boiled: true,
        };
        assert_eq!((*instance1.inner.lock().unwrap()).boiled, true);
    }
}
