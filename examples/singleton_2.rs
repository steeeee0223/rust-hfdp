use rust_hfdp::Singleton;

fn main() {
    Singleton!(u32, singleton, || 42);
    let instance = singleton::instance();

    println!("Instance: {:?}", *instance.inner().lock().unwrap());

    *instance.inner().get_mut().unwrap() = 24;

    println!("Instance: {:?}", *instance.inner().lock().unwrap());
}
