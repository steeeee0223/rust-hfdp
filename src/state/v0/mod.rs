use std::{collections::HashMap, marker::PhantomData};

pub struct Locked;
pub struct Unlocked;

pub struct PasswordManager<State = Locked> {
    master_pass: String,
    passwords: HashMap<String, String>,
    state: PhantomData<State>,
}
impl PasswordManager {
    pub fn new(master_pass: String) -> Self {
        PasswordManager {
            master_pass,
            passwords: Default::default(),
            state: Default::default(),
        }
    }
}
impl<State> PasswordManager<State> {
    pub fn encryption(&self) -> String {
        todo!()
    }
    pub fn version(&self) -> String {
        "v1".to_owned()
    }
}

impl PasswordManager<Locked> {
    pub fn unlock(self, master_pass: String) -> PasswordManager<Unlocked> {
        PasswordManager {
            master_pass,
            passwords: self.passwords,
            state: PhantomData::<Unlocked>,
        }
    }
}

impl PasswordManager<Unlocked> {
    pub fn lock(self) -> PasswordManager<Locked> {
        PasswordManager {
            master_pass: self.master_pass,
            passwords: self.passwords,
            state: PhantomData::<Locked>,
        }
    }
    pub fn list_passwords(&self) -> &HashMap<String, String> {
        &self.passwords
    }
    pub fn add_password(&mut self, username: String, password: String) {
        self.passwords.insert(username, password);
    }
}
