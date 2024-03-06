use rust_hfdp::singleton::v1::{ChocolateBoiler, Status, INIT};

fn main() {
    unsafe {
        INIT = Some(|| Status {
            empty: true,
            boiled: false,
        })
    }

    let boiler = ChocolateBoiler::instance();
    boiler.boil();
    boiler.fill();
    boiler.boil();
    boiler.drain();
}
