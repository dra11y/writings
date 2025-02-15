use axum::{Json, extract::Path};
use utoipa::OpenApi as DeriveOpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use writings::{EmbedAllTrait as _, Writings, WritingsTrait as _};

use crate::{ApiError, ApiResult, api_tag};

#[derive(DeriveOpenApi)]
#[openapi(components(schemas(Writings)))]
pub struct ByRefApiDoc;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::with_openapi(ByRefApiDoc::openapi()).routes(routes!(by_ref))
}

#[utoipa::path(
    get,
    path = "/{ref_id}",
    tag = api_tag(),
    params(
        ("ref_id" = String, Path, example = "646181142")
    ),
    responses(
        (status = OK, body = Writings, description = "Writings by ref_id"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
pub async fn by_ref(
    // MUST be a tuple or it doesn't make it into spec.
    Path((ref_id,)): Path<(String,)>,
) -> ApiResult<Json<Writings>> {
    Ok(Json(
        Writings::all()
            .iter()
            .find(|w| w.ref_id() == ref_id)
            .cloned()
            .ok_or(ApiError::NotFound)?,
    ))
}
