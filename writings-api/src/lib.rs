//! REST OpenAPI for the <a target="_blank" href="https://crates.io/crates/writings">writings</a> crate.

mod api_result;
pub mod by_ref;
pub mod gleanings;
pub mod hidden_words;
pub mod meditations;
pub mod pagination;
pub mod prayers;
pub mod roman_number;
pub mod search;
mod util;

pub use api_result::{ApiError, ApiResult};
use axum::{ServiceExt, extract::Request};
use normalize_path_except::NormalizePath;
use roman_number::RomanNumber;
use std::{net::Ipv4Addr, sync::OnceLock};
use tokio::net::TcpListener;
use utoipa::OpenApi as DeriveOpenApi;
use utoipa_axum::router::OpenApiRouter;

#[derive(DeriveOpenApi)]
#[openapi(
    info(title = "Bahá’í Writings API", contact()),
    components(schemas(RomanNumber))
)]
pub struct ApiDoc;

static WRITINGS_API_TAG: OnceLock<String> = OnceLock::new();

pub fn api_tag() -> &'static str {
    WRITINGS_API_TAG
        .get()
        .map(|s| s.as_str())
        .unwrap_or_default()
}

pub fn build_openapi_router(tag: Option<&str>) -> OpenApiRouter {
    WRITINGS_API_TAG.get_or_init(|| {
        tag.map(|s| s.to_string()).unwrap_or_else(|| {
            std::env::var("WRITINGS_API_TAG").unwrap_or_else(|_| "writings".to_string())
        })
    });
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/hidden-words", hidden_words::router())
        .nest("/prayers", prayers::router())
        .nest("/gleanings", gleanings::router())
        .nest("/meditations", meditations::router())
        .nest("/ref", by_ref::router())
        .nest("/search", search::router())
}

pub async fn serve() -> ApiResult<()> {
    let (app, api) = build_openapi_router(None).split_for_parts();
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
