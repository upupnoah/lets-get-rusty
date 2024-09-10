#![allow(dead_code, unused_variables, unused_imports)]

use std::{collections::HashMap, marker::PhantomData};

struct Locked;
struct Unlocked;

struct PasswordManager<State = Locked> {
    master_password: String,
    passwords: HashMap<String, String>,
    state: PhantomData<State>,
}

impl PasswordManager {
    pub fn new(master_pass: String) -> Self {
        PasswordManager {
            master_password: master_pass,
            passwords: Default::default(),
            state: PhantomData,
        }
    }
}

impl<State> PasswordManager<State> {
    // fn new(master_password: String) -> Self {
    //     Self {
    //         master_password,
    //         passwords: HashMap::new(),
    //         state: PhantomData,
    //     }
    // }

    fn encrypt(&self) -> String {
        todo!()
    }

    fn version(&self) -> String {
        todo!()
    }
}

impl PasswordManager<Locked> {
    fn unlock(&mut self, master_password: String) -> Result<PasswordManager<Unlocked>, String> {
        todo!()
    }
}

impl PasswordManager<Unlocked> {
    fn lock(&mut self) -> Result<PasswordManager<Locked>, String> {
        todo!()
    }

    fn add_password(&mut self, username: String, password: String) {
        todo!()
    }

    fn get_password(&self, username: String) -> Result<String, String> {
        todo!()
    }

    fn list_passwords(&self) -> Result<HashMap<String, String>, String> {
        todo!()
    }
}

fn main() {
    let mut manager = PasswordManager::new("1234".to_string());

    let mut unlocked_manager = manager.unlock("1234".to_string()).unwrap();

    unlocked_manager.add_password("user1".to_string(), "pass1".to_string());

    let password = unlocked_manager.get_password("user1".to_string()).unwrap();

    println!("Password for user1: {}", password);

    let locked_manager = unlocked_manager.lock().unwrap();
}
