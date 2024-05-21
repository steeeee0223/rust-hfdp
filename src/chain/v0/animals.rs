use super::{AbstractHandler, Handler};

pub struct MonkeyHandler {
    next: AbstractHandler<String>,
}
impl MonkeyHandler {
    pub fn new() -> Self {
        Self {
            next: AbstractHandler::new(),
        }
    }
}
impl Handler<String> for MonkeyHandler {
    fn set_next(&mut self, handler: Box<dyn Handler<String>>) {
        self.next.set_next(handler);
    }
    fn execute(&self, request: &String) -> Option<String> {
        match request.as_str() {
            "Banana" => Some(format!("Monkey: I'll eat the {}.", request)),
            _ => self.next.execute(request),
        }
    }
}

pub struct SquirrelHandler {
    next: AbstractHandler<String>,
}
impl SquirrelHandler {
    pub fn new() -> Self {
        Self {
            next: AbstractHandler::new(),
        }
    }
}
impl Handler<String> for SquirrelHandler {
    fn set_next(&mut self, handler: Box<dyn Handler<String>>) {
        self.next.set_next(handler);
    }
    fn execute(&self, request: &String) -> Option<String> {
        match request.as_str() {
            "Nut" => Some(format!("Squirrel: I'll eat the {}.", request)),
            _ => self.next.execute(request),
        }
    }
}

pub struct DogHandler {
    next: AbstractHandler<String>,
}
impl DogHandler {
    pub fn new() -> Self {
        Self {
            next: AbstractHandler::new(),
        }
    }
}
impl Handler<String> for DogHandler {
    fn set_next(&mut self, handler: Box<dyn Handler<String>>) {
        self.next.set_next(handler);
    }
    fn execute(&self, request: &String) -> Option<String> {
        match request.as_str() {
            "MeatBall" => Some(format!("Dog: I'll eat the {}.", request)),
            _ => self.next.execute(request),
        }
    }
}
