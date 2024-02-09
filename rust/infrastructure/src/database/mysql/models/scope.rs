//! Scope model

use chrono::{DateTime, NaiveDateTime, Utc};
use clean_architecture_domain::entities::scope::Scope;
use sqlx::FromRow;

#[derive(Debug, PartialEq, Eq, FromRow)]
pub struct ScopeModel {
    pub id: String,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl From<Scope> for ScopeModel {
    fn from(scope: Scope) -> Self {
        Self {
            id: scope.id,
            created_at: scope.created_at.naive_utc(),
            deleted_at: scope.deleted_at.map(|dt| dt.naive_utc()),
        }
    }
}

impl From<ScopeModel> for Scope {
    fn from(scope: ScopeModel) -> Scope {
        Scope {
            id: scope.id,
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(scope.created_at, Utc),
            deleted_at: scope
                .deleted_at
                .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_scope_entity() {
        let scope_id = "scope.module:read".to_string();
        let now = Utc::now();
        let now_naive = now.naive_utc();
        let scope = Scope {
            id: scope_id.clone(),
            created_at: now,
            deleted_at: None,
        };
        let expected: ScopeModel = ScopeModel {
            id: scope_id,
            created_at: now_naive,
            deleted_at: None,
        };
        assert_eq!(ScopeModel::from(scope), expected);
    }

    #[test]
    fn test_into_scope_entity_valid() {
        let scope_id = "scope.module:read".to_string();
        let now = Utc::now();
        let now_naive = now.naive_utc();
        let scope_model: ScopeModel = ScopeModel {
            id: scope_id.clone(),
            created_at: now_naive,
            deleted_at: None,
        };
        let expected = Scope {
            id: scope_id,
            created_at: now,
            deleted_at: None,
        };
        assert_eq!(Scope::from(scope_model), expected);
    }
}
