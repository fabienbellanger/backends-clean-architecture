//! Scope entity

use chrono::{DateTime, Utc};
use clean_architecture_shared::auth::AuthScope;

/// Users scope
pub const SCOPE_USERS: &str = "users";
/// Admin scope
pub const SCOPE_ADMIN: &str = "admin";

/// Scope ID
pub type ScopeId = String;

/// Scope entity
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Scope {
    pub id: ScopeId,
    pub created_at: DateTime<Utc>,
        pub deleted_at: Option<DateTime<Utc>>,
}

impl Scope {
    /// Create a new scope
    pub fn new(id: String) -> Self {
        Self {
            id,
            created_at: Utc::now(),
            deleted_at: None,
        }
    }
}

impl From<Scope> for AuthScope {
    fn from(scope: Scope) -> Self {
        Self::new(scope.id.to_string())
    }
}
