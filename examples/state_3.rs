use rust_hfdp::state::v0::PasswordManager;

fn main() {
    let manager = PasswordManager::new("pass123".to_owned());

    let mut manager = manager.unlock("pass123".to_owned());
    manager.add_password("steve".to_owned(), "12345".to_owned());

    let pass = manager.list_passwords();
    println!("{:?}", pass);
    manager.lock();
}
