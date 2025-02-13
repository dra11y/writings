use axum::{Json, extract::Path};
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi as DeriveOpenApi};
use utoipa_axum::{router::OpenApiRouter, routes};
use writings::{EmbedAllTrait as _, HiddenWord, HiddenWordKind};

use crate::{ApiResult, api_result::ApiError};

#[derive(DeriveOpenApi)]
#[openapi(components(schemas(HiddenWordKind, HiddenWord)))]
pub struct HiddenWordsApiDoc;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::with_openapi(HiddenWordsApiDoc::openapi())
        .routes(routes!(get_all_hidden_words))
        .routes(routes!(get_hidden_words_of_kind))
        .routes(routes!(get_hidden_word))
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = OK, body = Vec<HiddenWord>, description = "Hidden Words"),
    )
)]
pub async fn get_all_hidden_words() -> ApiResult<Json<Vec<HiddenWord>>> {
    Ok(Json(HiddenWord::all().to_vec()))
}

#[utoipa::path(
    get,
    path = "/{kind}",
    params(("kind" = HiddenWordKind, Path)),
    responses(
        (status = OK, body = Vec<HiddenWord>, description = "Hidden Words"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
pub async fn get_hidden_words_of_kind(
    Path(kind): Path<HiddenWordKind>,
) -> ApiResult<Json<Vec<HiddenWord>>> {
    Ok(Json(
        HiddenWord::all()
            .iter()
            .filter(|hw| hw.kind == kind)
            .cloned()
            .collect(),
    ))
}

#[derive(Deserialize, IntoParams)]
pub struct HiddenWordPath {
    kind: HiddenWordKind,
    num: u32,
}

#[utoipa::path(
    get,
    path = "/{kind}/{num}",
    params(HiddenWordPath),
    responses(
        (status = OK, body = HiddenWord, description = "Hidden Word"),
        (status = NOT_FOUND, description = "Hidden Word not found"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
pub async fn get_hidden_word(
    Path(HiddenWordPath { kind, num }): Path<HiddenWordPath>,
) -> ApiResult<Json<HiddenWord>> {
    Ok(Json(
        HiddenWord::all()
            .iter()
            .find(|hw| {
                hw.kind == kind
                    && match hw.number {
                        Some(n) => n == num,
                        None => num == 0,
                    }
            })
            .cloned()
            .ok_or(ApiError::NotFound)?,
    ))
}
