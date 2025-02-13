use axum::{Json, extract::Path};
use utoipa::OpenApi as DeriveOpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use writings::{EmbedAllTrait as _, GleaningsParagraph};

use crate::{ApiError, ApiResult, util::RomanNumber};

#[derive(DeriveOpenApi)]
#[openapi(components(schemas(GleaningsParagraph)))]
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
        (status = OK, body = Vec<GleaningsParagraph>, description = "Prayer Paragraphs"),
    )
)]
pub async fn get_all_gleanings() -> ApiResult<Json<Vec<GleaningsParagraph>>> {
    Ok(Json(GleaningsParagraph::all().to_vec()))
}

#[utoipa::path(
    get,
    path = "/{number}",
    params(RomanNumber),
    responses(
        (status = OK, body = Vec<GleaningsParagraph>, description = "Gleanings Paragraphs"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
#[axum::debug_handler]
pub async fn get_gleanings_number(
    Path(number): Path<RomanNumber>,
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
    params(RomanNumber, ("paragraph" = u32, Path)),
    responses(
        (status = OK, body = GleaningsParagraph, description = "Gleanings Paragraph"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
#[axum::debug_handler]
pub async fn get_gleanings_number_paragraph(
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
