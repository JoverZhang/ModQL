//! Core types and traits for the application.
//!
//! This crate provides the foundational abstractions used across the workspace.

/// A unique identifier for entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(pub u64);

/// A user entity with profile information.
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    /// The user's unique identifier.
    pub id: Id,
    /// Display name shown in the UI.
    pub name: String,
    /// Email address for notifications.
    email: String,
}

/// Status of an entity in the system.
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    /// Entity is active and can be used.
    Active,
    /// Entity is temporarily suspended.
    Suspended,
    /// Entity has been permanently removed.
    Deleted,
}

/// A repository for loading and storing entities.
///
/// Implement this trait to provide persistence for a specific entity type.
pub trait Repository {
    /// The entity type managed by this repository.
    type Item;
    /// The error type returned by repository operations.
    type Error;

    /// Retrieve an entity by its identifier.
    fn get(&self, id: Id) -> Result<Option<Self::Item>, Self::Error>;

    /// Persist an entity, returning its identifier.
    fn save(&mut self, item: &Self::Item) -> Result<Id, Self::Error>;
}

/// A trait for types that can describe themselves in a human-readable way.
///
/// Provides a default implementation that returns `"(no description)"`.
pub trait Describable {
    /// Return a human-readable description.
    fn describe(&self) -> String {
        "(no description)".to_string()
    }
}

/// A timestamp represented as milliseconds since the Unix epoch.
pub type Timestamp = u64;

/// Default page size for paginated queries.
pub const DEFAULT_PAGE_SIZE: usize = 25;
