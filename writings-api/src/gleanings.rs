use axum::{Json, extract::Path};
use utoipa::OpenApi as DeriveOpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use writings::{EmbedAllTrait as _, GleaningsParagraph};

use crate::{ApiError, ApiResult, api_tag, roman_number::RomanNumber};

#[derive(DeriveOpenApi)]
#[openapi(components(schemas(GleaningsParagraph, RomanNumber)))]
pub struct GleaningsApiDoc;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::with_openapi(GleaningsApiDoc::openapi())
        .routes(routes!(gleanings_all))
        .routes(routes!(gleanings_by_number))
        .routes(routes!(gleaning))
}

#[utoipa::path(
    get,
    path = "/",
    tag = api_tag(),
    responses(
        (status = OK, body = Vec<GleaningsParagraph>, description = "Prayer Paragraphs"),
    )
)]
pub async fn gleanings_all() -> ApiResult<Json<Vec<GleaningsParagraph>>> {
    Ok(Json(GleaningsParagraph::all().to_vec()))
}

#[utoipa::path(
    get,
    path = "/{number}",
    tag = api_tag(),
    responses(
        (status = OK, body = Vec<GleaningsParagraph>, description = "Gleanings Paragraphs"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
pub async fn gleanings_by_number(
    // MUST be a tuple or it doesn't make it into spec.
    Path((number,)): Path<(RomanNumber,)>,
) -> ApiResult<Json<Vec<GleaningsParagraph>>> {
    Ok(Json(
        GleaningsParagraph::all()
            .iter()
            .filter(|p| p.number == number.0)
            .cloned()
            .collect(),
    ))
}

#[utoipa::path(
    get,
    path = "/{number}/{paragraph}",
    tag = api_tag(),
    responses(
        (status = OK, body = GleaningsParagraph, description = "Gleanings Paragraph"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
pub async fn gleaning(
    Path((number, paragraph)): Path<(RomanNumber, u32)>,
) -> ApiResult<Json<GleaningsParagraph>> {
    Ok(Json(
        GleaningsParagraph::all()
            .iter()
            .find(|p| p.number == number.0 && p.paragraph == paragraph)
            .cloned()
            .ok_or(ApiError::NotFound)?,
    ))
}
