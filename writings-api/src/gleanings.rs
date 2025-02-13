use axum::{Json, extract::Path};
use utoipa::OpenApi as DeriveOpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use writings::{EmbedAllTrait as _, GleaningParagraph};

use crate::{ApiError, ApiResult, util::RomanNumber};

#[derive(DeriveOpenApi)]
#[openapi(components(schemas(GleaningParagraph)))]
pub struct GleaningsApiDoc;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::with_openapi(GleaningsApiDoc::openapi())
        .routes(routes!(get_all_gleanings))
        .routes(routes!(get_gleanings_number))
        .routes(routes!(get_gleanings_number_paragraph))
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = OK, body = Vec<GleaningParagraph>, description = "Prayer Paragraphs"),
    )
)]
pub async fn get_all_gleanings() -> ApiResult<Json<Vec<GleaningParagraph>>> {
    Ok(Json(GleaningParagraph::all().to_vec()))
}

#[utoipa::path(
    get,
    path = "/{number}",
    params(RomanNumber),
    responses(
        (status = OK, body = Vec<GleaningParagraph>, description = "Gleanings Paragraphs"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
#[axum::debug_handler]
pub async fn get_gleanings_number(
    Path(number): Path<RomanNumber>,
) -> ApiResult<Json<Vec<GleaningParagraph>>> {
    Ok(Json(
        GleaningParagraph::all()
            .iter()
            .filter(|p| p.number == number.0)
            .cloned()
            .collect(),
    ))
}

#[utoipa::path(
    get,
    path = "/{number}/{paragraph}",
    params(RomanNumber, ("paragraph" = u32, Path)),
    responses(
        (status = OK, body = GleaningParagraph, description = "Gleanings Paragraph"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
#[axum::debug_handler]
pub async fn get_gleanings_number_paragraph(
    Path((number, paragraph)): Path<(RomanNumber, u32)>,
) -> ApiResult<Json<GleaningParagraph>> {
    Ok(Json(
        GleaningParagraph::all()
            .iter()
            .find(|p| p.number == number.0 && p.paragraph == paragraph)
            .cloned()
            .ok_or(ApiError::NotFound)?,
    ))
}
