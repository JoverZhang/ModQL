//! Service layer for business logic.
//!
//! This crate implements the application's core use cases on top of the
//! `core` crate abstractions.

pub mod handler;

use core::{Describable, Id, Repository, User};

/// An in-memory user store.
pub struct UserStore {
    /// Users indexed by their identifier.
    users: Vec<User>,
}

impl Repository for UserStore {
    type Item = User;
    type Error = String;

    /// Look up a user by id.
    fn get(&self, id: Id) -> Result<Option<User>, String> {
        Ok(self.users.iter().find(|u| u.id == id).cloned())
    }

    /// Store a user, returning the assigned id.
    fn save(&mut self, item: &User) -> Result<Id, String> {
        let id = item.id;
        self.users.push(item.clone());
        Ok(id)
    }
}

impl Describable for UserStore {
    /// Describes the user store with its current size.
    fn describe(&self) -> String {
        format!("UserStore with {} users", self.users.len())
    }
}

/// Service-level configuration.
struct ServiceConfig {
    /// Maximum number of concurrent requests.
    max_concurrent: usize,
}

/// Initialize the service layer with an empty user store.
pub fn init() -> UserStore {
    UserStore { users: Vec::new() }
}
