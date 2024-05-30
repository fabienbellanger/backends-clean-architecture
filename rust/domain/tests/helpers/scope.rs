use async_trait::async_trait;
use chrono::NaiveDateTime;
use clean_architecture_domain::repositories::scope::request::{
    CreateScopeRepositoryRequest, DeleteScopeRepositoryRequest,
};
use clean_architecture_domain::repositories::scope::response::{
    CreateScopeRepositoryResponse, DeleteScopeRepositoryResponse, GetScopesRepositoryResponse, ScopeRepositoryResponse,
};
use clean_architecture_domain::repositories::scope::ScopeRepository;
use clean_architecture_shared::error::{ApiError, ApiResult};

pub(crate) const DATE: &str = "2023-04-01T12:10:00+00:00";
pub(crate) const SCOPE_ID: &str = "users:read";

pub(crate) struct TestScopeRepository {}

#[async_trait]
impl ScopeRepository for TestScopeRepository {
    async fn create(&self, _request: CreateScopeRepositoryRequest) -> ApiResult<CreateScopeRepositoryResponse> {
        Ok(())
    }

    async fn get_scopes(&self) -> ApiResult<GetScopesRepositoryResponse> {
        Ok(GetScopesRepositoryResponse {
            scopes: vec![ScopeRepositoryResponse {
                id: SCOPE_ID.to_string(),
                created_at: NaiveDateTime::parse_from_str(DATE, "%Y-%m-%dT%H:%M:%S%:z").unwrap(),
            }],
        })
    }

    async fn delete(&self, request: DeleteScopeRepositoryRequest) -> ApiResult<DeleteScopeRepositoryResponse> {
        let id = SCOPE_ID.to_string();

        if id == request.id {
            Ok(DeleteScopeRepositoryResponse { deleted: 1 })
        } else {
            Err(ApiError::NotFound {
                message: "no scope found".to_owned(),
            })
        }
    }
}
