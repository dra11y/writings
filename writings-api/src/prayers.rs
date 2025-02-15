use axum::{Json, extract::Path, routing::get};
use diacritics::remove_diacritics;
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi as DeriveOpenApi};
use utoipa_axum::{router::OpenApiRouter, routes};
use writings::{EmbedAllTrait as _, PrayerKind, PrayerParagraph};

use crate::{ApiResult, api_tag};

#[derive(DeriveOpenApi)]
// Register prayers_by_kind_section in OpenAPI properly for Swagger UI
#[openapi(paths(prayers_by_kind_section))]
#[openapi(components(schemas(PrayerKind, PrayerParagraph)))]
pub struct PrayersApiDoc;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::with_openapi(PrayersApiDoc::openapi())
        .routes(routes!(prayers_all))
        .routes(routes!(prayers_by_kind))
        // Register prayers_by_kind_section manually using axum's wildcard path syntax,
        // which is not OpenAPI spec and does not work with Swagger UI.
        .route("/{kind}/{*section}", get(prayers_by_kind_section))
}

#[utoipa::path(
    get,
    path = "/",
    tag = api_tag(),
    responses(
        (status = OK, body = Vec<PrayerParagraph>, description = "Prayer Paragraphs"),
    )
)]
pub async fn prayers_all() -> ApiResult<Json<Vec<PrayerParagraph>>> {
    Ok(Json(PrayerParagraph::all().to_vec()))
}

#[utoipa::path(
    get,
    path = "/{kind}",
    tag = api_tag(),
    params(("kind" = PrayerKind, Path)),
    responses(
        (status = OK, body = Vec<PrayerParagraph>, description = "Prayer Paragraphs"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
pub async fn prayers_by_kind(
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
    // This is proper path annotation for Swagger UI, but does not
    // register as a wildcard in axum, hence we register it twice.
    path = "/{kind}/{section}",
    tag = api_tag(),
    params(PrayersKindSectionPath),
    responses(
        (status = OK, body = Vec<PrayerParagraph>, description = "Prayer Paragraphs"),
        (status = BAD_REQUEST, description = "bad request / invalid parameters")
    )
)]
pub async fn prayers_by_kind_section(
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
