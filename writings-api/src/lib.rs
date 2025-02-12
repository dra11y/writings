mod api_result;

use std::net::Ipv4Addr;

pub use api_result::ApiResult;
use axum::{Json, Router, extract::Path};
use tokio::net::TcpListener;
use utoipa::{OpenApi as DeriveOpenApi, openapi::OpenApi};
use utoipa_axum::{router::OpenApiRouter, routes};
use writings::{EmbedAllTrait, HiddenWord, HiddenWordKind};

#[derive(DeriveOpenApi)]
pub struct ApiDoc;

#[utoipa::path(get, path = "/hidden-words/{kind}")]
pub async fn hidden_words(Path(kind): Path<HiddenWordKind>) -> ApiResult<Json<Vec<HiddenWord>>> {
    Ok(Json(
        HiddenWord::all()
            .iter()
            .filter(|hw| hw.kind == kind)
            .cloned()
            .collect(),
    ))
}

pub fn build_app_and_api() -> (Router, OpenApi) {
    let router = OpenApiRouter::with_openapi(ApiDoc::openapi()).routes(routes!(hidden_words));
    let (router, api) = router.split_for_parts();
    (
        #[cfg(feature = "swagger")]
        router.merge(
            utoipa_swagger_ui::SwaggerUi::new("/swagger-ui")
                .url("/apidoc/openapi.json", api.clone()),
        ),
        #[cfg(not(feature = "swagger"))]
        router,
        api,
    )
}

pub async fn run() -> ApiResult<()> {
    let app = build_app_and_api().0;
    let host: Ipv4Addr = std::env::var("HTTP_HOST")
        .unwrap_or_default()
        .parse()
        .unwrap_or(Ipv4Addr::LOCALHOST);
    let port: u16 = std::env::var("HTTP_PORT")
        .unwrap_or_default()
        .parse()
        .unwrap_or(3000);
    let listener = TcpListener::bind((host, port)).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
