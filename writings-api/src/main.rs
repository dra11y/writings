use writings_api::{WritingsApiResult, serve};

#[tokio::main]
async fn main() -> WritingsApiResult<()> {
    serve().await
}
