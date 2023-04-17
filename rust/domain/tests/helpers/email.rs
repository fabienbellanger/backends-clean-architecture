use clean_architecture_domain::ports::requests::user::ForgottenPasswordRequest;
use clean_architecture_domain::ports::services::email::EmailService;
use clean_architecture_shared::error::ApiResult;

pub(crate) struct TestEmailService {}

impl EmailService for TestEmailService {
    fn forgotten_password(
        &self,
        _request: ForgottenPasswordRequest,
        _token: &str,
    ) -> ApiResult<()> {
        Ok(())
    }
}
