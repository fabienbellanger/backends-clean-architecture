use async_trait::async_trait;
use chrono::{DateTime, Utc};
use clean_architecture_domain::entities::scope::Scope;
use clean_architecture_domain::ports::repositories::scope::ScopeRepository;
use clean_architecture_domain::ports::requests::scope::{CreateRequest, DeleteRequest};
use clean_architecture_shared::error::{ApiError, ApiResult};

pub(crate) const DATE: &str = "2023-04-01T12:10:00+00:00";
pub(crate) const SCOPE_ID: &str = "users:read";

pub(crate) struct TestScopeRepository {}

#[async_trait]
impl ScopeRepository for TestScopeRepository {
    async fn get_scopes(&self) -> ApiResult<Vec<Scope>> {
        Ok(vec![Scope {
            id: SCOPE_ID.to_string(),
            created_at: DateTime::parse_from_rfc3339(DATE).unwrap().with_timezone(&Utc),
            deleted_at: None,
        }])
    }

    async fn create(&self, _request: CreateRequest) -> ApiResult<()> {
        Ok(())
    }

    async fn delete(&self, request: DeleteRequest) -> ApiResult<u64> {
        let id = SCOPE_ID.to_string();

        if id == request.id {
            Ok(1)
        } else {
            Err(ApiError::NotFound {
                message: "no scope found".to_owned(),
            })
        }
    }
}
