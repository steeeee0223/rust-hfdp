use rust_hfdp::proxy::v0::{Client, Proxy, RealSubject};

fn main() {
    let real_subject = RealSubject {};
    println!("\nclient: executing the client code with a real subject:");
    Client::client_code(&real_subject);

    println!("\nclient: executing the same client code with a proxy:");
    let proxy = Proxy::new(&real_subject);
    Client::client_code(&proxy);
}
