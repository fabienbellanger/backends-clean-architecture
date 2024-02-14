//! Scope responses

use crate::entities::scope::Scope;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeResponse {
    pub id: String,
    pub created_at: String,
}

impl From<Scope> for ScopeResponse {
    /// Convert a `Scope` into `ScopeResponse`
    ///
    /// ```
    /// use clean_architecture_domain::entities::scope::Scope;
    /// use clean_architecture_domain::responses::scope::ScopeResponse;
    /// use chrono::{DateTime, Utc};
    /// use uuid::Uuid;
    ///
    /// let scope_id = "users:read".to_owned();
    /// let now = Utc::now();
    /// let scope = Scope {
    ///     id: scope_id.clone(),
    ///     created_at: now,
    ///     deleted_at: None,
    /// };
    ///
    /// let response = ScopeResponse {
    ///     id: scope_id,
    ///     created_at: now.to_rfc3339(),
    /// };
    ///
    /// assert_eq!(response, scope.into());
    /// ```
    fn from(scope: Scope) -> Self {
        Self {
            id: scope.id.to_string(),
            created_at: scope.created_at.to_rfc3339(),
        }
    }
}
