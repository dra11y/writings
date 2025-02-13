mod api_result;
pub mod hidden_words;
pub mod prayers;
mod util;

pub use api_result::{ApiError, ApiResult};
use axum::{Router, ServiceExt, extract::Request};
use normalize_path_except::NormalizePath;
use std::net::Ipv4Addr;
use tokio::net::TcpListener;
use utoipa::{OpenApi as DeriveOpenApi, openapi::OpenApi};
use utoipa_axum::router::OpenApiRouter;

#[derive(DeriveOpenApi)]
#[openapi()]
pub struct ApiDoc;

pub fn build_app_and_api() -> (Router, OpenApi) {
    let router = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/hidden-words", hidden_words::router())
        .nest("/prayers", prayers::router());
    router.split_for_parts()
}

pub async fn serve() -> ApiResult<()> {
    let (app, api) = build_app_and_api();
    let host: Ipv4Addr = util::get_from_env("HTTP_HOST", Ipv4Addr::LOCALHOST);
    let port: u16 = util::get_from_env("HTTP_PORT", 3000);
    let listener = TcpListener::bind((host, port)).await?;

    #[cfg(feature = "swagger")]
    let app = app.merge(
        utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api.clone()),
    );

    let app = NormalizePath::trim_trailing_slash(app, &["/swagger-ui"]);
    axum::serve(listener, ServiceExt::<Request>::into_make_service(app)).await?;
    Ok(())
}
