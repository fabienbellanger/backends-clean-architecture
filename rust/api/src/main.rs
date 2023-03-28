use clean_architecture_api::server::start_server;
use clean_architecture_shared::error::ApiResult;

#[tokio::main]
async fn main() -> ApiResult<()> {
    start_server().await
}
