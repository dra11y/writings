use axum::{Json, extract::Path};
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi as DeriveOpenApi};
use utoipa_axum::{router::OpenApiRouter, routes};
use writings::{EmbedAllTrait as _, HiddenWord, HiddenWordKind};

use crate::{ApiResult, api_result::ApiError, api_tag};

#[derive(DeriveOpenApi)]
#[openapi(components(schemas(HiddenWordKind, HiddenWord)))]
pub struct HiddenWordsApiDoc;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::with_openapi(HiddenWordsApiDoc::openapi())
        .routes(routes!(hidden_words_all))
        .routes(routes!(hidden_words_by_kind))
        .routes(routes!(hidden_word))
}

#[utoipa::path(
    get,
    path = "/",
    tag = api_tag(),
    responses(
        (status = OK, body = Vec<HiddenWord>, description = "Hidden Words"),
    )
)]
pub async fn hidden_words_all() -> ApiResult<Json<Vec<HiddenWord>>> {
    Ok(Json(HiddenWord::all().to_vec()))
}

#[utoipa::path(
    get,
    path = "/{kind}",
    tag = api_tag(),
    params(("kind" = HiddenWordKind, Path)),
    responses(
        (status = OK, body = Vec<HiddenWord>, description = "Hidden Words"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
pub async fn hidden_words_by_kind(
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
    tag = api_tag(),
    params(HiddenWordPath),
    responses(
        (status = OK, body = HiddenWord, description = "Hidden Word"),
        (status = NOT_FOUND, description = "Hidden Word not found"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
pub async fn hidden_word(
    Path(HiddenWordPath { kind, num }): Path<HiddenWordPath>,
) -> ApiResult<Json<HiddenWord>> {
    HiddenWord::all()
        .iter()
        .find(|hw| hw.kind == HiddenWordKind::Arabic && hw.number == Some(3))
        .cloned()
        .unwrap();
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
