use std::collections::HashMap;
use std::sync::RwLock;

use crate::models::user::User;

#[derive(Default)]
struct Store {
    users: HashMap<u64, User>,
    next_id: u64,
}

#[derive(Default)]
pub struct UserRepository {
    store: RwLock<Store>,
}

impl UserRepository {
    pub fn find_all(&self) -> Vec<User> {
        let store = self.store.read().unwrap();
        let mut users: Vec<User> = store.users.values().cloned().collect();
        users.sort_by_key(|u| u.id);
        users
    }

    pub fn find_by_id(&self, id: u64) -> Option<User> {
        self.store.read().unwrap().users.get(&id).cloned()
    }

    pub fn find_by_email(&self, email: &str) -> Option<User> {
        self.store
            .read()
            .unwrap()
            .users
            .values()
            .find(|u| u.email == email)
            .cloned()
    }

    pub fn insert(&self, name: String, email: String) -> User {
        let mut store = self.store.write().unwrap();
        store.next_id += 1;
        let user = User { id: store.next_id, name, email };
        store.users.insert(user.id, user.clone());
        user
    }

    pub fn update(&self, user: User) -> Option<User> {
        let mut store = self.store.write().unwrap();
        let slot = store.users.get_mut(&user.id)?;
        *slot = user.clone();
        Some(user)
    }

    pub fn delete(&self, id: u64) -> bool {
        self.store.write().unwrap().users.remove(&id).is_some()
    }
}