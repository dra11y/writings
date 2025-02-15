use axum::{Json, extract::Path};
use utoipa::OpenApi as DeriveOpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use writings::{EmbedAllTrait as _, MeditationParagraph};

use crate::{ApiError, ApiResult, api_tag, roman_number::RomanNumber, util::openapi_with_tag};

#[derive(DeriveOpenApi)]
#[openapi(components(schemas(MeditationParagraph)))]
pub struct MeditationsApiDoc;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::with_openapi(MeditationsApiDoc::openapi())
        .routes(routes!(meditations_all))
        .routes(routes!(meditations_by_number))
        .routes(routes!(meditation))
}

#[utoipa::path(
    get,
    path = "/",
    tag = api_tag(),
    responses(
        (status = OK, body = Vec<MeditationParagraph>, description = "Prayer Paragraphs"),
    )
)]
pub async fn meditations_all() -> ApiResult<Json<Vec<MeditationParagraph>>> {
    Ok(Json(MeditationParagraph::all().to_vec()))
}

#[utoipa::path(
    get,
    path = "/{number}",
    tag = api_tag(),
    // params(RomanNumber),
    responses(
        (status = OK, body = Vec<MeditationParagraph>, description = "Meditations Paragraphs"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
#[axum::debug_handler]
pub async fn meditations_by_number(
    Path((number,)): Path<(RomanNumber,)>,
) -> ApiResult<Json<Vec<MeditationParagraph>>> {
    Ok(Json(
        MeditationParagraph::all()
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
        (status = OK, body = MeditationParagraph, description = "Meditations Paragraph"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
pub async fn meditation(
    Path((number, paragraph)): Path<(RomanNumber, u32)>,
) -> ApiResult<Json<MeditationParagraph>> {
    Ok(Json(
        MeditationParagraph::all()
            .iter()
            .find(|p| p.number == number.0 && p.paragraph == paragraph)
            .cloned()
            .ok_or(ApiError::NotFound)?,
    ))
}
