use super::{RealSubject, Subject};

pub struct Proxy<'a> {
    real_subject: &'a RealSubject,
}

impl<'a> Proxy<'a> {
    pub fn new(real_subject: &'a RealSubject) -> Proxy {
        Proxy { real_subject }
    }
    fn check_access(&self) -> bool {
        // Some real checks should go here.
        println!("Proxy: checking access prior to firing a real request.");
        true
    }
    fn log_access(&self) {
        println!("Proxy: logging the request.");
    }
}

impl<'a> Subject for Proxy<'a> {
    // The most common applications of the Proxy pattern are lazy loading,
    // caching, controlling the access, logging, etc. A Proxy can perform one of
    // these things and then, depending on the result, pass the execution to the
    // same method in a linked RealSubject object.
    fn request(&self) {
        if self.check_access() {
            self.real_subject.request();
            self.log_access();
        }
    }
}
