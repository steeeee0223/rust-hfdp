pub trait Handler<T> {
    fn set_next(&mut self, handler: Box<dyn Handler<T>>);
    fn execute(&self, request: &T) -> Option<T>;
}

pub struct AbstractHandler<T> {
    next: Option<Box<dyn Handler<T>>>,
}
impl<T> AbstractHandler<T> {
    pub fn new() -> Self {
        Self { next: None }
    }
}
impl<T> Handler<T> for AbstractHandler<T> {
    fn set_next(&mut self, handler: Box<dyn Handler<T>>) {
        self.next = Some(handler);
    }
    fn execute(&self, request: &T) -> Option<T> {
        match self.next {
            Some(ref handler) => handler.execute(request),
            None => None,
        }
    }
}
