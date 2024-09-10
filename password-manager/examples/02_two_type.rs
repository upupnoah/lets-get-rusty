#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

struct LockedPasswordManager {
    master_password: String,
    passwords: HashMap<String, String>,
}

struct UnlockedPasswordManager {
    master_password: String,
    passwords: HashMap<String, String>,
}

impl LockedPasswordManager {
    fn new(master_password: String) -> Self {
        Self {
            master_password,
            passwords: HashMap::new(),
        }
    }

    fn unlock(&mut self, master_password: String) -> Result<UnlockedPasswordManager, String> {
        todo!()
    }

    fn encrypt(&self) -> String {
        todo!()
    }

    fn version(&self) -> String {
        todo!()
    }
}

impl UnlockedPasswordManager {
    fn new(master_password: String, passwords: HashMap<String, String>) -> Self {
        Self {
            master_password,
            passwords,
        }
    }

    fn lock(&mut self) -> Result<LockedPasswordManager, String> {
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

    fn encrypt(&self) -> String {
        todo!()
    }

    fn version(&self) -> String {
        todo!()
    }
}

fn main() {
    let mut manager = LockedPasswordManager::new("master_password".to_string());
    let mut unlocked_manager = manager.unlock("master_password".to_string()).unwrap();
    unlocked_manager.add_password("username".to_string(), "password".to_string());
    let password = unlocked_manager
        .get_password("username".to_string())
        .unwrap();
    println!("Password: {}", password);
}
