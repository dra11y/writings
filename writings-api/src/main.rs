use writings_api::{ApiResult, serve};

#[tokio::main]
async fn main() -> ApiResult<()> {
    serve().await
}
