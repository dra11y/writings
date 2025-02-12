use writings_api::{ApiResult, run};

#[tokio::main]
async fn main() -> ApiResult<()> {
    run().await
}
