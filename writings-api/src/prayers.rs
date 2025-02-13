use axum::{Json, extract::Path, routing::get};
use diacritics::remove_diacritics;
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi as DeriveOpenApi};
use utoipa_axum::{router::OpenApiRouter, routes};
use writings::{EmbedAllTrait as _, PrayerKind, PrayerParagraph};

use crate::ApiResult;

#[derive(DeriveOpenApi)]
// Register get_prayers_of_kind_and_section in OpenAPI properly for Swagger UI
#[openapi(paths(get_prayers_of_kind_and_section))]
#[openapi(components(schemas(PrayerKind, PrayerParagraph)))]
pub struct PrayersApiDoc;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::with_openapi(PrayersApiDoc::openapi())
        .routes(routes!(get_all_prayers))
        .routes(routes!(get_prayers_of_kind))
        // Register get_prayers_of_kind_and_section manually using axum's wildcard path syntax,
        // which is not OpenAPI spec and does not work with Swagger UI.
        .route("/{kind}/{*section}", get(get_prayers_of_kind_and_section))
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = OK, body = Vec<PrayerParagraph>, description = "Prayer Paragraphs"),
    )
)]
pub async fn get_all_prayers() -> ApiResult<Json<Vec<PrayerParagraph>>> {
    Ok(Json(PrayerParagraph::all().to_vec()))
}

#[utoipa::path(
    get,
    path = "/{kind}",
    params(("kind" = PrayerKind, Path)),
    responses(
        (status = OK, body = Vec<PrayerParagraph>, description = "Prayer Paragraphs"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
pub async fn get_prayers_of_kind(
    Path(kind): Path<PrayerKind>,
) -> ApiResult<Json<Vec<PrayerParagraph>>> {
    Ok(Json(
        PrayerParagraph::all()
            .iter()
            .filter(|p| p.kind == kind)
            .cloned()
            .collect(),
    ))
}

#[derive(Deserialize, IntoParams)]
pub struct PrayersKindSectionPath {
    #[param(example = "general")]
    kind: PrayerKind,
    #[param(format = "path", example = "teaching/western")]
    section: String,
}

#[utoipa::path(
    get,
    path = "/{kind}/{section}",
    params(PrayersKindSectionPath),
    responses(
        (status = OK, body = Vec<PrayerParagraph>, description = "Prayer Paragraphs"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
pub async fn get_prayers_of_kind_and_section(
    Path(PrayersKindSectionPath { kind, section }): Path<PrayersKindSectionPath>,
) -> ApiResult<Json<Vec<PrayerParagraph>>> {
    let path_sections: Vec<String> = section
        .split('/')
        .map(|s| remove_diacritics(&s.replace('-', " ")).to_lowercase())
        .collect();

    Ok(Json(
        PrayerParagraph::all()
            .iter()
            .filter(|p| p.kind == kind)
            .filter(|p| {
                // Convert prayer sections to normalized form (lowercase, no diacritics)
                let normalized_prayer_sections: Vec<String> = p
                    .section
                    .iter()
                    .map(|s| remove_diacritics(s).to_lowercase())
                    .collect();

                // Check if each path section matches some prayer section
                path_sections.iter().all(|path_section| {
                    normalized_prayer_sections
                        .iter()
                        .any(|prayer_section| prayer_section.contains(path_section))
                })
            })
            .cloned()
            .collect(),
    ))
}
